//! Чтение и запись состояния командира: список товаров, цены и деньги.

use super::Shop;
use super::items::{MAX_ITEMS, TOAST_FRAMES};
use super::layout::{
    FLAG_BASE, OFF_CASH1, OFF_CASH2, OFF_COBRA, OFF_FREE_CARGO, OFF_FUEL, OFF_LASERS,
    OFF_MISSILES, OFF_TECH,
};

impl Shop {
    /// Прочитать состояние командира и построить список товаров.
    pub(super) fn refresh(&mut self, mem: &[u8]) {
        let p = self.pilot();
        let cash2 = u16::from_le_bytes([mem[p + OFF_CASH2], mem[p + OFF_CASH2 + 1]]) as u32;
        let cash1 = u16::from_le_bytes([mem[p + OFF_CASH1], mem[p + OFF_CASH1 + 1]]) as u32;
        self.cash = cash2 * 10000 + cash1;
        self.fuel = mem[p + OFF_FUEL];
        self.missiles = mem[p + OFF_MISSILES];
        self.free_cargo = mem[p + OFF_FREE_CARGO];
        self.tech = mem[p + OFF_TECH];
        for (i, l) in self.lasers.iter_mut().enumerate() {
            *l = mem[p + OFF_LASERS + i];
        }
        self.in_flight = mem[self.layout.in_danger as usize] != 0;
        // Число товаров: `tech+3`, но не больше 14 (`cp 0Ch; jr c; ld a,0Eh`).
        let n = (self.tech as usize + 3).min(MAX_ITEMS);
        self.rows = (1..=n as u8).collect();
        for n in 1..=MAX_ITEMS as u8 {
            self.fitted[n as usize] = self.flag_fitted(mem, n);
        }
        self.cursor = self.cursor.min(self.rows.len().saturating_sub(1));
        // Наведение переживает перерисовку (после клика список обновляется,
        // но указатель остаётся на той же строке).
        self.hover = self.hover.filter(|&i| i < self.rows.len());
    }

    /// Ссылка на байт флага товара `number` (1..12) в блоке командира.
    pub(super) fn flag_addr(&self, number: u8) -> usize {
        self.pilot() + FLAG_BASE + number as usize
    }

    /// «Установлено» ли оборудование товара `number` (только флаговые).
    ///
    /// У Кладова (`sub_EC4B`): `(flag|0x80)+1 == byte_D08E`; для грузового
    /// отсека `sub_6FFB` сравнивает сырое значение (`flag+1 == byte_D08E`),
    /// и отдельно обрабатывает миссионную метку `0x59` (товар считается
    /// «не установленным», при покупке метка конвертируется в настоящий
    /// отсек).
    fn flag_fitted(&self, mem: &[u8], number: u8) -> bool {
        let flag = mem[self.flag_addr(number)];
        let cobra = mem[self.pilot() + OFF_COBRA];
        if number == 3 {
            flag.wrapping_add(1) == cobra
        } else {
            (flag | 0x80).wrapping_add(1) == cobra
        }
    }

    /// Отметить товар купленным (см. [`Self::flag_fitted`]).
    pub(super) fn mark_fitted(&mut self, mem: &mut [u8], number: u8) {
        // `loc_EA75`: `dec (hl)` — с нуля получается 0xFF.
        let a = self.flag_addr(number);
        mem[a] = mem[a].wrapping_sub(1);
    }

    /// Цена товара `number` (1..14) из таблицы, десятые.
    ///
    /// `sub_EB2F`: запись `(number-1)*3` от базы: `b0*10000 + b2*256 + b1`.
    /// Для FUEL средний байт — динамическая цена недостающего топлива
    /// (`byte_7060`).
    pub(super) fn price(&self, mem: &[u8], number: u8) -> u32 {
        let base = self.layout.prices as usize + (number as usize - 1) * 3;
        let b0 = mem[base] as u32;
        let b1 = mem[base + 1] as u32;
        let b2 = mem[base + 2] as u32;
        b0 * 10000 + b2 * 256 + b1
    }

    /// Цена для **отображения**.
    pub(super) fn price_shown(&self, mem: &[u8], number: u8) -> u32 {
        let base = self.layout.prices as usize + (number as usize - 1) * 3;
        let b0 = mem[base] as u32;
        let b1 = mem[base + 1] as u32;
        let b2 = mem[base + 2] as u32;
        b0 * 10000 + b2 * 256 + b1
    }

    /// Списать деньги (`sub_D3EF` + `sub_D3FD` — то же самое вычитание).
    pub(super) fn pay(&mut self, mem: &mut [u8], price: u32) -> bool {
        if self.cash < price {
            // Оригинал: «CASH?» (токен 0xC5) плюс откат снимка `sub_E85D`.
            self.set_toast("NOT ENOUGH CASH");
            return false;
        }
        self.cash -= price;
        self.write_cash(mem);
        true
    }

    /// Вернуть деньги (`loc_D3E7` + `IncCASH` — возврат цены старого лазера).
    pub(super) fn refund(&mut self, mem: &mut [u8], sum: u32) {
        self.cash += sum;
        self.write_cash(mem);
    }

    fn write_cash(&mut self, mem: &mut [u8]) {
        let p = self.pilot();
        let (c2, c1) = ((self.cash / 10000) as u16, (self.cash % 10000) as u16);
        mem[p + OFF_CASH2..p + OFF_CASH2 + 2].copy_from_slice(&c2.to_le_bytes());
        mem[p + OFF_CASH1..p + OFF_CASH1 + 2].copy_from_slice(&c1.to_le_bytes());
    }

    pub(super) fn set_toast(&mut self, msg: impl Into<String>) {
        self.toast = Some(msg.into());
        self.toast_frames = TOAST_FRAMES;
    }

    /// Обновить производную переменную `Fuel` (`SetPilotFuel`:
    /// `Fuel = 2*fuel + fuel/4` — дальность прыжка в шагах `fuel>>2`).
    pub(super) fn write_fuel_derived(&self, mem: &mut [u8]) {
        let f = self.fuel;
        mem[self.layout.fuel_derived as usize] = f.wrapping_mul(2).wrapping_add(f / 4);
    }
}
