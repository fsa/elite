//! Тесты экрана закупки.

use super::Shop;
use super::items::{FUEL_MAX, MISSILES_MAX, NAME_CHARS, PRICE_CHARS, TOAST_FRAMES};
use super::layout::{
    KLADOV_SHOP, OFF_CASH1, OFF_CASH2, OFF_FREE_CARGO, OFF_FUEL, OFF_LASERS, OFF_MISSION1,
    OFF_MISSILES, OFF_TECH, _ELITE_SHOP,
};
use super::render::{FITTED_FG, WARN_BG, fmt_fuel, wrap};
use crate::display::Frame;

/// Блок командира и данные системы версии Кладова — как в снапшоте
/// `zx_src/kladov.snapshot` (tech=10, cash=327986.8, ракеты 4, топливо 70,
/// грузовой отсек 0xFF, ECM 0x7F, IndDanger=0, система LAVE).
fn fake_kladov_mem() -> Vec<u8> {
    let mut m = vec![0u8; 0x10000];
    let p = KLADOV_SHOP.pilot as usize;
    // Имя командира (не используется экраном, но блок должен быть похож).
    m[p..p + 10].copy_from_slice(b"JAMESON   ");
    // cash2 = 327 (0x0147), cash1 = 9868 (0x268C) -> 327986.8 CR.
    m[p + OFF_CASH2..p + OFF_CASH2 + 2].copy_from_slice(&327u16.to_le_bytes());
    m[p + OFF_CASH1..p + OFF_CASH1 + 2].copy_from_slice(&9868u16.to_le_bytes());
    m[p + OFF_FREE_CARGO] = 0x14; // 20 тонн
    m[p + OFF_LASERS] = 1; // FRONT: pulse
    m[p + OFF_FUEL] = FUEL_MAX;
    m[p + OFF_MISSILES] = MISSILES_MAX;
    m[p + OFF_LASERS + 3] = 0; // RIGHT: пусто
    m[p + 48] = 0xFF; // большой грузовой отсек — установлен
    m[p + 49] = 0x7F; // E.C.M. — установлен (по-кладовски)
    m[p + OFF_TECH] = 10;
    // Таблица цен на 0x705F (14 записей по 3 байта, как в kladov.bin).
    let prices: [[u8; 3]; 14] = [
        [0x00, 0x00, 0x00], // FUEL: динамическая цена в среднем байте
        [0x00, 0x2C, 0x01], // 30.0
        [0x00, 0xA0, 0x0F], // 400.0
        [0x00, 0x70, 0x17], // 600.0
        [0x00, 0xA0, 0x0F], // 400.0 (pulse)
        [0x01, 0x00, 0x00], // 1000.0 (beam)
        [0x00, 0x82, 0x14], // 525.0
        [0x01, 0x00, 0x00], // 1000.0
        [0x00, 0x28, 0x23], // 900.0
        [0x01, 0x88, 0x13], // 1500.0
        [0x01, 0x88, 0x13], // 1500.0 (docking)
        [0x05, 0x00, 0x00], // 5000.0
        [0x06, 0x00, 0x00], // 6000.0 (military laser)
        [0x00, 0x40, 0x1F], // 800.0 (mining laser)
    ];
    let base = KLADOV_SHOP.prices as usize;
    for (i, rec) in prices.iter().enumerate() {
        m[base + i * 3..base + i * 3 + 3].copy_from_slice(rec);
    }
    m[base + 1] = 0x10; // динамическая цена топлива: 1.6 CR
    // Данные системы (LAVE) — из hdmap-тестов.
    let gl = KLADOV_SHOP.galaxy;
    m[gl.seed..gl.seed + 6].copy_from_slice(&[0x4A, 0x5A, 0x48, 0x02, 0x53, 0xB7]);
    m[gl.sys_y] = 173;
    m[gl.sys_x] = 20;
    m[gl.digraphs..gl.digraphs + 64]
        .copy_from_slice(b"ALLEXEGEZACEBISOUSESARMAINDIREA?ERATENBERALAVETIEDORQUANTEISRION");
    m[KLADOV_SHOP.in_danger as usize] = 0;
    m
}

fn opened() -> (Shop, Vec<u8>) {
    let mem = fake_kladov_mem();
    let mut shop = Shop::new();
    shop.open(&mem, KLADOV_SHOP);
    shop.win_w = 1280;
    shop.win_h = 720;
    (shop, mem)
}

fn cash_at(mem: &[u8], pilot: u16) -> u32 {
    let p = pilot as usize;
    let c2 = u16::from_le_bytes([mem[p + OFF_CASH2], mem[p + OFF_CASH2 + 1]]) as u32;
    let c1 = u16::from_le_bytes([mem[p + OFF_CASH1], mem[p + OFF_CASH1 + 1]]) as u32;
    c2 * 10000 + c1
}

fn cash_of(mem: &[u8]) -> u32 {
    cash_at(mem, KLADOV_SHOP.pilot)
}

/// Экран открывается по данным командира: список из tech+3 товаров,
/// установленное оборудование помечено, деньги и система — из памяти.
#[test]
fn open_reads_commander_and_lists_items() {
    let (shop, _mem) = opened();
    assert!(shop.active);
    // tech=10 -> 13 товаров: 1..13 (MINING LASER требует tech 11).
    assert_eq!(shop.rows, (1..=13).collect::<Vec<u8>>());
    assert_eq!(shop.cash, 327 * 10000 + 9868);
    assert_eq!(shop.system, "LAVE");
    assert!(shop.fitted[3], "грузовой отсек должен быть установлен");
    assert!(shop.fitted[4], "ECM должен быть установлен");
    assert!(!shop.fitted[11], "докинг-компьютер не установлен");
    assert_eq!(shop.missiles, MISSILES_MAX);
    assert!(!shop.in_flight);
}

/// Покупка докинг-компьютера (1500.0): флаг установлен, деньги списаны;
/// повторная покупка — «ALREADY FITTED» без списания.
#[test]
fn buy_docking_computer_writes_flag_and_cash() {
    let (mut shop, mut mem) = opened();
    let before = cash_of(&mem);
    shop.buy_number(&mut mem, 11);
    assert_eq!(mem[shop.flag_addr(11)], 0xFF, "флаг не записан");
    assert_eq!(cash_of(&mem), before - 15000);
    assert!(shop.fitted[11]);
    assert_eq!(shop.toast_text(), None);
    // Повторно — уже установлено, деньги не трогаем.
    shop.buy_number(&mut mem, 11);
    assert_eq!(
        shop.toast_text(),
        Some("DOCKING COMPUTER ALREADY FITTED")
    );
    assert_eq!(cash_of(&mem), before - 15000);
}

/// Ракет больше четырёх нельзя («ALL PRESENT» в оригинале) — без оплаты.
#[test]
fn missiles_full_refused_without_payment() {
    let (mut shop, mut mem) = opened();
    let before = cash_of(&mem);
    shop.buy_number(&mut mem, 2);
    assert_eq!(shop.toast_text(), Some("MISSILES FULL (4/4)"));
    assert_eq!(cash_of(&mem), before, "деньги не должны списаться");
    // Товар 2 — ракета: `pilot+45+2 = pilot+47` совпадает с PilotMissiles.
    assert_eq!(mem[shop.flag_addr(2)], MISSILES_MAX, "предусловие: 4 ракеты");
    // Мало ракет — покупка работает.
    mem[shop.pilot() + OFF_MISSILES] = 3;
    shop.refresh(&mem);
    shop.buy_number(&mut mem, 2);
    assert_eq!(mem[shop.pilot() + OFF_MISSILES], 4);
    assert_eq!(cash_of(&mem), before - 300, "МИССИЛА стоит 30.0");
}

/// Нехватка денег: тост и никаких изменений (оригинал при этом терял
/// деньги — см. комментарий к модулю).
#[test]
fn not_enough_cash_is_refused() {
    let (mut shop, mut mem) = opened();
    let p = shop.pilot();
    // Маленькие деньги: 50.0 CR.
    mem[p + OFF_CASH2..p + OFF_CASH2 + 2].copy_from_slice(&0u16.to_le_bytes());
    mem[p + OFF_CASH1..p + OFF_CASH1 + 2].copy_from_slice(&500u16.to_le_bytes());
    shop.refresh(&mem);
    shop.buy_number(&mut mem, 13); // MILITARY LASER, 6000.0
    assert_eq!(shop.toast_text(), Some("NOT ENOUGH CASH"));
    assert_eq!(cash_of(&mem), 500);
    assert_eq!(mem[shop.flag_addr(13)], 0, "лазер не должен записаться");
}

/// Заправка: топливо до 70, динамическая цена обнуляется, производная
/// переменная Fuel пересчитана (`2*fuel + fuel/4` = 157).
#[test]
fn fuel_purchase_refills_and_zeroes_price() {
    let (mut shop, mut mem) = opened();
    let p = shop.pilot();
    mem[p + OFF_FUEL] = 50;
    mem[KLADOV_SHOP.prices as usize + 1] = 0x10; // 1.6 CR
    shop.refresh(&mem);
    let before = cash_of(&mem);
    shop.buy_number(&mut mem, 1);
    assert_eq!(mem[p + OFF_FUEL], FUEL_MAX);
    assert_eq!(mem[KLADOV_SHOP.prices as usize + 1], 0, "цена не обнулена");
    assert_eq!(
        cash_of(&mem),
        before - 16,
        "списана динамическая цена 1.6 CR"
    );
    assert_eq!(mem[KLADOV_SHOP.fuel_derived as usize], 2 * 70 + 70 / 4);
    assert_eq!(shop.toast_text(), Some("FUEL TANK FULL"));
}

/// Лазер: крепление выбирается после покупки, старый лазер снимается и
/// его полная цена возвращается.
#[test]
fn laser_mount_replaces_and_refunds_old() {
    let (mut shop, mut mem) = opened();
    let p = shop.pilot();
    assert_eq!(mem[p + OFF_LASERS], 1, "предусловие: FRONT = pulse");
    let before = cash_of(&mem);
    shop.buy_number(&mut mem, 6); // EXTRA BEAM LASERS, 1000.0
    assert!(shop.mount.is_some(), "должно открыться окно крепления");
    assert_eq!(cash_of(&mem), before, "до выбора крепления не платим");
    shop.choose_mount(&mut mem, 0); // FRONT
    assert_eq!(mem[p + OFF_LASERS], 2, "слот не обновлён");
    // Заплатили 1000.0 за beam, вернули полную цену pulse (400.0).
    assert_eq!(cash_of(&mem), before - 10000 + 4000);
    assert_eq!(
        shop.toast_text(),
        Some("BEAM LASER MOUNTED FRONT")
    );
    // Пустое крепление: деньги списаны, возврата нет.
    let before = cash_of(&mem);
    shop.buy_number(&mut mem, 5); // PULSE, 400.0
    shop.choose_mount(&mut mem, 3); // RIGHT
    assert_eq!(mem[p + OFF_LASERS + 3], 1);
    assert_eq!(cash_of(&mem), before - 4000);
}

/// Отмена окна крепления не списывает денег.
#[test]
fn cancelled_mount_charges_nothing() {
    let (mut shop, mut mem) = opened();
    let before = cash_of(&mem);
    shop.buy_number(&mut mem, 5);
    assert!(shop.mount.is_some());
    shop.cancel_mount();
    assert!(shop.mount.is_none());
    assert_eq!(cash_of(&mem), before);
}

/// В полёте (IndDanger <> 0) экран открывается, но покупки запрещены.
#[test]
fn in_flight_blocks_purchases() {
    let (mut shop, mut mem) = opened();
    mem[KLADOV_SHOP.in_danger as usize] = 4;
    shop.refresh(&mem);
    let before = cash_of(&mem);
    shop.buy_number(&mut mem, 11);
    assert!(shop.in_flight);
    assert_eq!(
        shop.toast_text(),
        Some("DOCK AT A STATION TO BUY EQUIPMENT")
    );
    assert_eq!(cash_of(&mem), before);
    assert_eq!(mem[shop.flag_addr(11)], 0);
}

/// Мышь: наведение подсвечивает строку, клик покупает её.
#[test]
fn mouse_hover_and_click_buy_row() {
    let (mut shop, mut mem) = opened();
    let g = shop.geo();
    // Строка 0 (FUEL), центр.
    let y = g.list_y as f64 + g.row_h as f64 / 2.0;
    shop.point_at((g.x0 + 10) as f64, y);
    assert_eq!(shop.hover, Some(0));
    // Наводимся на докинг-компьютер (строка 10) и кликаем.
    let y = g.list_y as f64 + 10.5 * g.row_h as f64;
    shop.point_at((g.x0 + 10) as f64, y);
    assert_eq!(shop.hover, Some(10));
    let before = cash_of(&mem);
    shop.click(&mut mem);
    assert_eq!(cash_of(&mem), before - 15000);
    // Вне списка курсор не выставляется.
    shop.point_at(0.0, y);
    assert_eq!(shop.hover, None);
}

/// Клавиатура: две цифры выбирают товар (как `sub_E776`), Enter —
/// однозначный номер или выбранную строку.
#[test]
fn digits_and_enter_buy_items() {
    let (mut shop, mut mem) = opened();
    let before = cash_of(&mem);
    // «1» + «1» = товар 11 (DOCKING COMPUTER).
    shop.input_digit(&mut mem, 1);
    assert_eq!(shop.pending, Some(1));
    shop.input_digit(&mut mem, 1);
    assert_eq!(shop.pending, None);
    assert_eq!(cash_of(&mem), before - 15000);
    // «3» + Enter = товар 3 (грузовой отсек) — но он уже установлен.
    let before = cash_of(&mem);
    shop.input_digit(&mut mem, 3);
    shop.confirm(&mut mem);
    assert_eq!(cash_of(&mem), before);
    assert_eq!(
        shop.toast_text(),
        Some("LARGE CARGO BAY ALREADY FITTED")
    );
    // Выбранная строка + Enter: строка 0 = FUEL (цена 0x10 = 1.6).
    shop.move_selection(0);
    shop.confirm(&mut mem);
    assert_eq!(cash_of(&mem), before - 16);
    // Несуществующий номер — тост, без изменений.
    shop.input_digit(&mut mem, 1);
    shop.input_digit(&mut mem, 9);
    assert_eq!(shop.toast_text(), Some("NO SUCH ITEM"));
}

/// Раскладка Elite: при выданной 1-й миссии строка FUEL скрыта, цены
/// сдвинуты (баг оригинала), а покупка списывает правильную цену.
/// Проверка «установлено» — `flag != 0`.
#[test]
fn elite_layout_hides_fuel_and_shifts_prices_on_mission() {
    let mut mem = fake_kladov_mem();
    // Переносим данные в раскладку оригинала.
    let (kp, ep) = (
        KLADOV_SHOP.pilot as usize,
        _ELITE_SHOP.pilot as usize,
    );
    let commander = mem[kp..kp + 104].to_vec();
    mem[ep..ep + 104].copy_from_slice(&commander);
    let (kpr, epr) = (
        KLADOV_SHOP.prices as usize,
        _ELITE_SHOP.prices as usize,
    );
    let prices = mem[kpr..kpr + 42].to_vec();
    mem[epr..epr + 42].copy_from_slice(&prices);
    let gl = _ELITE_SHOP.galaxy;
    mem[gl.seed..gl.seed + 6].copy_from_slice(&[0x4A, 0x5A, 0x48, 0x02, 0x53, 0xB7]);
    mem[gl.sys_y] = 173;
    mem[gl.sys_x] = 20;
    mem[gl.digraphs..gl.digraphs + 64]
        .copy_from_slice(b"ALLEXEGEZACEBISOUSESARMAINDIREA?ERATENBERALAVETIEDORQUANTEISRION");
    mem[_ELITE_SHOP.in_danger as usize] = 0;
    // Выдана 1-я миссия — строка FUEL должна скрыться.
    mem[ep + OFF_MISSION1] = 1;
    let mut shop = Shop::new();
    shop.open(&mem, _ELITE_SHOP);
    shop.win_w = 1280;
    shop.win_h = 720;
    assert_eq!(shop.rows.first(), Some(&2), "FUEL должен быть скрыт");
    assert_eq!(shop.rows.last(), Some(&13));
    // Баг оригинала: цена у строки 2 показывается как цена записи 1
    // (динамическая цена топлива 1.6), хотя покупка стоит 30.0.
    assert_eq!(shop.price_shown(&mem, 2), 16);
    assert_eq!(shop.price(&mem, 2), 300);
    // Проверка «установлено» по-оригинальному: флаг 0x7F тоже установлен.
    mem[ep + 45 + 11] = 0x7F;
    shop.refresh(&mem);
    assert!(shop.fitted[11], "оригинал: любое flag != 0 = установлено");
    // Покупка пишет 0xFF (а не dec) и списывает правильную цену.
    mem[ep + 45 + 11] = 0;
    shop.refresh(&mem);
    let before = cash_at(&mem, _ELITE_SHOP.pilot);
    shop.buy_number(&mut mem, 11);
    assert_eq!(mem[ep + 45 + 11], 0xFF);
    assert_eq!(cash_at(&mem, _ELITE_SHOP.pilot), before - 15000);
}

/// Слой рисуется непрозрачно: есть шапка, строки, панель и помощь.
#[test]
fn draw_fills_layer() {
    let (mut shop, mem) = opened();
    let font = vec![0xFFu8; 0x300];
    let mut f = Frame::new(1280, 720);
    shop.draw(&mut f, &font, &mem);
    assert!(f.pixels().iter().all(|&p| p != 0), "слой не непрозрачный");
    let distinct: std::collections::HashSet<u32> =
        f.pixels().iter().copied().collect();
    // Палитра экрана: фон, панели, рамки, три оттенка текста, цена,
    // курсор, «установлено» — шрифт в тесте сплошной, поэтому цветов
    // ровно столько, сколько кистей (новых от глифов не появляется).
    assert!(distinct.len() >= 8, "слишком мало цветов: {}", distinct.len());
    // Крепление лазера FRONT = PULSE на панели корабля.
    assert!(f.pixels().iter().any(|&p| p == FITTED_FG));
    // Тост рисуется и гаснет через несколько кадров (ракеты уже полны —
    // покупка отклоняется с сообщением, память не меняется).
    shop.buy_number(&mut { fake_kladov_mem() }, 2);
    assert_eq!(shop.toast_text(), Some("MISSILES FULL (4/4)"));
    shop.draw(&mut f, &font, &mem);
    assert!(f.pixels().iter().any(|&p| p == WARN_BG), "нет панели тоста");
    for _ in 0..TOAST_FRAMES {
        shop.draw(&mut f, &font, &mem);
    }
    assert_eq!(shop.toast_text(), None, "тост не погас");
}

/// Раскладка: колонка цен не налезает на имена, панель корабля получает
/// заметную часть ширины, а топливо показывается в световых годах
/// (байт `70` = 7.0 св. года, как в оригинале).
#[test]
fn layout_has_room_for_prices_and_panel() {
    let (mut shop, _mem) = opened();
    shop.win_w = 1024;
    shop.win_h = 768;
    let g = shop.geo();
    let name_right = g.x_name + NAME_CHARS * g.ch;
    let price_left = g.x_price - PRICE_CHARS * g.ch;
    assert!(
        price_left >= name_right + g.ch,
        "цена налезает на название: {price_left} < {name_right}"
    );
    assert!(
        g.panel_w >= 20 * g.ch as usize,
        "панель слишком узкая: {}px",
        g.panel_w
    );
    assert_eq!(fmt_fuel(70), "7.0");
    assert_eq!(fmt_fuel(0), "0.0");
    assert_eq!(fmt_fuel(35), "3.5");
    assert!(wrap("survive ship destruction", 20).len() >= 2);
}

/// Отладочный дамп экрана в `tmp/equip.ppm` по данным образа Кладова:
/// `cargo test --release dump_equip -- --ignored`, затем, например,
/// `python3 -c '...'` или любой просмотрщик PPM.
#[test]
#[ignore]
fn dump_equip_to_tmp() {
    let base = env!("CARGO_MANIFEST_DIR");
    let rom = std::fs::read(format!("{base}/zx_bin/48.rom")).expect("zx_bin/48.rom");
    let img = std::fs::read(format!("{base}/zx_bin/kladov.bin")).expect("zx_bin/kladov.bin");
    let mut mem = vec![0u8; 0x10000];
    mem[..rom.len()].copy_from_slice(&rom);
    mem[0x4000..0x4000 + img.len()].copy_from_slice(&img);
    let mut shop = Shop::new();
    shop.open(&mem, KLADOV_SHOP);
    let font = crate::font::ZX_FONT;
    let mut f = Frame::new(1280, 720);
    shop.draw(&mut f, font, &mem);
    let mut out = Vec::with_capacity(f.width * f.height * 3 + 32);
    out.extend_from_slice(format!("P6\n{} {}\n255\n", f.width, f.height).as_bytes());
    for &p in f.pixels() {
        out.push((p >> 16) as u8);
        out.push((p >> 8) as u8);
        out.push(p as u8);
    }
    std::fs::write("tmp/equip.ppm", out).expect("write tmp/equip.ppm");
}
