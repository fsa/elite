//! Звёздное поле: генерация, вращение и движение звёзд.
//!
//! Координаты звёзд хранятся прямо в системе вида (`+Z` — вперёд), поэтому
//! поворот камеры применяется к самим звёздам. Скорость и темп задаёт Z80:
//! растяжение координат от центра повторяет `sub_78B1` оригинала.

use crate::render3d::Rotation;

use super::rng::Rng;
use super::view::{self, ViewDir};

/// Максимальная скорость в единицах `HighDeltaSpeed` (0..=31).
///
/// В оригинале `DeltaSpeed` — 16-битное слово с шагом `0xA0` на кадр, а его
/// старший байт `HighDeltaSpeed` меняется на 1 за 1/0.625 кадра. Полная шкала
/// заканчивается на 31: выше игра скорость не поднимает.
pub(super) const SPEED_MAX: f32 = 31.0;
/// Минимальное число звёзд на экране.
///
/// В оригинале таблица `byte_7A39` содержит ровно 16 звёзд на кадр
/// 256×192. На современном разрешении такое поле выглядит почти пустым,
/// поэтому фактическое число звёзд считается от площади вида
/// (см. [`star_count_for`]), а эта константа задаёт минимум.
pub(super) const STAR_COUNT: usize = 16;
/// Площадь вида, на которую приходится одна звезда.
///
/// В оригинале одна звезда на ~3072 пикселя. Здесь плотность вдвое ниже:
/// современная точка — это 1 px, и слишком густое поле превращается в шум.
const STAR_AREA_PER_STAR: f32 = 6_000.0;
/// Пределы числа звёзд, чтобы поле не пустело и не забивало экран.
pub(super) const STAR_COUNT_MIN: usize = 16;
pub(super) const STAR_COUNT_MAX: usize = 400;
/// Делитель шага звёздного поля.
///
/// В оригинале координаты звёзд растягиваются от центра в
/// `1 + b/256` раз за игровой такт, где `b = (HighDeltaSpeed >> 1) + 1`.
const STAR_ZOOM_DIVISOR: f32 = 256.0;
/// Частота игрового такта, на котором оригинал растягивает звёзды.
///
/// `CalcNewCrdObj_Pl`/`PrintStars` работают не на кадре 50 Гц, а примерно
/// на 5.5 Гц. Нативный рендер вызывает [`StarField::advance`] каждый кадр,
/// поэтому шаг растяжения пересчитывается на прошедшую долю такта.
const STAR_TICK_HZ: f32 = 5.5;
/// Радиус (`x/z` или `y/z`), после которого звезда ушла за кадр.
///
/// Оригинал не «приближает» звезду в бесконечность: когда её координата
/// выходит за край, она рождается заново ближе к центру. Тот же порог не даёт
/// звезде разгоняться в огромную полосу на последних кадрах.
const STAR_RESPAWN_RADIUS: f32 = 1.8;
/// Абсолютный предел расстояния до звезды.
///
/// После поворотов камеры большая боковая координата может стать глубиной,
/// поэтому одного углового порога мало: без предела звезда уползала бы всё
/// дальше от начала координат.
const STAR_MAX_DISTANCE: f32 = 15_000.0;
/// Шаг растяжения звёзд на такте J-драйва.
///
/// В `PrintStars` J-драйв вызывает `sub_78B1` три раза: со скоростями
/// `0x1E`, `0x10` и `0x10`, то есть с `b = 16`, `9` и `9`. Сумма `34`
/// даёт тот же разгон за такт, как три прохода оригинала.
const STAR_FAST_SCROLL_B: f32 = 16.0 + 9.0 + 9.0;
/// Поворот звёздного поля за единицу `HighAngleLR` (крен).
///
/// В `CalcNewCrdObj_Pl` крен применяется как `angleLR * 2 / 256`, а тангаж —
/// как `angleDU / 256`. Это малые углы за игровой такт, а не за кадр 50 Гц:
/// оригинал пересчитывает координаты примерно на 5.5 Гц, поэтому поворот
/// звёзд применяется только на такте (см. `Flight::sync_from_z80`).
pub(super) const STAR_ANGLE_LR_PER_UNIT: f32 = 2.0 / 256.0;
/// Поворот звёздного поля за единицу `HighAngleDU` (тангаж).
pub(super) const STAR_ANGLE_DU_PER_UNIT: f32 = 1.0 / 256.0;

/// Звезда в космическом пространстве.
pub(super) struct Star {
    /// Положение в координатах вида (`+Z` — вперёд).
    pub(super) pos: [f32; 3],
    pub(super) mag: u8,
}

/// Звёздное поле и его генератор.
pub(super) struct StarField {
    pub(super) stars: Vec<Star>,
    rng: Rng,
    /// Разогнать звёзды на ближайшем шаге симуляции (J-драйв).
    pub(super) boost: bool,
}

impl StarField {
    pub(super) fn new(count: usize) -> Self {
        let mut rng = Rng::new(0x1234_5678);
        let stars = make_stars(&mut rng, count);
        StarField {
            stars,
            rng,
            boost: false,
        }
    }

    /// Пересоздать поле (при открытии экрана полёта).
    pub(super) fn reset(&mut self, count: usize) {
        self.stars = make_stars(&mut self.rng, count);
        self.boost = false;
    }

    /// Пересоздать звёзды, не трогая флаг J-драйва (смена направления вида).
    pub(super) fn respawn(&mut self, count: usize) {
        self.stars = make_stars(&mut self.rng, count);
    }

    /// Подогнать число звёзд под площадь вида.
    pub(super) fn resize(&mut self, count: usize) {
        if self.stars.len() != count {
            self.stars = make_stars(&mut self.rng, count);
        }
    }

    pub(super) fn len(&self) -> usize {
        self.stars.len()
    }

    pub(super) fn rng_mut(&mut self) -> &mut Rng {
        &mut self.rng
    }

    /// Повернуть звёздное поле на шаг камеры.
    ///
    /// Звёзды хранятся прямо в координатах вида (`+Z` — вперёд). Поворот
    /// камеры на `turn` переводит старые координаты вида в новые той же
    /// матрицей, поэтому никакой отдельной ориентации поля не нужно.
    pub(super) fn turn(&mut self, turn: &Rotation) {
        for star in &mut self.stars {
            star.pos = turn.apply(star.pos);
        }
    }

    /// Запомнить, что на ближайшем шаге нужно разогнать звёзды (J-драйв).
    pub(super) fn mark_boost(&mut self, fast: bool) {
        self.boost |= fast;
    }

    /// Сдвинуть звёздное поле назад относительно корабля.
    ///
    /// Оригинал (`sub_78B1`) не сдвигает звёзды на фиксированное расстояние:
    /// каждый кадр он растягивает их координаты от центра в
    /// `1 + b/256` раз, где `b = (HighDeltaSpeed >> 1) + 1`. В перспективной
    /// проекции это ровно то же, что «звезда летит навстречу»: `x/z` растёт.
    /// Поэтому здесь масштабируем `x` и `y`, а не уменьшаем `z` — так скорость
    /// звезды не зависит от её начальной глубины и не взрывается у камеры.
    /// Ушедшие за край звезды рождаются заново ближе к центру.
    pub(super) fn advance(&mut self, dt: f32, speed: f32, view_dir: ViewDir) {
        let frames = (dt * 50.0).max(0.0);
        if frames <= 0.0 {
            return;
        }
        // J-драйв: на ближайшем шаге звёзды должны улететь так же далеко,
        // как после трёх проходов `sub_78B1` на одном такте оригинала.
        let boosted = self.boost;
        let b = if boosted {
            self.boost = false;
            STAR_FAST_SCROLL_B
        } else {
            ((speed * 0.5).floor() as f32) + 1.0
        };
        let b = b.max(1.0);
        // Оригинал растягивает поле раз в игровой такт (~5.5 Гц), а не каждый
        // кадр 50 Гц. Обычный шаг пересчитываем на прошедшую долю такта,
        // чтобы движение оставалось плавным; J-драйв — это один такт из трёх
        // проходов `sub_78B1`, поэтому его применяем целиком.
        let grow = if boosted {
            1.0 + b / STAR_ZOOM_DIVISOR
        } else {
            (1.0 + b / STAR_ZOOM_DIVISOR).powf(frames * STAR_TICK_HZ / 50.0)
        };
        let cam = view::rotation(view_dir).transpose();
        let StarField { stars, rng, .. } = self;
        for star in stars.iter_mut() {
            let v = cam.apply(star.pos);
            if v[2] <= view::NEAR {
                *star = spawn_star(rng);
                continue;
            }
            match view_dir {
                ViewDir::Forward => {
                    star.pos[0] *= grow;
                    star.pos[1] *= grow;
                }
                ViewDir::Back => {
                    star.pos[0] *= 1.0 / grow.max(1.0001);
                    star.pos[1] *= 1.0 / grow.max(1.0001);
                    star.pos[2] = -star.pos[2].abs().max(1.0);
                }
                ViewDir::Left => {
                    star.pos[1] *= grow;
                    star.pos[2] *= 1.0 / grow.max(1.0001);
                    star.pos[0] = -star.pos[0].abs().max(1.0);
                }
                ViewDir::Right => {
                    star.pos[1] *= grow;
                    star.pos[2] *= 1.0 / grow.max(1.0001);
                    star.pos[0] = star.pos[0].abs().max(1.0);
                }
            }
            let v2 = cam.apply(star.pos);
            let z = v2[2];
            let radius = if z > view::NEAR {
                (v2[0] * v2[0] + v2[1] * v2[1]).sqrt() / z
            } else {
                f32::INFINITY
            };
            let distance2 = v2.iter().map(|val| val * val).sum::<f32>();
            if radius <= STAR_RESPAWN_RADIUS
                && distance2 <= STAR_MAX_DISTANCE * STAR_MAX_DISTANCE
                && v2.iter().all(|val| val.is_finite())
            {
                continue;
            }
            *star = spawn_star(rng);
        }
    }
}

/// Сколько звёзд рисовать для вида такого размера.
///
/// Число растёт с площадью экрана: на 256×128 это исходные 16 звёзд, на
/// 1280×750 — около 160. Так плотность поля примерно сохраняется, но не
/// доходит до «шума» из сотен точек на маленьком окне.
pub(super) fn star_count_for(width: usize, height: usize) -> usize {
    let count = (width * height) as f32 / STAR_AREA_PER_STAR;
    (count.round() as usize).clamp(STAR_COUNT_MIN, STAR_COUNT_MAX)
}

/// Детерминированное звёздное поле: звёзды перед кораблём.
fn make_stars(rng: &mut Rng, count: usize) -> Vec<Star> {
    (0..count).map(|_| spawn_star(rng)).collect()
}

/// Родить звезду ближе к центру кадра.
///
/// Угловой радиус намеренно небольшой: звёзды должны расходиться из центра
/// пучком, как в оригинале, а не появляться уже по всему экрану. Тогда по их
/// движению видно направление полёта.
fn spawn_star(rng: &mut Rng) -> Star {
    let around = rng.next_f32() * std::f32::consts::TAU;
    let radius = 0.03 + rng.next_f32() * 0.25;
    let z = 1200.0 + rng.next_f32() * 4800.0;
    let (sa, ca) = around.sin_cos();
    Star {
        pos: [ca * radius * z, sa * radius * z, z],
        mag: (rng.next_u32() >> 24) as u8,
    }
}
