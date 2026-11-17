//! Нативный экран цен — `F8` (альтернатива оригинальной `K`).
//!
//! Показывает текущие цены на товары станции, прочитанные из памяти Z80,
//! остатки на складе и количество каждого товара в трюме корабля. Экран можно
//! открыть и в полёте — цены там те же, что у оригинальной команды `K`, но
//! торговать выбранной строкой можно только на станции: `←` продаёт одну
//! единицу, `→` покупает.
//! Оформление в стиле экранов информации о планете и командира.
//!
//! Цены пересчитываются самим Z80 в `InitGame`/`GenerateAnotherPrice` при
//! пристыковке: параметры из `TableBasePrice` (17 x 5 байт) комбинируются с
//! `PlanetEconomy` и `PilotRnd`, результат пишется в `ProductPrice`. Остатки
//! лежат отдельно, сразу за `PilotPlanetGov`. Нативный экран не считает цены
//! сам, а читает готовые таблицы — поэтому он всегда совпадает с эмулятором.
//!
//! Покупка/продажа повторяет оригинальные `sub_E77B` (`Cmd_2_BuyProducts`) и
//! `sub_E935` (`Cmd_3_SellProducts`): деньги, остатки, трюм и «криминальные»
//! товары меняются прямо в живом блоке командира, так что Z80 сразу видит
//! результат сделки.

use crate::display::Frame;
use crate::flight;

mod draw;
#[cfg(test)]
mod tests;

/// Адрес таблицы параметров генерации цен `TableBasePrice` (версия Кладова).
/// 17 записей по 5 байт: базовая цена, экономика, разброс и количество.
/// Используется Z80 в `GenerateAnotherPrice`; нативный экран читает уже
/// готовые [`ADDR_PRODUCT_PRICE`] и [`ADDR_QUANTITY`].
const ADDR_TABLE_BASE_PRICE: usize = 0x701B;
/// Адрес таблицы цен `ProductPrice` (версия Кладова).
/// 17 записей по 2 байта (LE), цена в десятых кредита. Оттуда же читает
/// оригинальный экран рынка (`Cmd_2_BuyProducts`).
const ADDR_PRODUCT_PRICE: usize = 0xE1BD;
/// Адрес таблицы остатков товаров — `PilotPlanetGov + 1` (версия Кладова).
/// 17 байт, по одному на товар; именно оттуда оригинал берёт «QUANTITY FOR SALE».
const ADDR_QUANTITY: usize = 0xD09E;
/// Число товаров на рынке.
const GOODS_COUNT: usize = 17;

/// Живой блок командира: `PilotName` (совпадает с `equip::KLADOV_SHOP.pilot`).
const PILOT: u16 = 0xD054;
/// `PilotLegalStatus` — растёт при покупке рабов и наркотиков.
const OFF_LEGAL: usize = 11;
/// `PilotCash2low` (u16 LE, «десятки тысяч» десятых кредита).
const OFF_CASH2: usize = 18;
/// `PilotCash1` (u16 LE, «десятые» 0..9999).
const OFF_CASH1: usize = 20;
/// Массив груза — 17 товаров сразу за `PilotRnd` (`PilotRnd + 1`).
const OFF_CARGO: usize = 23;
/// `PilotFreeCargo` — свободный тоннаж.
const OFF_FREE_CARGO: usize = 40;
/// `PilotExtWeapon`: бит 5 запрещает торговлю (в трюме десант/беженцы).
const OFF_EXT_WEAPON: usize = 92;
/// Максимум, который можно записать в две ячейки денег (`cash2*10000+cash1`).
const MAX_CASH_TENTHS: u32 = 65535 * 10000 + 9999;

/// Названия товаров в порядке таблицы рынка (версия Кладова).
const GOODS: [&str; GOODS_COUNT] = [
    "Food",
    "Textiles",
    "Radioactives",
    "Slaves",
    "Liquor/Wines",
    "Luxuries",
    "Narcotics",
    "Computers",
    "Machinery",
    "Alloys",
    "Firearms",
    "Furs",
    "Minerals",
    "Gold",
    "Platinum",
    "Gem-Stones",
    "Alien Items",
];

/// Единица измерения товара по индексу в таблице рынка.
fn unit(index: usize) -> &'static str {
    match index {
        13 | 14 => "kg",
        15 => "g",
        _ => "t",
    }
}

/// Раскладка экрана рынка (версия Кладова).
#[derive(Clone, Copy)]
pub struct MarketLayout {
    /// Адрес таблицы параметров генерации цен (17 x 5 байт).
    pub params: usize,
    /// Адрес таблицы цен (17 x u16 LE).
    pub prices: usize,
    /// Адрес таблицы остатков (17 x u8).
    pub quantities: usize,
    /// Адрес живого блока командира — деньги, груз и свободный тоннаж.
    pub pilot: u16,
}

/// Раскладка версии Кладова.
pub const KLADOV_MARKET: MarketLayout = MarketLayout {
    params: ADDR_TABLE_BASE_PRICE,
    prices: ADDR_PRODUCT_PRICE,
    quantities: ADDR_QUANTITY,
    pilot: PILOT,
};

/// Строка товара для отрисовки.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Item {
    /// Название товара.
    pub name: &'static str,
    /// Единица измерения (`t`, `kg`, `g`).
    pub unit: &'static str,
    /// Цена за единицу в десятых кредита.
    pub price_tenths: u16,
    /// Количество, доступное для покупки на станции.
    pub quantity: u8,
    /// Количество этого товара в грузовом отсеке корабля.
    pub in_hold: u8,
}

/// Снимок рынка для отрисовки.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Info {
    pub items: Vec<Item>,
    /// Деньги командира в десятых кредита (для подсказки на экране).
    pub cash_tenths: u32,
}

/// Состояние и отрисовка нативного экрана рынка.
pub struct Market {
    /// Включён ли нативный показ рынка (альтернатива оригинальному экрану `K`).
    pub native: bool,
    layout: MarketLayout,
    info: Info,
    /// Выбранная строка списка товаров (только на станции).
    cursor: usize,
    /// Результат последней сделки (или причина отказа).
    notice: Option<String>,
    /// Корабль на станции: в полёте экран только для просмотра цен, поэтому
    /// подсветка строки и подсказки торговли не рисуются.
    docked: bool,
}

impl Default for Market {
    fn default() -> Self {
        Self::new()
    }
}

impl Market {
    /// Новый выключенный экран.
    pub fn new() -> Self {
        Self {
            native: false,
            layout: KLADOV_MARKET,
            info: Info::default(),
            cursor: 0,
            notice: None,
            docked: false,
        }
    }

    /// Обновить данные рынка из памяти запущенной игры.
    pub fn refresh(&mut self, mem: &[u8], layout: MarketLayout) {
        self.layout = layout;
        self.info = read_info(mem, layout);
        self.cursor = self.cursor.min(self.info.items.len().saturating_sub(1));
        self.docked = flight::z80_is_docked(mem);
    }

    /// Включить нативный показ и прочитать текущие данные.
    pub fn enable(&mut self, mem: &[u8], layout: MarketLayout) {
        self.cursor = 0;
        self.notice = None;
        self.refresh(mem, layout);
        self.native = true;
    }

    /// Выключить нативный показ.
    pub fn disable(&mut self) {
        self.native = false;
    }

    /// Сдвинуть выбор по списку товаров стрелками вверх/вниз.
    pub fn move_selection(&mut self, delta: i32) {
        if self.info.items.is_empty() {
            return;
        }
        let last = self.info.items.len() - 1;
        self.cursor = (self.cursor as i32 + delta).clamp(0, last as i32) as usize;
    }

    /// Купить одну единицу выбранного товара (`→`).
    pub fn buy(&mut self, mem: &mut [u8]) {
        self.trade(mem, true);
    }

    /// Продать одну единицу выбранного товара (`←`).
    pub fn sell(&mut self, mem: &mut [u8]) {
        self.trade(mem, false);
    }

    /// Совершить сделку с выбранной строкой. Изменения пишутся прямо в живой
    /// блок командира, чтобы эмулятор сразу увидел новые деньги, остатки и груз.
    fn trade(&mut self, mem: &mut [u8], buy: bool) {
        if !flight::z80_is_docked(mem) {
            self.notice = Some("DOCK AT A STATION TO TRADE".into());
            return;
        }
        let Some(item) = self.info.items.get(self.cursor) else {
            return;
        };
        let i = self.cursor;
        let name = item.name;
        let pilot = self.layout.pilot as usize;
        // В оригинале (`Cmd_2_BuyProducts`) бит 5 PilotExtWeapon означает,
        // что в трюме десант/беженцы и торговать нельзя.
        if mem.get(pilot + OFF_EXT_WEAPON).copied().unwrap_or(0) & 0x20 != 0 {
            self.notice = Some("PASSENGERS ON BOARD - NO TRADE".into());
            return;
        }
        // (`sub_E34C`) золото, платина и драгоценные камни не занимают тоннаж.
        let tonnage = i < 13 || i == 16;
        let stock = self.layout.quantities + i;
        let cargo = pilot + OFF_CARGO + i;
        let free = pilot + OFF_FREE_CARGO;
        let price = read_u16(mem, self.layout.prices + i * 2) as u32;
        let mut cash = read_cash(mem, pilot);

        if buy {
            if mem.get(stock).copied().unwrap_or(0) == 0 {
                self.notice = Some("NONE FOR SALE".into());
                return;
            }
            if cash < price {
                self.notice = Some("NOT ENOUGH CASH".into());
                return;
            }
            if tonnage && mem.get(free).copied().unwrap_or(0) == 0 {
                self.notice = Some("CARGO HOLD FULL".into());
                return;
            }
            if mem.get(cargo).copied().unwrap_or(0) == u8::MAX {
                self.notice = Some("CARGO HOLD FULL".into());
                return;
            }
            mem[stock] -= 1;
            mem[cargo] += 1;
            cash -= price;
            if tonnage {
                mem[free] -= 1;
            }
            // (`sub_D1B5`) рабы и наркотики повышают юридический статус.
            if i == 3 || i == 6 {
                let legal = &mut mem[pilot + OFF_LEGAL];
                *legal = legal.saturating_add(2);
            }
            self.notice = Some(format!("BOUGHT 1 {}", name));
        } else {
            if mem.get(cargo).copied().unwrap_or(0) == 0 {
                self.notice = Some(format!("NO {} IN HOLD", name));
                return;
            }
            mem[cargo] -= 1;
            cash = cash.saturating_add(price);
            mem[stock] = mem[stock].saturating_add(1);
            if tonnage {
                mem[free] = mem[free].saturating_add(1);
            }
            self.notice = Some(format!("SOLD 1 {}", name));
        }
        write_cash(mem, pilot, cash);
        self.refresh(mem, self.layout);
    }

    /// Нарисовать экран во весь слой.
    pub fn draw(&self, hd: &mut Frame, font: &[u8; crate::font::ZX_FONT_LEN]) {
        draw::draw(hd, font, self);
    }
}

/// Прочитать данные рынка из памяти.
pub(crate) fn read_info(mem: &[u8], layout: MarketLayout) -> Info {
    let pilot = layout.pilot as usize;
    let mut items = Vec::with_capacity(GOODS_COUNT);
    for i in 0..GOODS_COUNT {
        let price_tenths = read_u16(mem, layout.prices + i * 2);
        let quantity = mem.get(layout.quantities + i).copied().unwrap_or(0);
        let in_hold = mem.get(pilot + OFF_CARGO + i).copied().unwrap_or(0);
        items.push(Item {
            name: GOODS[i],
            unit: unit(i),
            price_tenths,
            quantity,
            in_hold,
        });
    }
    Info {
        items,
        cash_tenths: read_cash(mem, pilot),
    }
}

/// Деньги из блока командира: `cash2*10000 + cash1` десятых кредита.
fn read_cash(mem: &[u8], pilot: usize) -> u32 {
    let cash2 = read_u16(mem, pilot + OFF_CASH2) as u32;
    let cash1 = read_u16(mem, pilot + OFF_CASH1) as u32;
    cash2 * 10000 + cash1
}

/// Записать деньги обратно, нормализуя перенос из `cash1` в `cash2`
/// (`IncCASH`/`sub_D3FD` в оригинале).
fn write_cash(mem: &mut [u8], pilot: usize, cash: u32) {
    let cash = cash.min(MAX_CASH_TENTHS);
    let cash2 = (cash / 10000) as u16;
    let cash1 = (cash % 10000) as u16;
    mem[pilot + OFF_CASH2..pilot + OFF_CASH2 + 2].copy_from_slice(&cash2.to_le_bytes());
    mem[pilot + OFF_CASH1..pilot + OFF_CASH1 + 2].copy_from_slice(&cash1.to_le_bytes());
}

fn read_u16(mem: &[u8], addr: usize) -> u16 {
    let lo = mem.get(addr).copied().unwrap_or(0) as u16;
    let hi = mem.get(addr + 1).copied().unwrap_or(0) as u16;
    lo | (hi << 8)
}
