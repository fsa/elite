//! Нативная модель галактики Elite: 256 звёзд из 6-байтного семени.
//!
//! Это независимая реализация — те же числа, что даёт Z80, но без эмуляции.
//! Семя галактики лежит в памяти игры (`PilotGalaxyData0`, у Кладова `0xD090`);
//! каждая следующая звезда получается из предыдущей четырьмя шагами
//! `ModifyPlanetData` (у Кладова `0xE063`):
//!
//! ```text
//! (w0, w1, w2) <- (w1, w2, w0 + w1 + w2)   (мод 0x10000)
//! ```
//!
//! Три 16-битных слова `w0..w2` — по два байта (младший, старший). Из них
//! берутся координаты звезды на карте галактики:
//!
//! * `x = старший байт w1` — горизонталь, `0..255`;
//! * `y_raw = старший байт w0` — «сырая» вертикаль `0..255`, экранная строка
//!   `y = (y_raw >> 1) + 24` (поле `24..151`).
//!
//! Из-за деления пополам вертикаль занимает вдвое меньше строк, чем значений
//! `y_raw`: на экране карта сжата по вертикали вдвое относительно горизонтали.
//! Именно поэтому при показе галактики масштабы по осям разные (см. `hdmap`).
//!
//! Расстояния на карте считаются точно так же, как в игре, — вплоть до
//! странностей: модуль разности у Кладова на единицу меньше (см.
//! [`Metric`]), а квадратный корень игра считает своей рутиной
//! [`game_sqrt`], и результат отличается от точного корня на единицу.

/// Число звёзд в галактике.
pub const COUNT: usize = 256;

/// Поле звезды по вертикали: строки `24..=151` (128 строк).
pub const FIELD_TOP: i32 = 24;
pub const FIELD_HEIGHT: i32 = 128;

/// Одна звезда галактики.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Planet {
    /// Порядковый номер в последовательности галактики (`0..255`).
    pub index: usize,
    /// Экранная горизонталь `0..255` — старший байт `w1`.
    pub x: i32,
    /// Сырая вертикаль `0..255` — старший байт `w0`.
    pub y_raw: i32,
    /// Шесть байт состояния звезды: `w0`, `w1`, `w2` (мл., ст.).
    pub state: [u8; 6],
}

impl Planet {
    /// Экранная строка карты `24..151`.
    pub fn y(&self) -> i32 {
        (self.y_raw >> 1) + FIELD_TOP
    }

    /// Радиус кружка планеты (как на малой карте игры, `PrnPlanetOnSmallMap`):
    /// класс `(hi(w2) & 0x0C) >> 2` отображается в `3..5`.
    pub fn radius(&self) -> i32 {
        let c = ((self.state[5] as i32) & 0x0C) >> 2;
        (if c == 0 { 1 } else { c }) + 2
    }
}

/// Особенности расчёта расстояний в конкретной версии игры.
///
/// Модуль разности в игре считается отдельной рутиной, и в версии Кладова
/// она отличается от оригинальной на единицу. Поэтому расстояния в двух
/// версиях игры различаются — и нативная карта обязана повторять именно тот
/// вариант, который считает запущенная игра.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Metric {
    /// Оригинальный Elite: `loc_AD3D` (`cp b; jr nc; ld c,b; ld b,a; ld a,c;
    /// sub b`) считает правильный модуль разности `|a - b|`.
    Exact,
    /// Версия Кладова: `ABS_A_sub_B` (`sub b; ret nc; cpl; ret`) при `a < b`
    /// делает `cpl` от `a - b`, то есть возвращает `|a - b| - 1` — на
    /// единицу меньше правильного. Ошибка заметна на глаз: расстояние до
    /// звезды слева от курсора получается меньше на 0.4 св. года.
    Kladov,
}

impl Metric {
    /// Модуль разности как в игре: `|a - b|`, где `a` — уменьшаемое.
    ///
    /// В игре уменьшаемым всегда бывает координата курсора, а вычитаемым —
    /// координата звезды (или текущей системы), поэтому направление важно:
    /// у Кладова разность на единицу меньше, когда курсор левее (или выше)
    /// другой точки.
    pub fn abs_diff(self, a: i32, b: i32) -> i32 {
        let d = a - b;
        match self {
            Metric::Exact => d.abs(),
            Metric::Kladov if d < 0 => -d - 1,
            Metric::Kladov => d,
        }
    }
}

/// Раскладка адресов галактики в памяти конкретной версии игры.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GalaxyLayout {
    /// Семя галактики (`PilotGalaxyData0`), 6 байт.
    pub seed: usize,
    /// Таблица диграфов имён (`TxtPairOfChr`/`byte_FDB3`), 64 байт.
    pub digraphs: usize,
    /// Номер текущей галактики (`PilotNumGalaxy`, 0..7).
    pub galaxy: usize,
    /// Координаты текущей системы: X и «сырой» Y.
    pub sys_x: usize,
    pub sys_y: usize,
    /// Топливо (`Pilot_Fuel`): задаёт радиус достижимости на карте.
    pub fuel: usize,
    /// Особенности расчёта расстояний в этой версии игры.
    pub metric: Metric,
}

/// Версия Кладова (`zx_bin/kladov.bin`).
pub const KLADOV_GALAXY: GalaxyLayout = GalaxyLayout {
    seed: 0xD090,
    digraphs: 0xFDB3,
    galaxy: 0xD065,
    sys_x: 0xD099,
    sys_y: 0xD097,
    fuel: 0xD082,
    metric: Metric::Kladov,
};

/// Оригинальная версия (`zx_bin/elite.bin`).
pub const ELITE_GALAXY: GalaxyLayout = GalaxyLayout {
    seed: 0xA852,
    digraphs: 0xFBC3,
    galaxy: 0xA827,
    sys_x: 0xA85B,
    sys_y: 0xA859,
    fuel: 0xA844,
    metric: Metric::Exact,
};

/// Шаг `ModifyPlanetData`.
fn twist(s: [u8; 6]) -> [u8; 6] {
    let w0 = u16::from_le_bytes([s[0], s[1]]);
    let w1 = u16::from_le_bytes([s[2], s[3]]);
    let w2 = u16::from_le_bytes([s[4], s[5]]);
    let n0 = w1;
    let n1 = w2;
    let n2 = w0.wrapping_add(w1).wrapping_add(w2);
    [
        n0 as u8,
        (n0 >> 8) as u8,
        n1 as u8,
        (n1 >> 8) as u8,
        n2 as u8,
        (n2 >> 8) as u8,
    ]
}

/// Построить все звёзды галактики из 6-байтного семени.
pub fn planets(seed: [u8; 6]) -> Vec<Planet> {
    let mut s = seed;
    let mut out = Vec::with_capacity(COUNT);
    for index in 0..COUNT {
        out.push(Planet {
            index,
            x: s[3] as i32,
            y_raw: s[1] as i32,
            state: s,
        });
        for _ in 0..4 {
            s = twist(s);
        }
    }
    out
}

/// Имя планеты — порт `GeneratePlanetName` (у Кладова `0xE016`).
///
/// Имя собирается из диграфов таблицы `byte_FDB3` (у Кладова `0xFDB3`).
/// Таблица читается по индексу `hi(w2) & 0x1F`; на каждом шаге состояние
/// прокручивается `ModifyPlanetData`. Шагов три, а если младший байт `w0`
/// имеет бит 6 — четыре. Второй символ диграфа `?` не выводится.
pub fn name(state: [u8; 6], digraphs: &[u8]) -> String {
    let mut s = state;
    let calls = if s[0] & 0x40 != 0 { 4 } else { 3 };
    let mut out = String::new();
    for _ in 0..calls {
        let a = (s[5] & 0x1F) as usize;
        let i = a * 2;
        if a != 0 && i + 1 < digraphs.len() {
            out.push(digraphs[i] as char);
            if digraphs[i + 1] != b'?' {
                out.push(digraphs[i + 1] as char);
            }
        }
        s = twist(s);
    }
    out
}

/// Постоянный цвет планеты на карте галактики.
///
/// Цвет выводится из тех же данных, что и имя: на каждом шаге прокрутки
/// `ModifyPlanetData` в хеш подмешивается индекс диграфа (`hi(w2) & 0x1F`)
/// и байт состояния. Поэтому цвет постоянен для одной и той же звезды, но
/// различается от звезды к звезде — так же, как различаются имена.
pub fn color(state: [u8; 6]) -> u32 {
    // FNV-1a по «имени» звезды.
    let mut s = state;
    let mut h: u32 = 0x811C_9DC5;
    for _ in 0..4 {
        h ^= (s[5] & 0x1F) as u32;
        h = h.wrapping_mul(0x0100_0193);
        h ^= s[0] as u32;
        h = h.wrapping_mul(0x0100_0193);
        s = twist(s);
    }
    h ^= h >> 13;
    h = h.wrapping_mul(0x5BD1_E995);
    h ^= h >> 15;

    let hue = (h % 360) as f32;
    let sat = 0.45 + ((h >> 9) & 0x7F) as f32 / 127.0 * 0.40;
    let light = 0.60 + ((h >> 17) & 0x3F) as f32 / 63.0 * 0.18;
    hsl_to_rgb(hue, sat, light)
}

/// HSL (оттенок `0..360`, насыщенность и светлота `0..1`) в `0xFFRRGGBB`.
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> u32 {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let hp = h / 60.0;
    let x = c * (1.0 - (hp % 2.0 - 1.0).abs());
    let (r, g, b) = match hp as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = l - c / 2.0;
    let ch = |v: f32| (((v + m) * 255.0).round().clamp(0.0, 255.0)) as u32;
    0xFF00_0000 | (ch(r) << 16) | (ch(g) << 8) | ch(b)
}

/// Квадрат расстояния от курсора `(cx, cy_raw)` до звезды — та же метрика,
/// что в игре (`CalcDistanceToCursor2`): `dx^2 + ((dy_raw / 2))^2`. Так как
/// вертикаль карты сжата вдвое, «сырая» разница по вертикали тоже делится
/// пополам. Разности считаются модулем разности версии игры (см.
/// [`Metric::abs_diff`]), уменьшаемым всегда является курсор.
pub fn distance2(metric: Metric, cx: i32, cy_raw: i32, p: &Planet) -> i32 {
    let dx = metric.abs_diff(cx, p.x);
    let dy = metric.abs_diff(cy_raw, p.y_raw) >> 1;
    dx * dx + dy * dy
}

/// Ближайшая к курсору звезда — как ищет игра (`SearchPlanetNearCursor`).
///
/// Игра перебирает все 256 звёзд и оставляет ту, у которой квадрат
/// расстояния до курсора минимален; при равенстве остаётся последняя
/// (значение записывается в `MinDistance` без проверки на «меньше»). Звёзды,
/// для которых сумма не помещается в 16 бит, игра пропускает.
///
/// У Кладова есть ещё одно правило (`CalcDistanceToCursor`): прежде чем
/// считать расстояние, игра сверяет координаты звезды с координатами курсора
/// и, если курсор стоит ровно на звезде, заканчивает поиск этой звездой.
/// Поэтому звезда «под курсором» всегда выбирается, даже если рядом есть
/// другая, столь же близкая.
pub fn nearest(
    planets: &[Planet],
    metric: Metric,
    cx: i32,
    cy_raw: i32,
) -> Option<&Planet> {
    let mut best: Option<&Planet> = None;
    let mut best_key = i32::MAX;
    for p in planets {
        // Курсор ровно на звезде — игра на этом заканчивает поиск.
        if metric == Metric::Kladov && p.x == cx && p.y_raw == cy_raw {
            return Some(p);
        }
        let d2 = distance2(metric, cx, cy_raw, p);
        if d2 > 0xFFFF {
            continue; // переполнение 16 бит — такую звезду игра пропускает
        }
        // У Кладова в `MinDistance` попадает значение с установленным битом 0
        // (`set 0,l` в `CalcDistanceToCursor`), поэтому сравнение идёт по
        // `d2 | 1`: две звезды с квадратами 100 и 101 для игры равны.
        let key = match metric {
            Metric::Kladov => d2 | 1,
            Metric::Exact => d2,
        };
        if key <= best_key {
            best_key = key;
            best = Some(p);
        }
    }
    best
}

/// Регистры Z80, которые меняет рутина извлечения корня.
struct SqrtRegs {
    a: u8,
    b: u8,
    c: u8,
    h: u8,
    l: u8,
    /// Флаг переноса (`carry`).
    carry: u8,
}

impl SqrtRegs {
    /// `srl h`.
    fn srl_h(&mut self) {
        self.carry = self.h & 1;
        self.h >>= 1;
    }

    /// `rr l` — циклический сдвиг вправо через флаг переноса.
    fn rr_l(&mut self) {
        let nc = self.l & 1;
        self.l = (self.l >> 1) | (self.carry << 7);
        self.carry = nc;
    }

    /// `rra`.
    fn rra(&mut self) {
        let nc = self.a & 1;
        self.a = (self.a >> 1) | (self.carry << 7);
        self.carry = nc;
    }

    /// `rr h`.
    fn rr_h(&mut self) {
        let nc = self.h & 1;
        self.h = (self.h >> 1) | (self.carry << 7);
        self.carry = nc;
    }

    /// `srl h; rr l` — сдвиг 16-битного `hl` вправо на бит.
    fn shr16(&mut self) {
        self.srl_h();
        self.rr_l();
    }

    /// `rr h; rr l` — циклический сдвиг 16-битного `hl` вправо.
    fn ror16(&mut self) {
        self.rr_h();
        self.rr_l();
    }

    /// `srl h; rr l; rra` — сдвиг 24-битного `hl:a` вправо на бит.
    fn shr24(&mut self) {
        self.shr16();
        self.rra();
    }

    /// `add hl,bc`.
    fn add_hl_bc(&mut self) {
        let hl = ((self.h as u16) << 8) | self.l as u16;
        let bc = ((self.b as u16) << 8) | self.c as u16;
        let res = hl as u32 + bc as u32;
        self.carry = if res > 0xFFFF { 1 } else { 0 };
        self.h = (res >> 8) as u8;
        self.l = res as u8;
    }
}

/// Деление 24-битного числа на 16-битное — порт `Div_BC_HLAdivDE`
/// (`0xE8D0` оригинала): делимое — `hl:a`, делитель — `de`,
/// частное возвращается в `bc`.
fn div24by16(dividend: u32, divisor: u16) -> u16 {
    let mut h = (dividend >> 16) as u8;
    let mut l = (dividend >> 8) as u8;
    let mut a = dividend as u8;
    // `ld bc,0FFFEh` — в `b` живёт метка конца деления.
    let mut b: u8 = 0xFF;
    let mut c: u8 = 0xFE;
    loop {
        // add a,a ; rl l ; rl h
        let carry = a >> 7;
        a <<= 1;
        let nc = l >> 7;
        l = (l << 1) | carry;
        let carry = nc;
        let nc = h >> 7;
        h = (h << 1) | carry;
        let carry = nc;
        let hl = ((h as u16) << 8) | l as u16;
        if carry != 0 {
            // and a ; sbc hl,de
            let res = hl.wrapping_sub(divisor).wrapping_sub(1);
            h = (res >> 8) as u8;
            l = res as u8;
            // scf ; rl c ; rl b
            let nc = c >> 7;
            c = (c << 1) | 1;
            let carry = nc;
            let nc = b >> 7;
            b = (b << 1) | carry;
            let carry = nc;
            if carry == 0 {
                break;
            }
        } else if hl >= divisor {
            // sbc hl,de ; scf ; rl c ; rl b
            let res = hl - divisor;
            h = (res >> 8) as u8;
            l = res as u8;
            let nc = c >> 7;
            c = (c << 1) | 1;
            let carry = nc;
            let nc = b >> 7;
            b = (b << 1) | carry;
            let carry = nc;
            if carry == 0 {
                break;
            }
        } else {
            // add hl,de ; sla c ; rl b
            let nc = c >> 7;
            c <<= 1;
            let carry = nc;
            let nc = b >> 7;
            b = (b << 1) | carry;
            let carry = nc;
            if carry == 0 {
                break;
            }
        }
    }
    ((b as u16) << 8) | c as u16
}

/// `loc_DC97` — деление и прибавление частного.
fn sqrt4(r: &mut SqrtRegs, de: u16) {
    let bc = div24by16(((r.h as u32) << 16) | ((r.l as u32) << 8) | r.a as u32, de);
    r.b = (bc >> 8) as u8;
    r.c = bc as u8;
    // srl b ; rr c
    let carry = r.b & 1;
    r.b >>= 1;
    let nc = r.c & 1;
    r.c = (r.c >> 1) | (carry << 7);
    r.carry = nc;
    // xor a
    r.a = 0;
    r.carry = 0;
    // pop hl — в `hl` попадает делитель
    r.h = (de >> 8) as u8;
    r.l = de as u8;
    // add hl,bc
    r.add_hl_bc();
}

/// `loc_DC8D` — один шаг итерации.
fn sqrt5(r: &mut SqrtRegs, de: u16) {
    // rr h ; rr l
    r.ror16();
    let old_hl = ((r.h as u16) << 8) | r.l as u16;
    // ex de,hl
    r.h = (de >> 8) as u8;
    r.l = de as u8;
    // srl h ; rr l ; rra
    r.shr24();
    // дальше `loc_DC97` с делителем из `de` (бывший `hl`)
    sqrt4(r, old_hl);
}

/// `loc_DC7A` — основная часть рутины корня.
fn sqrt2(r: &mut SqrtRegs) {
    // xor a
    r.a = 0;
    r.carry = 0;
    // push hl ; push hl
    let x = ((r.h as u16) << 8) | r.l as u16;
    let mut stack = [x, x];
    let mut sp = 2usize;
    // srl h ; rr l ; rra
    r.shr24();
    // ld de,8000h
    sqrt4(r, 0x8000);
    // pop de ; call Sqrt5
    sp -= 1;
    sqrt5(r, stack[sp]);
    // pop de ; Sqrt5 (проходом дальше)
    sp -= 1;
    sqrt5(r, stack[sp]);
}

/// Квадратный корень игры (`Sqrt_H_HL`) — дословный перенос Z80-рутины
/// (`0xDC53` оригинала, `0x9EF5` у Кладова).
///
/// Игра считает корень приближённо: результат отличается от точного
/// `isqrt` на единицу (иногда на две), поэтому «правильный» корень
/// не подходит — числа на карте должны совпадать с оригиналом до
/// последней цифры. Возвращается старший байт результата (в игре — `h`).
pub fn game_sqrt(n: u16) -> u8 {
    let mut r = SqrtRegs {
        a: 0,
        b: 0,
        c: 0,
        h: (n >> 8) as u8,
        l: n as u8,
        carry: 0,
    };
    // ld a,h ; cp 10h
    if r.h >= 0x10 {
        // на верхнюю ветку рутины
        sqrt2(&mut r);
        return r.h;
    }
    if r.h == 0 {
        // ld h,l ; ld l,a
        r.h = r.l;
        r.l = r.a;
        sqrt2(&mut r);
        // ld b,4 ; 4 x (srl h ; rr l)
        for _ in 0..4 {
            r.shr16();
        }
        return r.h;
    }
    // ld b,4 ; 4 x (add hl,hl)
    for _ in 0..4 {
        let hl = ((r.h as u16) << 8) | r.l as u16;
        let res = hl << 1;
        r.h = (res >> 8) as u8;
        r.l = res as u8;
    }
    sqrt2(&mut r);
    // 2 x (srl h ; rr l)
    for _ in 0..2 {
        r.shr16();
    }
    r.h
}

/// Топливо для прыжка (в десятых долях светового года) из квадрата
/// расстояния — как в игре (`CalcFuelForDistance`): `4 * sqrt(d2)`, где
/// корень считает рутина игры [`game_sqrt`]. Нулевое расстояние игра
/// обрабатывает отдельно (корень не считается вовсе), а при переполнении
/// 16 бит подставляет `0xFFFF`, откуда 1020.
pub fn fuel_for_distance(d2: i32) -> i32 {
    if d2 == 0 {
        return 0;
    }
    let n = if d2 > 0xFFFF { 0xFFFFu16 } else { d2 as u16 };
    4 * game_sqrt(n) as i32
}

/// Топливо для прыжка от текущей системы `(sys_x, sys_y_raw)` к курсору
/// `(cx, cy_raw)` — ровно как считает игра (`CalcFuelForJump`).
pub fn fuel_to_cursor(
    metric: Metric,
    sys_x: i32,
    sys_y_raw: i32,
    cx: i32,
    cy_raw: i32,
) -> i32 {
    let dx = metric.abs_diff(cx, sys_x);
    let dy = metric.abs_diff(cy_raw, sys_y_raw) >> 1;
    fuel_for_distance(dx * dx + dy * dy)
}

/// Расстояние от текущей системы до курсора в световых годах — так его
/// показывает игра на карте галактики (шаг — 0.4 св. года).
pub fn distance_to_cursor_ly(
    metric: Metric,
    sys_x: i32,
    sys_y_raw: i32,
    cx: i32,
    cy_raw: i32,
) -> f32 {
    fuel_to_cursor(metric, sys_x, sys_y_raw, cx, cy_raw) as f32 / 10.0
}

/// Радиус достижимости на карте галактики (в единицах поля `x`/строка) для
/// запаса топлива `fuel`: в игре `RadiusCircle = Pilot_Fuel >> 2`
/// (`Cmd_I_GlobalMap` / `CalcFuelForDistance`). Это окружность в метрике
/// [`distance2`], то есть обычный круг в координатах `(x, строка)`.
pub fn jump_radius(fuel: u8) -> i32 {
    (fuel >> 2) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Семя галактики из запущенной версии Кладова (`tmp/wf/mem.bin`, `0xD090`).
    const SEED: [u8; 6] = [0x4A, 0x5A, 0x48, 0x02, 0x53, 0xB7];

    /// Таблица диграфов имён (`byte_FDB3`, 32 пары символов).
    const DIGRAPHS: &[u8] =
        b"ALLEXEGEZACEBISOUSESARMAINDIREA?ERATENBERALAVETIEDORQUANTEISRION";

    #[test]
    fn planets_match_known_galaxy() {
        let p = planets(SEED);
        assert_eq!(p.len(), COUNT);
        // Текущая система Джеймсона — LAVE, звезда №7: (x=20, y_raw=173).
        assert_eq!((p[7].x, p[7].y_raw), (20, 173));
        assert_eq!(p[7].state.to_vec(), b"\x38\xad\x9c\x14\x1d\x15".to_vec());
        // Система ORESQU — звезда №204: (x=22, y_raw=217).
        assert_eq!((p[204].x, p[204].y_raw), (22, 217));
        assert_eq!(p[204].state.to_vec(), b"\xba\xd9\x90\x16\xb3\xf9".to_vec());
    }

    #[test]
    fn planet_names_match_game() {
        let p = planets(SEED);
        assert_eq!(name(p[7].state, DIGRAPHS), "LAVE");
        assert_eq!(name(p[204].state, DIGRAPHS), "ORESQU");
    }

    #[test]
    fn planet_color_is_stable_and_varied() {
        let p = planets(SEED);
        // Постоянство: одинаковые данные дают одинаковый цвет.
        assert_eq!(color(p[7].state), color(p[7].state));
        assert_ne!(color(p[7].state), color(p[204].state));
        // Цвет непрозрачный и не чёрный.
        for pl in &p {
            let c = color(pl.state);
            assert_eq!(c >> 24, 0xFF, "планета {}", pl.index);
            assert_ne!(c & 0x00FF_FFFF, 0, "планета {}", pl.index);
        }
        // Разные звёзды — разные цвета.
        let distinct: std::collections::HashSet<u32> =
            p.iter().map(|pl| color(pl.state)).collect();
        assert!(distinct.len() > 200, "слишком мало цветов: {}", distinct.len());
    }

    #[test]
    fn radius_is_three_to_five() {
        let p = planets(SEED);
        for pl in &p {
            let r = pl.radius();
            assert!((3..=5).contains(&r), "radius {r} for planet {}", pl.index);
        }
    }

    #[test]
    fn nearest_uses_squeezed_vertical_metric() {
        let p = planets(SEED);
        // Точка ровно на звезде №7 — она и ближайшая.
        assert_eq!(nearest(&p, Metric::Kladov, 20, 173).unwrap().index, 7);
        // Сдвиг по «сырой» вертикали на 4 — это 2 экранные строки.
        assert_eq!(distance2(Metric::Kladov, 20, 177, &p[7]), 4);
        // Горизонталь не делится: сдвиг на 4 даёт 16.
        assert_eq!(distance2(Metric::Kladov, 24, 173, &p[7]), 16);
    }

    /// Расстояние на карте галактики: LAVE (№7) -> DISO (№147). В версии
    /// Кладова 3.2 св. года, в оригинале Elite — 3.6: модуль разности у
    /// Кладова на единицу меньше, когда курсор левее системы.
    #[test]
    fn distance_in_light_years_matches_game() {
        let p = planets(SEED);
        assert_eq!((p[147].x, p[147].y_raw), (11, 174));
        assert_eq!(
            fuel_to_cursor(Metric::Kladov, p[7].x, p[7].y_raw, p[147].x, p[147].y_raw),
            32
        );
        assert_eq!(
            distance_to_cursor_ly(
                Metric::Kladov,
                p[7].x,
                p[7].y_raw,
                p[147].x,
                p[147].y_raw
            ),
            3.2
        );
        assert_eq!(
            fuel_to_cursor(Metric::Exact, p[7].x, p[7].y_raw, p[147].x, p[147].y_raw),
            36
        );
        assert_eq!(
            distance_to_cursor_ly(
                Metric::Exact,
                p[7].x,
                p[7].y_raw,
                p[147].x,
                p[147].y_raw
            ),
            3.6
        );
        // До самой системы — ноль.
        assert_eq!(fuel_to_cursor(Metric::Kladov, p[7].x, p[7].y_raw, 20, 173), 0);
    }

    /// LAVE (№7) -> RIEDQUAD (№46): в версии Кладова 6.4 св. года, в
    /// оригинале Elite — 6.8 (разница ровно в модуле разности).
    #[test]
    fn distance_to_riedquad_matches_game() {
        let p = planets(SEED);
        assert_eq!((p[46].x, p[46].y_raw), (3, 181));
        assert_eq!(
            fuel_to_cursor(Metric::Kladov, p[7].x, p[7].y_raw, p[46].x, p[46].y_raw),
            64
        );
        assert_eq!(
            distance_to_cursor_ly(
                Metric::Kladov,
                p[7].x,
                p[7].y_raw,
                p[46].x,
                p[46].y_raw
            ),
            6.4
        );
        assert_eq!(
            fuel_to_cursor(Metric::Exact, p[7].x, p[7].y_raw, p[46].x, p[46].y_raw),
            68
        );
        assert_eq!(
            distance_to_cursor_ly(
                Metric::Exact,
                p[7].x,
                p[7].y_raw,
                p[46].x,
                p[46].y_raw
            ),
            6.8
        );
    }

    /// Радиус достижимости: топливо 70 -> 17 единиц поля (как в карте I).
    #[test]
    fn jump_radius_matches_game() {
        assert_eq!(jump_radius(70), 17);
        assert_eq!(jump_radius(7), 1);
        assert_eq!(jump_radius(0), 0);
    }

    /// Корень игры считается приближённо: `Sqrt_H_HL` возвращает не точный
    /// `isqrt`, а число, отличающееся на единицу (иногда на две). Опорные
    /// значения сняты с настоящего кода игры (см. `probe`).
    #[test]
    fn game_sqrt_matches_game_values() {
        assert_eq!(game_sqrt(0), 2);
        assert_eq!(game_sqrt(1), 2);
        assert_eq!(game_sqrt(64), 8);
        assert_eq!(game_sqrt(100), 10);
        assert_eq!(game_sqrt(256), 16);
        assert_eq!(game_sqrt(7690), 88);
        assert_eq!(game_sqrt(0xFFFF), 255);
        // Точное значение для сравнения: 7690 -> 87.69 -> 88.
        assert_eq!(game_sqrt(7690), 88);
    }

    /// Топливо для прыжка считается так же, как в игре: четыре корня.
    #[test]
    fn fuel_for_distance_matches_game() {
        assert_eq!(fuel_for_distance(0), 0);
        assert_eq!(fuel_for_distance(64), 32); // 3.2 св. года
        assert_eq!(fuel_for_distance(272), 64); // 6.4 св. года
        assert_eq!(fuel_for_distance(305), 68); // 6.8 св. года
        // Переполнение 16 бит: игра подставляет 0xFFFF -> 1020.
        assert_eq!(fuel_for_distance(0x10000), 1020);
    }
}
