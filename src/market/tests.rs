use super::*;

#[test]
fn read_info_reads_prices_quantity_and_unit() {
    let mut mem = vec![0u8; 0x8000];
    // item 0 (Food, t): price 0x0018 -> 2.4, quantity 3
    mem[ADDR_PRODUCT_PRICE] = 0x18;
    mem[ADDR_PRODUCT_PRICE + 1] = 0x00;
    mem[ADDR_TABLE_BASE_PRICE + 4] = 3;
    // item 13 (Gold, kg): quantity 5
    mem[ADDR_TABLE_BASE_PRICE + 13 * 5 + 4] = 5;
    // item 15 (Gem-Stones, g): quantity 0
    mem[ADDR_TABLE_BASE_PRICE + 15 * 5 + 4] = 0;
    // item 16 (Alien Items, t): price 0x0200 -> 51.2, quantity 1
    let last = GOODS_COUNT - 1;
    mem[ADDR_PRODUCT_PRICE + last * 2] = 0x00;
    mem[ADDR_PRODUCT_PRICE + last * 2 + 1] = 0x02;
    mem[ADDR_TABLE_BASE_PRICE + last * 5 + 4] = 1;

    let info = read_info(&mem, KLADOV_MARKET);
    assert_eq!(info.items.len(), GOODS_COUNT);
    assert_eq!(info.items[0].name, "Food");
    assert_eq!(info.items[0].unit, "t");
    assert_eq!(info.items[0].price_tenths, 0x0018);
    assert_eq!(info.items[0].quantity, 3);
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
}


