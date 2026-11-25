//! Звёздное поле: генерация, вращение и движение звёзд.
//!
//! Координаты звёзд хранятся в системе корабля (`+Z` — вперёд), поэтому поле
//! покрывает всю сферу вокруг кабины: в переднем виде звёзды расходятся из
//! центра, в заднем — сходятся к нему, а в боковых — пересекают экран
//! горизонтально. Так получается из одного и того же движения: при полёте
//! вперёд координата `Z` всех звёзд уменьшается.
//!
//! Скорость и темп задаёт Z80. Растяжение координат из `sub_78B1` оригинала
//! переведено в эквивалентное поступательное движение поля (см.
//! [`StarField::advance`]).

use crate::render3d::Rotation;

use super::rng::Rng;

/// Максимальная скорость в единицах `HighDeltaSpeed` (0..=31).
///
/// В оригинале `DeltaSpeed` — 16-битное слово с шагом `0xA0` на кадр, а его
/// старший байт `HighDeltaSpeed` меняется на 1 за 1/0.625 кадра. Полная шкала
/// заканчивается на 31: выше игра скорость не поднимает.
pub(super) const SPEED_MAX: f32 = 31.0;
/// Минимальное число звёзд в поле.
///
/// В оригинале таблица `byte_7A39` содержит ровно 16 звёзд на кадр 256×192 и
/// все они лежат в узком конусе перед кораблём. Нативное поле равномерно
/// покрывает сферу, поэтому в каждый вид попадает лишь часть звёзд — минимум
/// подобран так, чтобы на исходном кадре было видно те же ~16 точек.
pub(super) const STAR_COUNT: usize = 400;
/// Площадь вида, на которую приходится одна звезда поля.
///
/// При фокусе `2 * высота` в один вид попадает около 3.5% сферы вместо ~9%
/// при широком обзоре. Плотность поднята примерно в 2.6 раза, чтобы на экране
/// снова было около 16 звёзд, как в плоском поле оригинала.
const STAR_AREA_PER_STAR: f32 = 230.0;
/// Пределы числа звёзд, чтобы поле не пустело и не забивало экран.
pub(super) const STAR_COUNT_MIN: usize = 400;
pub(super) const STAR_COUNT_MAX: usize = 10_000;
/// Радиус сферического звёздного облака вокруг корабля.
pub(super) const STAR_FIELD_RADIUS: f32 = 6_000.0;
/// Минимальное расстояние рождения звезды.
///
/// Совсем близкие звёзды проносятся через камеру за один кадр и выглядят как
/// случайные вспышки, поэтому центр облака оставлен пустым.
const STAR_FIELD_MIN_RADIUS: f32 = 400.0;
/// Скорость поля в единицах координат на единицу `b` в секунду.
///
/// В оригинале за игровой такт (`STAR_TICK_HZ` ≈ 5.5 Гц) координаты звезды
/// растягивались в `1 + b/256` раз. Для звезды на глубине ~2200 единиц это
/// эквивалентно смещению `z` примерно на `b * 2200 / 256` за такт.
const STAR_UNITS_PER_B: f32 = 2200.0 * 5.5 / 256.0;
/// Добавка скорости за единицу `HighDeltaSpeed` сверх базовой `b`.
///
/// Одного `b = (HighDeltaSpeed >> 1) + 1` мало: на максимальной скорости
/// звёзды в дальних слоях облака двигались слишком медленно. Множитель растёт
/// линейно и почти не меняет крейсерский темп, но заметно ускоряет полёт на
/// пределе.
const STAR_SPEED_RAMP: f32 = 1.0 / 16.0;
/// Общий множитель скорости обычного полёта.
///
/// Оригинальная скорость звёзд субъективно слишком медленная и на минимуме,
/// и на максимуме, поэтому обычное движение ускорено вдвое. J-драйв этот
/// множитель не затрагивает — у него своя скорость.
const STAR_SPEED_MULTIPLIER: f32 = 2.0;
/// Во сколько раз J-драйв разгоняет звёздное поле.
///
/// В оригинале `PrintStars` на одном такте делает три прохода `sub_78B1`.
/// Здесь разгон растянут на короткий промежуток, чтобы ускорение было видно
/// глазом, а не мелькало один кадр.
const J_DRIVE_SPEED_MULTIPLIER: f32 = 4.0;
/// Сколько секунд длится разгон J-драйва.
const J_DRIVE_TIME: f32 = 0.45;
/// Выдержка, с которой рисуется штрих звезды на J-драйве.
const STAR_STREAK_TIME: f32 = 0.15;
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
    /// Положение в координатах корабля (`+Z` — вперёд).
    pub(super) pos: [f32; 3],
    pub(super) mag: u8,
}

/// Звёздное поле и его генератор.
pub(super) struct StarField {
    pub(super) stars: Vec<Star>,
    rng: Rng,
    /// Разогнать звёзды на ближайшем шаге симуляции (J-драйв).
    pub(super) boost: bool,
    /// Остаток разгона J-драйва в секундах.
    j_drive: f32,
    /// Длина штриха звезды в координатах мира (0 — рисовать точкой).
    streak: f32,
}

impl StarField {
    pub(super) fn new(count: usize) -> Self {
        let mut rng = Rng::new(0x1234_5678);
        let stars = make_stars(&mut rng, count);
        StarField {
            stars,
            rng,
            boost: false,
            j_drive: 0.0,
            streak: 0.0,
        }
    }

    /// Пересоздать поле (при открытии экрана полёта).
    pub(super) fn reset(&mut self, count: usize) {
        self.stars = make_stars(&mut self.rng, count);
        self.boost = false;
        self.j_drive = 0.0;
        self.streak = 0.0;
    }

    /// Подогнать число звёзд под площадь вида.
    pub(super) fn resize(&mut self, count: usize) {
        if self.stars.len() != count {
            self.stars = make_stars(&mut self.rng, count);
        }
    }

    pub(super) fn rng_mut(&mut self) -> &mut Rng {
        &mut self.rng
    }

    /// Длина штриха звезды при рендере (0 вне J-драйва).
    pub(super) fn streak(&self) -> f32 {
        self.streak
    }

    /// Повернуть звёздное поле на шаг камеры.
    ///
    /// Звёзды хранятся прямо в координатах корабля. Поворот камеры на `turn`
    /// переводит старые координаты в новые той же матрицей, поэтому никакой
    /// отдельной ориентации поля не нужно.
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
    /// Корабль летит вперёд по `+Z`, поэтому в его системе координат все
    /// звёзды смещаются в `-Z`. Это одно движение сразу даёт нужный эффект во
    /// всех видах:
    ///
    /// - в переднем (`+Z`) звёзды приближаются и расходятся от центра;
    /// - в заднем (`-Z`) удаляются и сходятся к центру;
    /// - в боковых (`±X`) проходят поперёк экрана.
    ///
    /// Базовая скорость взята из `sub_78B1`: `b = (HighDeltaSpeed >> 1) + 1`,
    /// но с ростом скорости дополнительно увеличивается (см.
    /// [`STAR_SPEED_RAMP`]), иначе на максимуме дальние звёзды почти стоят.
    /// Обычный полёт дополнительно ускорен в [`STAR_SPEED_MULTIPLIER`] раз.
    /// На J-драйве поле разгоняется на короткое время, а рендер рисует звёзды
    /// штрихами — так ускорение видно глазом; множитель обычного полёта к
    /// J-драйву не применяется.
    pub(super) fn advance(&mut self, dt: f32, speed: f32) {
        if !(dt > 0.0) {
            return;
        }
        if self.boost {
            self.boost = false;
            self.j_drive = J_DRIVE_TIME;
        }
        let boosted = self.j_drive > 0.0;
        let b = if boosted {
            speed_b(speed.clamp(0.0, SPEED_MAX)) * J_DRIVE_SPEED_MULTIPLIER
        } else {
            speed_b(speed.clamp(0.0, SPEED_MAX)) * STAR_SPEED_MULTIPLIER
        };
        self.j_drive = (self.j_drive - dt).max(0.0);
        self.streak = if boosted {
            b * STAR_UNITS_PER_B * STAR_STREAK_TIME
        } else {
            0.0
        };

        let step = b * STAR_UNITS_PER_B * dt;
        self.move_stars(step);
    }

    /// Движение звёзд во время тоннеля гиперпрыжка.
    ///
    /// `initial_speed` — скорость корабля в начале прыжка, `progress` — доля
    /// анимации тоннеля в диапазоне [0, 1]. К середине скорость плавно растёт
    /// до удвоенной максимальной, а звёзды вытягиваются в штрихи как при
    /// J-драйве; к концу скорость и растяжение возвращаются к исходным.
    pub(super) fn advance_hyperspace(&mut self, dt: f32, initial_speed: f32, progress: f32) {
        if !(dt > 0.0) {
            return;
        }
        let p = progress.clamp(0.0, 1.0);
        let envelope = 1.0 - (2.0 * p - 1.0).abs();
        let peak = SPEED_MAX * 2.0;
        let speed = initial_speed + (peak - initial_speed) * envelope;
        let b = speed_b(speed.max(0.0)) * STAR_SPEED_MULTIPLIER;
        self.j_drive = 0.0;
        self.boost = false;
        self.streak = b * STAR_UNITS_PER_B * STAR_STREAK_TIME * envelope;

        let step = b * STAR_UNITS_PER_B * dt;
        self.move_stars(step);
    }

    /// Сдвинуть все звёзды на `step` назад и пересоздать ушедшие за облако.
    fn move_stars(&mut self, step: f32) {
        let StarField { stars, rng, .. } = self;
        for star in stars.iter_mut() {
            if !star.pos.iter().all(|value| value.is_finite()) {
                *star = spawn_star(rng);
                continue;
            }
            star.pos[2] -= step;
            // Звезда прошла позади корабля и ушла за облако — рождаем новую
            // в случайной точке сферы. При большом числе звёзд такие
            // «подмены» незаметны.
            if star.pos[2] < -STAR_FIELD_RADIUS {
                *star = spawn_star(rng);
            }
        }
    }
}

/// Базовый коэффициент скорости звёзд для значения `speed`.
fn speed_b(speed: f32) -> f32 {
    ((speed * 0.5).floor() + 1.0).max(1.0) * (1.0 + speed * STAR_SPEED_RAMP)
}

/// Сколько звёзд держать в сферическом поле для вида такого размера.
///
/// Число растёт с площадью экрана: на 256×128 получается исходная плотность
/// поля, на 1280×750 — около 4200 звёзд, из которых в кадр попадает примерно
/// столько же, сколько раньше попадало из плоского поля.
pub(super) fn star_count_for(width: usize, height: usize) -> usize {
    let count = (width * height) as f32 / STAR_AREA_PER_STAR;
    (count.round() as usize).clamp(STAR_COUNT_MIN, STAR_COUNT_MAX)
}

/// Детерминированное звёздное поле, равномерно заполняющее сферу.
fn make_stars(rng: &mut Rng, count: usize) -> Vec<Star> {
    (0..count).map(|_| spawn_star(rng)).collect()
}

/// Родить звезду в случайной точке сферического облака.
///
/// Направление равномерно по сфере: тогда во всех видах плотность одинакова,
/// и звёзды не сбиваются в центр экрана. Радиус распределён по объёму, чтобы
/// ближние и дальние звёзды встречались одинаково часто.
fn spawn_star(rng: &mut Rng) -> Star {
    let z = rng.next_f32() * 2.0 - 1.0;
    let around = rng.next_f32() * std::f32::consts::TAU;
    let min = STAR_FIELD_MIN_RADIUS / STAR_FIELD_RADIUS;
    let radius = STAR_FIELD_RADIUS * (min + (1.0 - min) * rng.next_f32().cbrt());
    let ring = (1.0 - z * z).max(0.0).sqrt();
    let (sa, ca) = around.sin_cos();
    Star {
        pos: [ca * ring * radius, sa * ring * radius, z * radius],
        mag: (rng.next_u32() >> 24) as u8,
    }
}
