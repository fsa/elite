//! Ввод и покупки: выбор строки, покупка товара, крепление лазера.

use super::Shop;
use super::items::{
    BAY_TONS, FUEL_MAX, ITEMS, Kind, LASER_ITEMS, LASER_NAMES, MAX_ITEMS, MISSILES_MAX, MOUNTS,
};
use super::layout::{OFF_FREE_CARGO, OFF_FUEL, OFF_LASERS, OFF_MINERAL, OFF_MISSILES};

impl Shop {
    /// Сдвинуть выбор клавиатурой (стрелки вверх/вниз).
    pub fn move_selection(&mut self, delta: i32) {
        if self.rows.is_empty() {
            return;
        }
        let n = self.rows.len() as i32;
        let cur = self.cursor as i32 + delta;
        self.cursor = cur.clamp(0, n - 1) as usize;
    }

    /// Цифра в модалке крепления лазера: `1..4` = FRONT/REAR/LEFT/RIGHT.
    /// `0..3` тоже принимаются для совместимости с оригинальной раскладкой.
    pub fn input_digit(&mut self, mem: &mut [u8], d: u8) {
        if self.mount.is_some() {
            let slot = match d {
                1..=4 => d - 1,
                0 => 0,
                _ => return,
            };
            self.mount_sel = slot;
            self.choose_mount(mem, slot);
        }
    }

    /// Enter: в модалке — подтвердить выбранное крепление, иначе купить строку.
    pub fn confirm(&mut self, mem: &mut [u8]) {
        if self.mount.is_some() {
            self.choose_mount(mem, self.mount_sel);
            return;
        }
        if let Some(&number) = self.rows.get(self.cursor) {
            self.buy_number(mem, number);
        }
    }

    /// Сдвинуть выбор крепления в модалке.
    pub fn move_mount_selection(&mut self, delta: i32) {
        if self.mount.is_none() {
            return;
        }
        self.mount_sel = (self.mount_sel as i32 + delta).clamp(0, 3) as u8;
    }

    /// Клик мыши: покупка строки под указателем либо выбор крепления.
    pub fn click(&mut self, mem: &mut [u8]) {
        if self.mount.is_some() {
            if let Some(m) = self.mount_hover {
                self.mount_sel = m;
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
        self.mount_sel = 0;
        self.mount_hover = None;
    }

    /// Купить товар `number` (1..14) — валидация до списания денег.
    pub fn buy_number(&mut self, mem: &mut [u8], number: u8) {
        if self.mount.is_some() {
            return;
        }
        if self.in_flight {
            // Оригинал сюда вообще не попадает: `Cmd_4` вызывается только с
            // экрана станции. Для нативного экрана — запрет с подсказкой.
            self.set_toast("Dock at a station to buy equipment");
            return;
        }
        if !(1..=MAX_ITEMS as u8).contains(&number) || !self.rows.contains(&number) {
            self.set_toast("No such item");
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
                self.set_toast("Fuel tank full");
            }
            Kind::Missile => {
                // `loc_EA3B`: `cp 4; jp nc, <ALL PRESENT>; inc a`.
                if self.missiles >= MISSILES_MAX {
                    // Оригинал: «ALL PRESENT» (токен 0x7C).
                    self.set_toast(format!(
                        "Missiles full ({}/{})",
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
                    self.set_toast(format!("{} already fitted", item.name));
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
                    self.set_toast(format!("{} already fitted", item.name));
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
                    self.set_toast("Not enough cash");
                    return;
                }
                self.mount = Some(number);
                self.mount_sel = 0;
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
        self.mount_sel = 0;
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
        self.set_toast(format!(
            "{} laser mounted {}",
            LASER_NAMES[laser as usize],
            MOUNTS[m as usize].to_ascii_lowercase()
        ));
        self.refresh(mem);
    }
}
