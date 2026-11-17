//! Нативный экран цен на станции — `F8` (альтернатива оригинальной `K`).
//!
//! Показывает текущие цены на товары станции, прочитанные из памяти Z80.
//! Оформление в стиле экранов информации о планете и командира.
//!
//! Цены пересчитываются самим Z80 в `InitGame`/`GenerateAnotherPrice` при
//! пристыковке: параметры из `TableBasePrice` (17 x 5 байт) комбинируются с
//! `PlanetEconomy` и `PilotRnd`, результат пишется в `ProductPrice`. Остатки
//! лежат отдельно, сразу за `PilotPlanetGov`. Нативный экран не считает цены
//! сам, а читает готовые таблицы — поэтому он всегда совпадает с эмулятором.

use crate::display::Frame;

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
}

/// Раскладка версии Кладова.
pub const KLADOV_MARKET: MarketLayout = MarketLayout {
    params: ADDR_TABLE_BASE_PRICE,
    prices: ADDR_PRODUCT_PRICE,
    quantities: ADDR_QUANTITY,
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
}

/// Снимок рынка для отрисовки.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Info {
    pub items: Vec<Item>,
}

/// Состояние и отрисовка нативного экрана рынка.
pub struct Market {
    /// Включён ли нативный показ рынка (альтернатива оригинальному экрану `K`).
    pub native: bool,
    layout: MarketLayout,
    info: Info,
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
        }
    }

    /// Обновить данные рынка из памяти запущенной игры.
    pub fn refresh(&mut self, mem: &[u8], layout: MarketLayout) {
        self.layout = layout;
        self.info = read_info(mem, layout);
    }

    /// Включить нативный показ и прочитать текущие данные.
    pub fn enable(&mut self, mem: &[u8], layout: MarketLayout) {
        self.refresh(mem, layout);
        self.native = true;
    }

    /// Выключить нативный показ.
    pub fn disable(&mut self) {
        self.native = false;
    }

    /// Нарисовать экран во весь слой.
    pub fn draw(&self, hd: &mut Frame, font: &[u8; crate::font::ZX_FONT_LEN]) {
        draw::draw(hd, font, &self.info);
    }
}

/// Прочитать данные рынка из памяти.
pub(crate) fn read_info(mem: &[u8], layout: MarketLayout) -> Info {
    let mut items = Vec::with_capacity(GOODS_COUNT);
    for i in 0..GOODS_COUNT {
        let price_tenths = read_u16(mem, layout.prices + i * 2);
        let quantity = mem.get(layout.quantities + i).copied().unwrap_or(0);
        items.push(Item {
            name: GOODS[i],
            unit: unit(i),
            price_tenths,
            quantity,
        });
    }
    Info { items }
}

fn read_u16(mem: &[u8], addr: usize) -> u16 {
    let lo = mem.get(addr).copied().unwrap_or(0) as u16;
    let hi = mem.get(addr + 1).copied().unwrap_or(0) as u16;
    lo | (hi << 8)
}
