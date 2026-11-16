//! Нативный экран цен на станции — клавиша `K` (рынок).
//!
//! Показывает текущие цены на товары станции, прочитанные из памяти Z80.
//! Оформление в стиле экранов информации о планете и командира.

use crate::display::Frame;

mod draw;
#[cfg(test)]
mod tests;

/// Адрес таблицы базовых параметров/остатков товаров в памяти Z80 (версия Кладова).
/// 17 записей по 5 байт: параметры цены/экономики и количество на продажу;
/// сразу за таблицей следует `ProductPrice`.
const ADDR_TABLE_BASE_PRICE: usize = 0x70B9;
/// Адрес таблицы цен товаров в памяти Z80 (версия Кладова).
/// 17 записей по 2 байта (LE), цена в десятых кредита.
const ADDR_PRODUCT_PRICE: usize = 0x710E;
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
    /// Адрес таблицы базовых параметров/остатков (17 x 5 байт).
    pub table_base: usize,
    /// Адрес таблицы цен (17 x u16 LE).
    pub prices: usize,
}

/// Раскладка версии Кладова.
pub const KLADOV_MARKET: MarketLayout = MarketLayout {
    table_base: ADDR_TABLE_BASE_PRICE,
    prices: ADDR_PRODUCT_PRICE,
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
        let quantity = mem
            .get(layout.table_base + i * 5 + 4)
            .copied()
            .unwrap_or(0);
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
