//! Нативный экран закупки оборудования (EQUIP SHIP) — клавиша F4.
//!
//! Оригинальная команда `Cmd_4_EquipShip` (клавиша `4` главного меню на
//! станции) печатает список текстовым драйвером ZX и читает номер товара с
//! клавиатуры (`sub_E776`: две цифры плюс Enter). Этот модуль — нативная
//! (Rust) замена: полноэкранный экран на HD-слое [`crate::display::Frame`],
//! с мышью и клавиатурой; игра на паузе, ввод не уходит в эмулятор. Данные
//! читаются и пишутся прямо в живой блок командира в памяти Z80, поэтому
//! покупки настоящие — после закрытия экрана игра видит изменения.
//!
//! Логика покупок соответствует версии Кладова:
//!
//! - цены — таблица из 14 трёхбайтовых записей ([`ShopLayout::prices`]):
//!   `b0*10000 + b2*256 + b1` десятых; для FUEL первая запись — динамическая
//!   цена недостающего топлива в среднем байте (`sub_9B83` пишет её в
//!   `prices+1`, при покупке топлива она обнуляется);
//! - доступно `N = min(tech+3, 14)` товаров (`sub_D8CE`/`byte_EB5E`);
//! - ракет — максимум 4 (`inc a; cp 4` -> «ALL PRESENT»);
//! - большой грузовой отсек добавляет 15 тонн (`add a,0Fh`);
//! - лазер (товары 5, 6, 13, 14) крепится на выбор `0..3` =
//!   FRONT/REAR/LEFT/RIGHT (`pilot+41+крепление`); при замене старого лазера
//!   возвращается его полная цена по таблице `{5,6,13,14}` (`loc_FD87+2`);
//! - топливо заправляется до 70 (`SetPilotFuel`) и обновляет производную
//!   переменную `Fuel = 2*fuel + fuel/4` (дальность прыжка);
//! - деньги хранятся как `cash2*10000 + cash1` десятых и списываются так же,
//!   как в оригинале (`sub_D3EF` вычитает `b0` из «тысяч», `sub_D3FD` —
//!   остаток из «десятых» с заёмом).
//!
//! Осознанные отклонения от оригинала (без ущерба игровому процессу):
//!
//! - оригинал списывает деньги **до** проверок и теряет их при ошибках
//!   «PRESENT»/«ALL PRESENT» (снимок состояния `sub_E83C`/`sub_E85D`
//!   откатывается только для ошибки «CASH?» и неверной клавиши крепления);
//!   здесь сначала проверка, потом оплата — деньги не пропадают;
//! - тексты ошибок — современные тосты («NOT ENOUGH CASH»,
//!   «<ТОВАР> ALREADY FITTED», «MISSILES FULL (4/4)»); оригинальные
//!   «CASH?», «<ТОВАР> PRESENT», «ALL PRESENT» оставлены только здесь,
//!   в комментариях;
//! - в оригинале ввод номера — двухзначное число с подтверждением Enter, а
//!   любая «не та» клавиша выкидывает в главное меню; здесь цифры набираются
//!   так же (первая `1..9`, вторая `0..9`), но ошибка номера просто
//!   показывает тост.
//!
//! Список товаров рисуется как в игре, а покупка списывает цену из таблицы по
//! индексу (`sub_EB2F`).
//!
//! Подсистемы модуля:
//!
//! - [`items`] — товары, их типы и константы экрана;
//! - [`layout`] — адреса блока командира и раскладка экрана закупки;
//! - [`state`] — чтение/запись состояния командира, цены и деньги;
//! - [`purchase`] — ввод, покупка и выбор крепления лазера;
//! - [`geometry`] — геометрия окна и попадание мышью;
//! - [`render`] — отрисовка экрана, панели и модального окна.

use crate::galaxy;

mod geometry;
mod items;
mod layout;
mod purchase;
mod render;
mod state;
#[cfg(test)]
mod tests;

pub use self::layout::{KLADOV_SHOP, ShopLayout};

use self::items::MAX_ITEMS;

/// Состояние и отрисовка экрана закупки.
pub struct Shop {
    /// Показывается ли экран сейчас.
    pub active: bool,
    layout: ShopLayout,
    /// Номера товаров в порядке списка (FUEL может отсутствовать).
    rows: Vec<u8>,
    /// «Установлено» по номеру товара (1..14).
    fitted: [bool; MAX_ITEMS + 1],
    /// Деньги десятыми (`cash2*10000 + cash1`).
    cash: u32,
    fuel: u8,
    missiles: u8,
    free_cargo: u8,
    tech: u8,
    /// Типы лазеров в креплениях FRONT/REAR/LEFT/RIGHT (0 — нет).
    lasers: [u8; 4],
    /// Имя текущей системы (из модели галактики).
    system: String,
    /// <>0 в полёте (`IndDanger`) — покупки запрещены.
    in_flight: bool,
    /// Выбранная строка списка (клавиатура).
    cursor: usize,
    /// Строка под указателем мыши.
    hover: Option<usize>,
    /// Первая цифра номера товара (как в `sub_E776`).
    pending: Option<u8>,
    /// Some(номер товара-лазера): открыто окно выбора крепления.
    mount: Option<u8>,
    /// Вариант крепления под указателем мыши.
    mount_hover: Option<u8>,
    /// Текст тоста и оставшиеся кадры показа.
    toast: Option<String>,
    toast_frames: u32,
    win_w: u32,
    win_h: u32,
}

impl Shop {
    pub fn new() -> Self {
        Shop {
            active: false,
            layout: KLADOV_SHOP,
            rows: Vec::new(),
            fitted: [false; MAX_ITEMS + 1],
            cash: 0,
            fuel: 0,
            missiles: 0,
            free_cargo: 0,
            tech: 0,
            lasers: [0; 4],
            system: String::new(),
            in_flight: false,
            cursor: 0,
            hover: None,
            pending: None,
            mount: None,
            mount_hover: None,
            toast: None,
            toast_frames: 0,
            win_w: 0,
            win_h: 0,
        }
    }

    /// Открыть экран: снять состояние командира из памяти запущенной игры.
    pub fn open(&mut self, mem: &[u8], layout: ShopLayout) {
        self.layout = layout;
        // Имя текущей системы — та же модель галактики, что у карты F10.
        let gl = layout.galaxy;
        let seed: [u8; 6] = mem[gl.seed..gl.seed + 6].try_into().unwrap();
        let planets = galaxy::planets(seed);
        let (sx, sy) = (mem[gl.sys_x] as i32, mem[gl.sys_y] as i32);
        self.system = galaxy::nearest(&planets, gl.metric, sx, sy)
            .map(|p| galaxy::name(p.state, &mem[gl.digraphs..gl.digraphs + 64]))
            .unwrap_or_default();
        self.refresh(mem);
        self.cursor = 0;
        self.hover = None;
        self.pending = None;
        self.mount = None;
        self.mount_hover = None;
        self.toast = None;
        self.toast_frames = 0;
        self.active = true;
    }

    /// Закрыть экран (возврат к экрану игры; изменения уже в памяти).
    pub fn close(&mut self) {
        self.active = false;
    }

    /// Запомнить размер окна (до первого кадра), чтобы попадание мышью
    /// работало сразу после открытия.
    pub fn resize(&mut self, w: u32, h: u32) {
        self.win_w = w;
        self.win_h = h;
    }

    /// Текст текущего тоста (для тестов и отладки).
    #[allow(dead_code)] // читают тесты
    pub fn toast_text(&self) -> Option<&str> {
        self.toast.as_deref()
    }

    /// Открыто ли окно выбора крепления (Esc закрывает его, а не экран).
    pub fn mount_open(&self) -> bool {
        self.mount.is_some()
    }

    /// Деньги командира десятыми (для тестов и отладки).
    #[allow(dead_code)] // читают тесты
    pub fn cash_tenths(&self) -> u32 {
        self.cash
    }

    pub(super) fn pilot(&self) -> usize {
        self.layout.pilot as usize
    }
}

impl Default for Shop {
    fn default() -> Self {
        Shop::new()
    }
}
