//! Метрика карты и дословный перенос игровых расчётов корня/топлива.

use super::*;

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
pub fn nearest(planets: &[Planet], metric: Metric, cx: i32, cy_raw: i32) -> Option<&Planet> {
    let mut best: Option<&Planet> = None;
    let mut best_key = i32::MAX;
    for p in planets {
        // Курсор ровно на звезде — игра на этом заканчивает поиск.
        if p.x == cx && p.y_raw == cy_raw {
            return Some(p);
        }
        let d2 = distance2(metric, cx, cy_raw, p);
        if d2 > 0xFFFF {
            continue; // переполнение 16 бит — такую звезду игра пропускает
        }
        let key = d2 | 1;
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
pub fn fuel_to_cursor(metric: Metric, sys_x: i32, sys_y_raw: i32, cx: i32, cy_raw: i32) -> i32 {
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
