//! Раскладка переменных панели в памяти Z80 и чтение показаний.
//!
//! Здесь собраны адреса переменных игры для обеих версий ([`ELITE_COCKPIT`],
//! [`KLADOV_COCKPIT`]) и преобразование байтов живой памяти в снимок
//! [`CockpitValues`]. Отрисовка о памяти ничего не знает.

use super::values::{
    CockpitValues, Compass, DEFLECTION_MAX, Danger, GAUGE_MAX, Indicator, MISSILE_SLOTS,
    RadarContact, RadarContactKind, RadarContacts,
};

/// Размер слота `SlotObj` в памяти Z80.
const SLOT_SIZE: u16 = 0x27;
/// Смещение `oEnerg` в слоте: 0 — слот пуст.
const SLOT_OENERG: u16 = 0x22;
/// Число основных слотов объектов (без станции и ракеты).
const SHIP_SLOTS: u16 = 6;

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
    /// Признак межпланетного пространства (`InFreeSpace`).
    pub in_free_space: u16,
    /// Координаты планеты (`PlanetCrd`, 9 байт знак-величина).
    pub planet_crd: u16,
    /// Фиктивные координаты планеты вне системы (`fsPlCrdXl`, 9 байт).
    pub fake_planet_crd: u16,
    /// Слот станции (`objCoriolis`).
    pub coriolis_slot: u16,
    /// Первый из шести основных слотов объектов (`DataOfAround`).
    pub ship_slot0: u16,
    /// Слот ракеты (`objMissile`).
    pub missile_slot: u16,
    /// Флаг доступности J-драйва у станции (`J_Drive_Coriolis`).
    pub j_drive_coriolis: u16,
    /// Признак близости станции (`CoriolisNear`), включает символ `S`.
    pub coriolis_near: u16,
    /// Оставшееся время работы ECM (`TimeWorkECM`), `0` — выключена.
    pub ecm_time: u16,
}

/// Раскладка переменных панели для оригинальной версии (`elite.bin`).
#[allow(dead_code)] // понадобится на следующих шагах заполнения панели
pub const ELITE_COCKPIT: CockpitLayout = CockpitLayout {
    shield_front: 0x7046,
    shield_rear: 0x7048,
    fuel: 0x704A,
    temperature: 0x704C,
    laser_temp: 0x704E,
    altitude: 0x7050,
    speed: 0x7052,
    shield_unit1: 0x7054,
    rotate_lr: 0x705C,
    rotate_du: 0x705E,
    missiles: 0xA845,
    in_free_space: 0x7061,
    planet_crd: 0x706E,
    // Для оригинальной версии адрес фиктивных координат пока не найден.
    fake_planet_crd: 0,
    coriolis_slot: 0x5FA6,
    ship_slot0: 0x5FCD,
    missile_slot: 0x60B7,
    j_drive_coriolis: 0x707B,
    coriolis_near: 0x7077,
    ecm_time: 0xDA16,
};

/// Раскладка переменных панели для версии Кладова (`kladov.bin`), +0xA2.
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
    in_free_space: 0x7103,
    planet_crd: 0x7110,
    fake_planet_crd: 0x8AC3,
    coriolis_slot: 0x6048,
    ship_slot0: 0x606F,
    missile_slot: 0x6159,
    j_drive_coriolis: 0x711D,
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
            radar: self.read_radar(mem),
            station_near: byte(self.coriolis_near) != 0,
            ecm_on: byte(self.ecm_time) != 0,
            blink: true,
        }
    }

    /// Прочитать указатель компаса.
    ///
    /// Цель выбирается как в `PrintCompass`:
    ///
    /// * вне системы — фиктивные координаты `fsPlCrd`;
    /// * рядом с планетой — станция (`objCoriolis`), если J-драйв у неё
    ///   доступен, иначе планета;
    /// * в остальных случаях — планета.
    fn read_compass(&self, mem: &[u8]) -> Compass {
        let byte = |addr: u16| mem.get(addr as usize).copied().unwrap_or(0);
        let target = if self.in_free_space != 0 && byte(self.in_free_space) != 0 {
            (self.fake_planet_crd != 0).then_some(self.fake_planet_crd)
        } else if self.planet_crd != 0 {
            let high =
                (byte(self.planet_crd + 2) | byte(self.planet_crd + 5) | byte(self.planet_crd + 8))
                    & 0x7F;
            let station = self.coriolis_slot != 0
                && self.j_drive_coriolis != 0
                && byte(self.j_drive_coriolis) != 0xFF;
            if high < 2 && station {
                Some(self.coriolis_slot)
            } else {
                Some(self.planet_crd)
            }
        } else {
            None
        };
        let Some(target) = target else {
            return Compass::centered();
        };
        let crd = |offset: u16| {
            read_coordinate(mem.get(target as usize + offset as usize..).unwrap_or(&[]))
        };
        let x = crd(0);
        let y = crd(3);
        let z = crd(6);
        let len = (x * x + y * y + z * z).sqrt();
        let offset = |v: f32| {
            if len < 0.5 {
                0
            } else {
                (v * COMPASS_POINTER_MAX as f32 / len)
                    .round()
                    .clamp(-(COMPASS_POINTER_MAX as f32), COMPASS_POINTER_MAX as f32)
                    as i8
            }
        };
        Compass {
            x: offset(x),
            y: offset(y),
            filled: z >= 0.0,
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
        self.push_slot(mem, self.coriolis_slot, RadarContactKind::Station, &mut contacts);
        for i in 0..SHIP_SLOTS {
            self.push_slot(
                mem,
                self.ship_slot0.wrapping_add(i.wrapping_mul(SLOT_SIZE)),
                RadarContactKind::Ship,
                &mut contacts,
            );
        }
        self.push_slot(mem, self.missile_slot, RadarContactKind::Missile, &mut contacts);
        contacts
    }

    /// Добавить объект из слота Z80, если он существует и целиком в памяти.
    fn push_slot(
        &self,
        mem: &[u8],
        base: u16,
        kind: RadarContactKind,
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
        });
    }
}

/// Максимальное смещение указателя компаса от центра, как в оригинале:
/// `CompasPointerCrdX/Y` лежат в диапазоне `-8..=8`.
const COMPASS_POINTER_MAX: i8 = 8;

/// Атрибут экрана, которым `PrintIndDanger` задаёт цвет кружка опасности
/// (левый верхний угол 2x2 знакоместа, адрес в области атрибутов `0x5A28`).
///
/// Читать нужно именно экранный атрибут, а не переменную `IndDanger`:
/// `PrintIndDanger` в конце каждой печати снова записывает в `IndDanger`
/// значение `0x04` («в полёте»), а также подменяет его на красный при пустых
/// щитах и мигает красным при неполном четвёртом энергоблоке. В памяти игры
/// к концу кадра почти всегда остаётся `0x04`, тогда как на экране в этот
/// момент может быть жёлтый, красный или погасший кружок.
pub(super) const DANGER_ATTR: u16 = 0x5A28;

/// Прочитать текущий цвет индикатора опасности с экрана.
fn read_danger(mem: &[u8]) -> Danger {
    Danger::from_attribute(mem.get(DANGER_ATTR as usize).copied().unwrap_or(0))
}

/// Прочитать 24-битную координату в формате знак-величина.
fn read_coordinate(b: &[u8]) -> f32 {
    read_game_coordinate(b) as f32
}

/// Прочитать 24-битную координату как целое число единиц игры.
fn read_game_coordinate(b: &[u8]) -> i32 {
    if b.len() < 3 {
        return 0;
    }
    let mag = (((b[2] & 0x7F) as i32) << 16) | ((b[1] as i32) << 8) | b[0] as i32;
    if b[2] & 0x80 != 0 { -mag } else { mag }
}
