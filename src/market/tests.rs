use super::*;
use crate::flight;

fn set_cash(mem: &mut [u8], tenths: u32) {
    write_cash(mem, PILOT as usize, tenths);
}

fn cash(mem: &[u8]) -> u32 {
    read_cash(mem, PILOT as usize)
}

fn set_price(mem: &mut [u8], i: usize, tenths: u16) {
    let a = ADDR_PRODUCT_PRICE + i * 2;
    mem[a] = tenths as u8;
    mem[a + 1] = (tenths >> 8) as u8;
}

fn price(mem: &[u8], i: usize) -> u16 {
    read_u16(mem, ADDR_PRODUCT_PRICE + i * 2)
}

#[test]
fn market_title_has_system_name_in_caps() {
    assert_eq!(draw::title("Lave"), "LAVE MARKET PRICES");
    assert_eq!(draw::title(""), "MARKET PRICES");
}

#[test]
fn market_rows_are_spaced_like_f7_and_f9() {
    let l = draw::layout(1024, 768);
    assert_eq!(
        l.header_bottom_y,
        l.header_top_y + l.line,
        "строки заголовка колонок остаются плотными"
    );
    assert_eq!(
        l.first_row_y,
        l.header_bottom_y + l.row_h,
        "отступ от заголовка до первой строки равен шагу строк"
    );
    assert_eq!(
        l.row_h,
        l.line + l.line / 2,
        "шаг строк таблицы такой же, как на F7/F9"
    );
}

#[test]
fn market_status_is_a_single_hint_line() {
    assert_eq!(
        draw::status_line(1000, true),
        "Cash: 100.0 Cr  Select: Up/Down  Sell: Left  Buy: Right"
    );
    assert_eq!(draw::status_line(1000, false), "Cash: 100.0 Cr");
    assert_eq!(
        draw::status_line(655_359_999, true),
        "Cash: 65535999.9 Cr  Select: Up/Down  Sell: Left  Buy: Right"
    );
}

#[test]
fn market_status_fields_split_name_and_value_for_colors() {
    let fields = draw::status_fields(1000, true);
    let parts: Vec<_> = fields
        .iter()
        .map(|f| (f.name, f.value.as_str()))
        .collect();
    assert_eq!(
        parts,
        vec![
            ("Cash", "100.0 Cr"),
            ("Select", "Up/Down"),
            ("Sell", "Left"),
            ("Buy", "Right"),
        ]
    );
}

#[test]
fn read_info_reads_prices_quantity_and_unit() {
    let mut mem = vec![0u8; 0x10000];
    // item 0 (Food, t): price 0x0018 -> 2.4, quantity 3
    set_price(&mut mem, 0, 0x0018);
    mem[ADDR_QUANTITY] = 3;
    // item 13 (Gold, kg): quantity 5
    mem[ADDR_QUANTITY + 13] = 5;
    // item 15 (Gem-Stones, g): quantity 0
    mem[ADDR_QUANTITY + 15] = 0;
    // item 16 (Alien Items, t): price 0x0200 -> 51.2, quantity 1
    let last = GOODS_COUNT - 1;
    set_price(&mut mem, last, 0x0200);
    mem[ADDR_QUANTITY + last] = 1;
    // Груз и деньги из живого блока командира.
    mem[PILOT as usize + OFF_CARGO] = 7;
    mem[PILOT as usize + OFF_CARGO + last] = 2;
    set_cash(&mut mem, 1234);

    let info = read_info(&mem, KLADOV_MARKET);
    assert_eq!(info.items.len(), GOODS_COUNT);
    assert_eq!(info.items[0].name, "Food");
    assert_eq!(info.items[0].unit, "t");
    assert_eq!(info.items[0].price_tenths, 0x0018);
    assert_eq!(info.items[0].quantity, 3);
    assert_eq!(info.items[0].in_hold, 7);
    assert_eq!(info.items[13].name, "Gold");
    assert_eq!(info.items[13].unit, "kg");
    assert_eq!(info.items[13].quantity, 5);
    assert_eq!(info.items[15].name, "Gem-Stones");
    assert_eq!(info.items[15].unit, "g");
    assert_eq!(info.items[15].quantity, 0);
    assert_eq!(info.items[last].name, "Alien Items");
    assert_eq!(info.items[last].unit, "t");
    assert_eq!(info.items[last].price_tenths, 0x0200);
    assert_eq!(info.items[last].quantity, 1);
    assert_eq!(info.items[last].in_hold, 2);
    assert_eq!(info.cash_tenths, 1234);
}

#[test]
fn buy_reduces_stock_cash_and_free_cargo_and_fills_hold() {
    let mut mem = vec![0u8; 0x10000];
    set_price(&mut mem, 0, 36); // 3.6 кредита
    mem[ADDR_QUANTITY] = 5;
    mem[PILOT as usize + OFF_FREE_CARGO] = 10;
    set_cash(&mut mem, 100);

    let mut market = Market::new();
    market.enable(&mem, KLADOV_MARKET, true, "Lave");
    market.buy(&mut mem, true);

    assert_eq!(mem[ADDR_QUANTITY], 4);
    assert_eq!(mem[PILOT as usize + OFF_CARGO], 1);
    assert_eq!(mem[PILOT as usize + OFF_FREE_CARGO], 9);
    assert_eq!(cash(&mem), 100 - 36);
    assert_eq!(price(&mem, 0), 36);
    assert_eq!(market.info.items[0].quantity, 4);
    assert_eq!(market.info.items[0].in_hold, 1);
    assert_eq!(market.info.cash_tenths, 100 - 36);
}

#[test]
fn sell_returns_cargo_space_and_money() {
    let mut mem = vec![0u8; 0x10000];
    set_price(&mut mem, 0, 36);
    mem[ADDR_QUANTITY] = 5;
    mem[PILOT as usize + OFF_CARGO] = 2;
    mem[PILOT as usize + OFF_FREE_CARGO] = 8;
    set_cash(&mut mem, 100);

    let mut market = Market::new();
    market.enable(&mem, KLADOV_MARKET, true, "Lave");
    market.sell(&mut mem, true);

    assert_eq!(mem[ADDR_QUANTITY], 6);
    assert_eq!(mem[PILOT as usize + OFF_CARGO], 1);
    assert_eq!(mem[PILOT as usize + OFF_FREE_CARGO], 9);
    assert_eq!(cash(&mem), 100 + 36);
    assert_eq!(market.info.items[0].in_hold, 1);
}

#[test]
fn trade_checks_stock_cash_and_hold_space() {
    let mut mem = vec![0u8; 0x10000];
    set_price(&mut mem, 0, 50);
    mem[ADDR_QUANTITY] = 1;
    mem[PILOT as usize + OFF_FREE_CARGO] = 3;
    set_cash(&mut mem, 100);

    let mut market = Market::new();
    market.enable(&mem, KLADOV_MARKET, true, "Lave");

    // Первая покупка опустошает склад.
    market.buy(&mut mem, true);
    assert_eq!(mem[ADDR_QUANTITY], 0);
    assert_eq!(mem[PILOT as usize + OFF_CARGO], 1);
    assert_eq!(cash(&mem), 50);

    // Повтор при пустом складе ничего не меняет.
    market.buy(&mut mem, true);
    assert_eq!(mem[PILOT as usize + OFF_CARGO], 1);
    assert_eq!(cash(&mem), 50);
    assert!(market.notice.as_deref().unwrap().contains("NONE FOR SALE"));

    // Не хватает денег.
    mem[ADDR_QUANTITY] = 1;
    set_cash(&mut mem, 10);
    market.buy(&mut mem, true);
    assert_eq!(mem[ADDR_QUANTITY], 1);
    assert_eq!(cash(&mem), 10);
    assert!(market.notice.as_deref().unwrap().contains("NOT ENOUGH CASH"));

    // Трюм полон.
    set_cash(&mut mem, 100);
    mem[PILOT as usize + OFF_FREE_CARGO] = 0;
    market.buy(&mut mem, true);
    assert_eq!(mem[ADDR_QUANTITY], 1);
    assert_eq!(cash(&mem), 100);
    assert!(market.notice.as_deref().unwrap().contains("CARGO HOLD FULL"));

    // Нечего продавать.
    mem[PILOT as usize + OFF_CARGO] = 0;
    market.sell(&mut mem, true);
    assert_eq!(cash(&mem), 100);
    assert!(market.notice.as_deref().unwrap().contains("IN HOLD"));
}

#[test]
fn precious_metals_do_not_use_cargo_space() {
    let mut mem = vec![0u8; 0x10000];
    set_price(&mut mem, 13, 500);
    mem[ADDR_QUANTITY + 13] = 2;
    mem[PILOT as usize + OFF_FREE_CARGO] = 0;
    set_cash(&mut mem, 1000);

    let mut market = Market::new();
    market.enable(&mem, KLADOV_MARKET, true, "Lave");
    market.move_selection(13);
    market.buy(&mut mem, true);

    assert_eq!(mem[ADDR_QUANTITY + 13], 1);
    assert_eq!(mem[PILOT as usize + OFF_CARGO + 13], 1);
    assert_eq!(mem[PILOT as usize + OFF_FREE_CARGO], 0);
    assert_eq!(cash(&mem), 1000 - 500);
}

#[test]
fn trade_is_refused_outside_a_station() {
    let mut mem = vec![0u8; 0x10000];
    // IndDanger <> 0 — корабль в полёте.
    mem[flight::Z80_IND_DANGER] = 1;
    set_price(&mut mem, 0, 40);
    mem[ADDR_QUANTITY] = 2;
    mem[PILOT as usize + OFF_CARGO] = 1;
    mem[PILOT as usize + OFF_FREE_CARGO] = 5;
    set_cash(&mut mem, 500);

    let mut market = Market::new();
    market.enable(&mem, KLADOV_MARKET, false, "Lave");
    market.buy(&mut mem, false);
    assert_eq!(mem[ADDR_QUANTITY], 2);
    assert_eq!(mem[PILOT as usize + OFF_CARGO], 1);
    assert_eq!(cash(&mem), 500);
    assert!(market.notice.as_deref().unwrap().contains("DOCK"));

    market.sell(&mut mem, false);
    assert_eq!(mem[ADDR_QUANTITY], 2);
    assert_eq!(mem[PILOT as usize + OFF_CARGO], 1);
    assert_eq!(cash(&mem), 500);
    assert!(market.notice.as_deref().unwrap().contains("DOCK"));
}

#[test]
fn illegal_goods_raise_legal_status_and_passengers_block_trade() {
    let mut mem = vec![0u8; 0x10000];
    set_price(&mut mem, 3, 100);
    mem[ADDR_QUANTITY + 3] = 1;
    mem[PILOT as usize + OFF_FREE_CARGO] = 5;
    set_cash(&mut mem, 200);

    let mut market = Market::new();
    market.enable(&mem, KLADOV_MARKET, true, "Lave");
    market.move_selection(3);
    market.buy(&mut mem, true);
    assert_eq!(mem[PILOT as usize + OFF_LEGAL], 2);
    assert_eq!(mem[PILOT as usize + OFF_CARGO + 3], 1);

    // Десант в трюме: торговля запрещена.
    mem[PILOT as usize + OFF_EXT_WEAPON] = 0x20;
    mem[ADDR_QUANTITY + 3] = 1;
    set_cash(&mut mem, 200);
    market.sell(&mut mem, true);
    assert_eq!(mem[ADDR_QUANTITY + 3], 1);
    assert_eq!(cash(&mem), 200);
    assert!(market.notice.as_deref().unwrap().contains("NO TRADE"));
}

/// В полёте экран — только таблица цен: нет подсветки выбранной строки,
/// подсказок выбора/торговли и сообщений о сделках.
#[test]
fn draw_in_flight_has_no_selection_or_trade_hints() {
    let mut mem = vec![0u8; 0x10000];
    set_price(&mut mem, 0, 40);

    let mut docked = Market::new();
    docked.enable(&mem, KLADOV_MARKET, true, "Lave");
    assert!(docked.docked);
    docked.move_selection(1);

    mem[flight::Z80_IND_DANGER] = 1;
    let mut flying = Market::new();
    flying.enable(&mem, KLADOV_MARKET, false, "Lave");
    assert!(!flying.docked);
    flying.move_selection(1);

    // Шрифт со всеми включёнными пикселями: цвет текста легко отличить от фона.
    let font = [0xFFu8; crate::font::ZX_FONT_LEN];
    let mut a = Frame::new(1024, 768);
    let mut b = Frame::new(1024, 768);
    docked.draw(&mut a, &font);
    flying.draw(&mut b, &font);
    let docked_px = a.pixels();
    let flying_px = b.pixels();

    // Подсветка строки (`SELECT_BG`) осталась только на станции.
    assert!(docked_px.iter().any(|&p| p == 0xFF00_5458));
    assert!(!flying_px.iter().any(|&p| p == 0xFF00_5458));

    // Подсказка торговли написана тем же голубым, что и подписи колонок;
    // в полёте таких пикселей меньше.
    let label = 0xFF00_D7D7;
    let docked_labels = docked_px.iter().filter(|&&p| p == label).count();
    let flying_labels = flying_px.iter().filter(|&&p| p == label).count();
    assert!(docked_labels > flying_labels);
}
