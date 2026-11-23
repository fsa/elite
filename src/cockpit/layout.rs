//! Раскладка переменных панели в памяти Z80 и чтение показаний.
//!
//! Здесь собраны адреса переменных игры ([`KLADOV_COCKPIT`]) и
//! преобразование байтов живой памяти в снимок [`CockpitValues`]. Отрисовка
//! о памяти ничего не знает.

use super::values::{
    COMPASS_CIRCLE_MAX_OFFSET, CockpitValues, Compass, CompassCircle, CompassGlyph, DEFLECTION_MAX,
    Danger, GAUGE_MAX, Indicator, MISSILE_SLOTS, RadarContact, RadarContactKind, RadarContacts,
};

/// Размер слота `SlotObj` в памяти Z80.
const SLOT_SIZE: u16 = 0x27;
/// Смещение `oEnerg` в слоте: 0 — слот пуст.
const SLOT_OENERG: u16 = 0x22;
/// Число основных слотов объектов (без станции и ракеты).
const SHIP_SLOTS: u16 = 6;
/// Самомодифицируемый байт `sub_FD02+1` (`0` — солнце, `9` — планета).
/// `sub_6F29` читает его, чтобы выбрать растр «стороны света» в космосе.
const COMPASS_TARGET: u16 = 0xFD03;

/// Адреса переменных панели в памяти игры (текущее значение показания).
///
/// Каждый индикатор-полоса — байт `0..0xA0` (160 т.е. 100%), полоса занимает
/// 5x5 «пикселей» ZX от `AdrScrIndicators` (значение сдвигается вправо на 2).
/// Адреса различаются у версий на постоянный сдвиг (Кладов +0xA2).
#[allow(dead_code)] // понадобится на следующих шагах заполнения панели
#[derive(Clone, Copy)]
pub struct CockpitLayout {
    /// Передний щит (`ShieldFront`).
    pub shield_front: u16,
    /// Задний щит (`ShieldRear`).
    pub shield_rear: u16,
    /// Топливо (`Fuel`).
    pub fuel: u16,
    /// Температура кабины (`Temperature`).
    pub temperature: u16,
    /// Температура лазера (`TempLaser`).
    pub laser_temp: u16,
    /// Высота/дальность до поверхности (`Altitude`).
    pub altitude: u16,
    /// Скорость (`Speed`).
    pub speed: u16,
    /// Четыре энергоблока (`ShieldUnit1..4`), идут подряд с шагом 2.
    pub shield_unit1: u16,
    /// Отклонение влево/вправо (`RotateLR`), нейтраль `0x15`.
    pub rotate_lr: u16,
    /// Отклонение вверх/вниз (`RotateDU`), нейтраль `0x15`.
    pub rotate_du: u16,
    /// Запас ракет (`PilotMissiles`), 0..4.
    pub missiles: u16,
    /// Указатель компаса (`CompasPointerCrdX`), знак-величина.
    pub compass_x: u16,
    /// Указатель компаса (`CompasPointerCrdY`), знак-величина.
    pub compass_y: u16,
    /// Тип указателя (`CompasPointerType`): `0` — цель впереди.
    pub compass_type: u16,
    /// Признак межпланетного пространства (`InFreeSpace`): обычный указатель
    /// комбинирует «стороны света» для солнца и планеты.
    pub in_free_space: u16,
    /// Координаты солнца (`SunCrd`), три байта на ось.
    pub sun_crd: u16,
    /// Координаты планеты (`PlanetCrd`), три байта на ось.
    pub planet_crd: u16,
    /// Слот станции (`objCoriolis`).
    pub coriolis_slot: u16,
    /// Первый из шести основных слотов объектов (`DataOfAround`).
    pub ship_slot0: u16,
    /// Слот ракеты (`objMissile`).
    pub missile_slot: u16,
    /// Признак близости станции (`CoriolisNear`), включает символ `S`.
    pub coriolis_near: u16,
    /// Оставшееся время работы ECM (`TimeWorkECM`), `0` — выключена.
    pub ecm_time: u16,
}

/// Раскладка переменных панели для версии Кладова (`kladov128.bin`), +0xA2.
#[allow(dead_code)] // понадобится на следующих шагах заполнения панели
pub const KLADOV_COCKPIT: CockpitLayout = CockpitLayout {
    shield_front: 0x70E8,
    shield_rear: 0x70EA,
    fuel: 0x70EC,
    temperature: 0x70EE,
    laser_temp: 0x70F0,
    altitude: 0x70F2,
    speed: 0x70F4,
    shield_unit1: 0x70F6,
    rotate_lr: 0x70FE,
    rotate_du: 0x7100,
    missiles: 0xD083,
    compass_x: 0x81A6,
    compass_y: 0x81A7,
    compass_type: 0x81A8,
    in_free_space: 0x7103,
    sun_crd: 0x7107,
    planet_crd: 0x7110,
    coriolis_slot: 0x6048,
    ship_slot0: 0x606F,
    missile_slot: 0x6159,
    coriolis_near: 0x711C,
    ecm_time: 0x9C9B,
};

impl CockpitLayout {
    /// Прочитать показания панели из живой памяти Z80.
    ///
    /// Значения шкал ограничиваются [`GAUGE_MAX`], отклонения приводятся
    /// к нейтрали `0x15`, запас ракет — к [`MISSILE_SLOTS`].
    pub fn read(&self, mem: &[u8]) -> CockpitValues {
        let byte = |addr: u16| mem.get(addr as usize).copied().unwrap_or(0);
        let gauge = |addr: u16| Indicator::Gauge(byte(addr).min(GAUGE_MAX));
        let deflection = |addr: u16| {
            let offset = byte(addr) as i16 - 0x15;
            Indicator::Deflection(
                offset.clamp(-(DEFLECTION_MAX as i16), DEFLECTION_MAX as i16) as i8,
            )
        };
        let in_free_space = byte(self.in_free_space) != 0;
        let flight_view = mem
            .get(crate::flight::Z80_WHAT_ON_SCREEN)
            .copied()
            .unwrap_or(0)
            < 4;
        // Тела видны только внутри планетной системы: в открытом космосе
        // обычный компас работает с отдельными знаками «стороны света».
        let circle = |base: u16| {
            if flight_view && !in_free_space {
                self.read_compass_circle(mem, base)
            } else {
                CompassCircle::hidden()
            }
        };
        let station = (self.coriolis_slot as usize) + SLOT_OENERG as usize;
        let station = if flight_view && mem.get(station).copied().unwrap_or(0) != 0 {
            circle(self.coriolis_slot)
        } else {
            CompassCircle::hidden()
        };
        CockpitValues {
            left: [
                gauge(self.shield_front),
                gauge(self.shield_rear),
                gauge(self.fuel),
                gauge(self.temperature),
                gauge(self.laser_temp),
                gauge(self.altitude),
                Indicator::Missiles(byte(self.missiles).min(MISSILE_SLOTS as u8)),
            ],
            right: [
                gauge(self.speed),
                deflection(self.rotate_lr),
                deflection(self.rotate_du),
                gauge(self.shield_unit1),
                gauge(self.shield_unit1 + 2),
                gauge(self.shield_unit1 + 4),
                gauge(self.shield_unit1 + 6),
            ],
            danger: read_danger(mem),
            compass: self.read_compass(mem),
            compass_planet: circle(self.planet_crd),
            compass_station: station,
            compass_sun: circle(self.sun_crd),
            in_free_space,
            compass_blink_phase: true,
            radar: self.read_radar(mem),
            station_near: byte(self.coriolis_near) != 0,
            ecm_on: byte(self.ecm_time) != 0,
            blink: true,
        }
    }

    /// Прочитать указатель обычного режима компаса.
    ///
    /// Оригинал выбирает цель в `PrintCompass` и записывает готовое смещение в
    /// `CompasPointerCrdX/Y` и `CompasPointerType`, откуда его берёт `RenewCompass`.
    /// Нативный компас читает те же переменные: пересчёт по координатам планеты,
    /// станции и межпланетного «фантома» легко расходится с оригиналом и показывает
    /// старую цель даже после переключения на станцию.
    fn read_compass(&self, mem: &[u8]) -> Compass {
        let byte = |addr: u16| mem.get(addr as usize).copied().unwrap_or(0);
        if mem
            .get(crate::flight::Z80_WHAT_ON_SCREEN)
            .copied()
            .unwrap_or(0)
            >= 4
        {
            return Compass::hidden();
        }
        // `CompasPointerCrdX/Y` хранятся в знак-величине, как и координаты игрока.
        let signed = |v: u8| {
            if v & 0x80 != 0 {
                -((v & 0x7F) as i8)
            } else {
                v as i8
            }
        };
        let behind = byte(self.compass_type) != 0;
        let glyph = if byte(self.in_free_space) != 0 {
            // В космосе `sub_6F29` берёт «стороны света», а не ромб/квадрат.
            // Нечётное `sub_FD02+1` означает цель-планету: в этом кадре
            // `sub_FD02` смещает IX на 9 байтов от фиктивного солнца.
            if byte(COMPASS_TARGET) & 1 != 0 {
                if behind {
                    CompassGlyph::PlanetBack
                } else {
                    CompassGlyph::PlanetFront
                }
            } else if behind {
                CompassGlyph::SunBack
            } else {
                CompassGlyph::SunFront
            }
        } else if behind {
            CompassGlyph::Empty
        } else {
            CompassGlyph::Filled
        };
        Compass {
            x: signed(byte(self.compass_x)),
            y: signed(byte(self.compass_y)),
            glyph,
            visible: true,
        }
    }

    /// Вычислить положение круга тела на альтернативном компасе.
    ///
    /// Это нативный расчёт: не читается готовый указатель `PrintCompass`, а
    /// направление считается по координатам самого тела. Координаты уже ведутся
    /// относительно игрока, поэтому смещение — нормированная на длину вектора
    /// проекция, умноженная на радиус 8 «пикселей» оригинала. Берём все 24 бита:
    /// знак лежит в старшем бите третьего байта, остальные его биты — часть
    /// величины. По знаку `Z` решается, тело впереди или позади.
    fn read_compass_circle(&self, mem: &[u8], base: u16) -> CompassCircle {
        let at = base as usize;
        let read = |offset: usize| {
            let from = at + offset;
            if from + 3 > mem.len() {
                0
            } else {
                read_compass_coordinate(&mem[from..from + 3])
            }
        };
        let x = read(0);
        let y = read(3);
        let z = read(6);
        let len = ((x as f32).powi(2) + (y as f32).powi(2) + (z as f32).powi(2)).sqrt();
        let scale = if len > 0.0 { 8.0 / len } else { 0.0 };
        let offset = |v: i32| {
            (v as f32 * scale)
                .trunc()
                .clamp(
                    -(COMPASS_CIRCLE_MAX_OFFSET as f32),
                    COMPASS_CIRCLE_MAX_OFFSET as f32,
                ) as i8
        };
        CompassCircle {
            x: offset(x),
            y: offset(y),
            behind: z < 0,
            visible: true,
        }
    }

    /// Прочитать контакты для радара.
    ///
    /// Обходится тот же набор слотов, что и в `PrintObjScaner`: станция,
    /// шесть основных слотов и ракета. Пустые слоты (`oEnerg == 0`)
    /// пропускаются. Координаты уже ведутся в системе игрока, поэтому
    /// радарам не нужно ничего пересчитывать.
    fn read_radar(&self, mem: &[u8]) -> RadarContacts {
        let mut contacts = RadarContacts::empty();
        self.push_slot(mem, self.coriolis_slot, RadarContactKind::Station, 0, &mut contacts);
        for i in 0..SHIP_SLOTS {
            self.push_slot(
                mem,
                self.ship_slot0.wrapping_add(i.wrapping_mul(SLOT_SIZE)),
                RadarContactKind::Ship,
                (i + 1) as u8,
                &mut contacts,
            );
        }
        self.push_slot(mem, self.missile_slot, RadarContactKind::Missile, 7, &mut contacts);
        contacts
    }

    /// Добавить объект из слота Z80, если он существует и целиком в памяти.
    fn push_slot(
        &self,
        mem: &[u8],
        base: u16,
        kind: RadarContactKind,
        slot: u8,
        contacts: &mut RadarContacts,
    ) {
        if base == 0 {
            return;
        }
        let base = base as usize;
        let size = SLOT_SIZE as usize;
        if base + size > mem.len() || mem[base + SLOT_OENERG as usize] == 0 {
            return;
        }
        contacts.push(RadarContact {
            x: read_game_coordinate(&mem[base..base + 3]),
            y: read_game_coordinate(&mem[base + 3..base + 6]),
            z: read_game_coordinate(&mem[base + 6..base + 9]),
            kind,
            slot,
        });
    }
}

/// Атрибут кружка опасности на панели кокпита (`WhatOnScreen < 4`):
/// `PrintIndDanger` пишет цвет в левый верхний угол 2x2 знакоместа.
///
/// Читать нужно именно экранный атрибут, а не переменную `IndDanger`:
/// `PrintIndDanger` в конце каждой печати снова записывает в `IndDanger`
/// значение `0x04` («в полёте»), а также подменяет его на красный при пустых
/// щитах и мигает красным при неполном четвёртом энергоблоке. В памяти игры
/// к концу кадра почти всегда остаётся `0x04`, тогда как на экране в этот
/// момент может быть жёлтый, красный или погасший кружок.
pub(super) const DANGER_ATTR: u16 = 0x5A28;

/// Атрибут того же кружка на вспомогательных экранах (`WhatOnScreen >= 4`).
///
/// Там `PrintIndDanger` рисует кружок в правом верхнем углу (строка 1,
/// колонка 28, символы `0x21`-`0x24`) и пишет его цвет по адресу `0x583C`.
/// Адрес `0x5A28` принадлежит панели кокпита: на картах и экране капитана
/// там лежат атрибуты текста, поэтому читать его на этих экранах нельзя.
pub(super) const DANGER_SCREEN_ATTR: u16 = 0x583C;

/// Прочитать текущий цвет индикатора опасности с экрана.
pub(super) fn read_danger(mem: &[u8]) -> Danger {
    let screen = mem
        .get(crate::flight::Z80_WHAT_ON_SCREEN)
        .copied()
        .unwrap_or(0);
    let at = if screen >= 4 {
        DANGER_SCREEN_ATTR
    } else {
        DANGER_ATTR
    };
    Danger::from_attribute(mem.get(at as usize).copied().unwrap_or(0))
}

/// Прочитать 24-битную координату как целое число единиц игры.
fn read_game_coordinate(b: &[u8]) -> i32 {
    if b.len() < 3 {
        return 0;
    }
    let mag = (((b[2] & 0x7F) as i32) << 16) | ((b[1] as i32) << 8) | b[0] as i32;
    if b[2] & 0x80 != 0 { -mag } else { mag }
}

/// Координата тела для компаса в формате знак-величина Z80.
///
/// Старший бит третьего байта — знак, остальные семь битов — часть величины.
/// Нативный расчёт использует все 24 бита: на больших расстояниях старший байт
/// заметно влияет на направление.
fn read_compass_coordinate(b: &[u8]) -> i32 {
    if b.len() < 3 {
        return 0;
    }
    let mag = (((b[2] & 0x7F) as i32) << 16) | ((b[1] as i32) << 8) | b[0] as i32;
    if b[2] & 0x80 != 0 { -mag } else { mag }
}
