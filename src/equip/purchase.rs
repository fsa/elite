//! Ввод и покупки: выбор строки, покупка товара, крепление лазера.

use super::Shop;
use super::items::{
    BAY_TONS, FUEL_MAX, ITEMS, Kind, LASER_ITEMS, LASER_NAMES, MAX_ITEMS, MISSILES_MAX, MOUNTS,
};
use super::layout::{OFF_FREE_CARGO, OFF_FUEL, OFF_LASERS, OFF_MINERAL, OFF_MISSILES};

impl Shop {
    /// Сдвинуть выбор клавиатурой (стрелки вверх/вниз).
    pub fn move_selection(&mut self, delta: i32) {
        self.pending = None;
        if self.rows.is_empty() {
            return;
        }
        let n = self.rows.len() as i32;
        let cur = self.cursor as i32 + delta;
        self.cursor = cur.clamp(0, n - 1) as usize;
    }

    /// Цифра номера товара (как в `sub_E776`: первая `1..9`, вторая `0..9`,
    /// Enter подтверждает однозначный номер). В модалке крепления цифра `0..3`
    /// выбирает позицию — как в оригинале (`cp 30h; cp 34h`).
    pub fn input_digit(&mut self, mem: &mut [u8], d: u8) {
        if self.mount.is_some() {
            if d < 4 {
                self.choose_mount(mem, d);
            }
            return;
        }
        match self.pending.take() {
            None => {
                if (1..=9).contains(&d) {
                    self.pending = Some(d);
                }
            }
            Some(first) => {
                let number = first * 10 + d;
                self.buy_number(mem, number);
            }
        }
    }

    /// Enter: подтвердить набранный номер либо купить выбранную строку.
    pub fn confirm(&mut self, mem: &mut [u8]) {
        if self.mount.is_some() {
            return;
        }
        match self.pending.take() {
            Some(first) => self.buy_number(mem, first),
            None => {
                if let Some(&number) = self.rows.get(self.cursor) {
                    self.buy_number(mem, number);
                }
            }
        }
    }

    /// Клик мыши: покупка строки под указателем либо выбор крепления.
    pub fn click(&mut self, mem: &mut [u8]) {
        if self.mount.is_some() {
            if let Some(m) = self.mount_hover {
                self.choose_mount(mem, m);
            }
            return;
        }
        if let Some(i) = self.hover {
            if let Some(&number) = self.rows.get(i) {
                self.buy_number(mem, number);
            }
        }
    }

    /// Отмена окна выбора крепления (деньги ещё не списаны — просто закрыть).
    pub fn cancel_mount(&mut self) {
        self.mount = None;
        self.mount_hover = None;
    }

    /// Купить товар `number` (1..14) — валидация до списания денег.
    pub fn buy_number(&mut self, mem: &mut [u8], number: u8) {
        if self.mount.is_some() {
            return;
        }
        if self.in_flight {
            // Оригинал сюда вообще не попадает: `Cmd_4` вызывается только с
            // экрана станции. Для F4-варианта — запрет с подсказкой.
            self.set_toast("DOCK AT A STATION TO BUY EQUIPMENT");
            return;
        }
        if !(1..=MAX_ITEMS as u8).contains(&number) || !self.rows.contains(&number) {
            self.set_toast("NO SUCH ITEM");
            return;
        }
        let item = &ITEMS[(number - 1) as usize];
        let price = self.price(mem, number);
        match item.kind {
            Kind::Fuel => {
                // Оригинал (`loc_EA3B`): обнуляет динамическую цены топлива,
                // зовёт `SetPilotFuel(70h)` и перерисовывает список. Проверки
                // «полный бак» нет — полный бак стоит 0 и это no-op.
                if !self.pay(mem, price) {
                    return;
                }
                mem[self.layout.prices as usize + 1] = 0;
                self.fuel = FUEL_MAX;
                mem[self.pilot() + OFF_FUEL] = FUEL_MAX;
                self.write_fuel_derived(mem);
                self.set_toast("FUEL TANK FULL");
            }
            Kind::Missile => {
                // `loc_EA3B`: `cp 4; jp nc, <ALL PRESENT>; inc a`.
                if self.missiles >= MISSILES_MAX {
                    // Оригинал: «ALL PRESENT» (токен 0x7C).
                    self.set_toast(format!(
                        "MISSILES FULL ({}/{})",
                        self.missiles, MISSILES_MAX
                    ));
                    return;
                }
                if !self.pay(mem, price) {
                    return;
                }
                self.missiles += 1;
                mem[self.pilot() + OFF_MISSILES] = self.missiles;
            }
            Kind::Bay => {
                if self.fitted[number as usize] {
                    self.set_toast(format!("{} ALREADY FITTED", item.name));
                    return;
                }
                if !self.pay(mem, price) {
                    return;
                }
                // `loc_EA57`: `add a,0Fh` в PilotFreeCargo.
                self.free_cargo = self.free_cargo.wrapping_add(BAY_TONS);
                mem[self.pilot() + OFF_FREE_CARGO] = self.free_cargo;
                // Миссионная метка отсека у Кладова (`sub_6FFB`): при покупке
                // превращается в настоящий отсек, минеральная жизнь — прочь.
                if mem[self.flag_addr(3)] == 0x59 {
                    mem[self.pilot() + OFF_MINERAL] = 0;
                }
                self.mark_fitted(mem, number);
            }
            Kind::Flag => {
                if self.fitted[number as usize] {
                    self.set_toast(format!("{} ALREADY FITTED", item.name));
                    return;
                }
                if !self.pay(mem, price) {
                    return;
                }
                self.mark_fitted(mem, number);
            }
            Kind::Laser(_) => {
                // Деньги списываются только при выборе крепления (оригинал
                // списывает сразу, но при неверной клавише откатывает снимок
                // `sub_E85D` — итог тот же).
                if self.cash < price {
                    self.set_toast("NOT ENOUGH CASH");
                    return;
                }
                self.mount = Some(number);
                self.mount_hover = None;
            }
        }
        self.refresh(mem);
    }

    /// Закрепить лазер товара `self.mount` в позиции `m` (0..3).
    ///
    /// Оригинал (`RAM:EAF5..loc_EB1C`): тип лазера пишется в слот
    /// `PilotFrontLaser+крепление`, старый лазер снимается и его полная цена
    /// возвращается по таблице `{5,6,13,14}`.
    pub fn choose_mount(&mut self, mem: &mut [u8], m: u8) {
        let Some(number) = self.mount.take() else {
            return;
        };
        self.mount_hover = None;
        let Kind::Laser(laser) = ITEMS[(number - 1) as usize].kind else {
            return;
        };
        let price = self.price(mem, number);
        if !self.pay(mem, price) {
            self.refresh(mem);
            return;
        }
        let slot = self.pilot() + OFF_LASERS + m as usize;
        let old = mem[slot];
        mem[slot] = laser;
        if old != 0 && (1..=4).contains(&old) {
            let back = self.price(mem, LASER_ITEMS[old as usize]);
            self.refund(mem, back);
        }
        self.set_toast(format!("{} LASER MOUNTED {}", LASER_NAMES[laser as usize], MOUNTS[m as usize]));
        self.refresh(mem);
    }
}
