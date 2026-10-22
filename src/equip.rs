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
//! Логика покупок соответствует обоим вариантам игры (различия — только в
//! проверке «уже установлено», см. [`ShopLayout::dec_flags`]):
//!
//! - цены — таблица из 14 трёхбайтовых записей ([`ShopLayout::prices`):
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
//! Баг отображения оригинала сохранён: в версии Elite строка FUEL скрывается,
//! когда выдана 1-я миссия (`PilotMission1 != 0`), но указатель цены при этом
//! **не сдвигается** — все строки ниже показывают цену соседнего товара
//! (`Cmd_4_BuyEquip`: `word_B73B` увеличивается только при печати строки).
//! Список рисуется «как в оригинале», а покупка списывает правильную цену
//! (оригинал тоже берёт её из таблицы по индексу, `sub_B70A`).

use crate::display::Frame;
use crate::galaxy::{self, GalaxyLayout};

/// Товар списка закупки (номер 1..14).
struct Item {
    /// Оригинальное название (токены `0x69..0x76`).
    name: &'static str,
    /// Короткая подсказка (современное дополнение к оригинальному списку).
    desc: &'static str,
    kind: Kind,
}

/// Тип товара — определяет поведение покупки.
#[derive(Clone, Copy, PartialEq)]
enum Kind {
    /// Топливо: заправка до 70, обнуление динамической цены.
    Fuel,
    /// Ракета: +1, максимум 4.
    Missile,
    /// Большой грузовой отсек: +15 тонн.
    Bay,
    /// Обычный флаг в блоке командира.
    Flag,
    /// Лазер указанного типа (1..4) — с выбором крепления.
    Laser(u8),
}

/// Список товаров в порядке оригинала (как в `Cmd_4_EquipShip`).
const ITEMS: [Item; 14] = [
    Item { name: "FUEL", desc: "refill the fuel tank", kind: Kind::Fuel },
    Item { name: "MISSILE", desc: "one more missile (max 4)", kind: Kind::Missile },
    Item { name: "LARGE CARGO BAY", desc: "+15 tons of cargo space", kind: Kind::Bay },
    Item { name: "E.C.M.SYSTEM", desc: "defends against missiles", kind: Kind::Flag },
    Item { name: "EXTRA PULSE LASERS", desc: "cheap lasers", kind: Kind::Laser(1) },
    Item { name: "EXTRA BEAM LASERS", desc: "fast lasers", kind: Kind::Laser(2) },
    Item { name: "FUEL SCOOPS", desc: "collect floating cargo", kind: Kind::Flag },
    Item { name: "ESCAPE POD", desc: "survive ship destruction", kind: Kind::Flag },
    Item { name: "ENERGY BOMB", desc: "dump all energy at once", kind: Kind::Flag },
    Item { name: "ENERGY UNIT", desc: "recharge energy faster", kind: Kind::Flag },
    Item { name: "DOCKING COMPUTER", desc: "automatic docking", kind: Kind::Flag },
    Item { name: "GALACTIC HYPERSPACE", desc: "jump to another galaxy", kind: Kind::Flag },
    Item { name: "MILITARY LASER", desc: "powerful lasers", kind: Kind::Laser(3) },
    Item { name: "MINING LASER", desc: "lasers for asteroid mining", kind: Kind::Laser(4) },
];

/// Крепления лазера: `pilot+41+индекс` (PilotFrontLaser и далее).
const MOUNTS: [&str; 4] = ["FRONT", "REAR", "LEFT", "RIGHT"];
/// Названия типов лазеров (0 — нет лазера).
const LASER_NAMES: [&str; 5] = ["-", "PULSE", "BEAM", "MILITARY", "MINING"];

/// Название типа лазера из байта памяти (в повреждённом сейве может быть
/// что угодно — не паникуем).
fn laser_name(t: u8) -> &'static str {
    LASER_NAMES.get(t as usize).copied().unwrap_or("?")
}
/// Товары-лазеры по типу лазера (1..4) — из них оригиналы возвращают цену
/// при замене (`loc_FD87+2` = `{5, 6, 13, 14}`).
const LASER_ITEMS: [u8; 5] = [0, 5, 6, 13, 14];

/// Максимум товаров в списке.
const MAX_ITEMS: usize = 14;
/// Полный бак (SetPilotFuel: `ld a,46h; call SetPilotFuel`).
const FUEL_MAX: u8 = 70;
/// Максимум ракет (`cp 4; jp nc`).
const MISSILES_MAX: u8 = 4;
/// Тонн добавляет большой грузовой отсек (`add a,0Fh`).
const BAY_TONS: u8 = 15;
/// Ширина колонки названий: самое длинное — «GALACTIC HYPERSPACE» (19).
const NAME_CHARS: i32 = 19;
/// Ширина колонки цены: влезает и «6000.0», и «FITTED».
const PRICE_CHARS: i32 = 7;
/// Ширина списка в символах: рамка + номер 2 + зазор + имя 19 + зазор +
/// цена 7 + отступ (см. [`Shop::geo`]).
const LIST_CHARS: i32 = 1 + 2 + 1 + NAME_CHARS + 1 + PRICE_CHARS + 1;
/// Минимум символов, оставляемых панели корабля (описания переносятся).
const PANEL_MIN_CHARS: i32 = 18;
/// Сколько кадров показывается тост (~5 с при 50 к/с).
const TOAST_FRAMES: u32 = 250;

// ————— Смещения внутри блока командира (одинаковы в обеих версиях) —————

/// `PilotCash2low`: u16 LE, «тысячи» десятых (цена товара += `b0` сюда).
const OFF_CASH2: usize = 18;
/// `PilotCash1`: u16 LE, «десятые» (0..9999, переполнение уходит в cash2).
const OFF_CASH1: usize = 20;
/// `PilotFreeCargo`: свободные тонны трюма.
const OFF_FREE_CARGO: usize = 40;
/// `PilotFrontLaser`: крепления лазеров FRONT/REAR/LEFT/RIGHT.
const OFF_LASERS: usize = 41;
/// `PilotMission1`: <>0 выдана 1-я миссия.
const OFF_MISSION1: usize = 45;
/// `Pilot_Fuel`.
const OFF_FUEL: usize = 46;
/// `PilotMissiles`.
const OFF_MISSILES: usize = 47;
/// `PilotMineralLife` (только у Кладова: конвертация миссионного отсека).
const OFF_MINERAL: usize = 51;
/// Флаги товаров: номер товара `n` -> `pilot + 45 + n` (`sub_EB38`/`sub_B713`).
const FLAG_BASE: usize = 45;
/// `byte_D08E` (у Кладова) — «<>0 есть Cobra Mk4», в проверке «установлено».
const OFF_COBRA: usize = 58;
/// `PilotPlanetTechLvl`.
const OFF_TECH: usize = 72;

/// Раскладка данных экрана закупки (адреса зависят от версии игры).
#[derive(Clone, Copy)]
pub struct ShopLayout {
    /// Адрес живого блока командира (`PilotName`).
    pub pilot: u16,
    /// База таблицы цен (14 записей по 3 байта).
    pub prices: u16,
    /// `IndDanger`: <>0 — в полёте, покупки запрещены.
    pub in_danger: u16,
    /// Производная переменная `Fuel = 2*fuel + fuel/4` (дальность прыжка).
    pub fuel_derived: u16,
    /// Прятать ли строку FUEL при выданной 1-й миссии (баг отображения
    /// оригинала Elite, см. комментарий в начале файла).
    pub hide_fuel_on_mission: bool,
    /// Проверка/отметка «установлено» по-кладовски: покупка делает
    /// `flag -= 1`, «установлено» = `(flag|0x80)+1 == byte_D08E` (при
    /// `byte_D08E == 0` — `flag ∈ {0x7F, 0xFF}`). В оригинале покупка пишет
    /// `0xFF`, а «установлено» = `flag != 0` (`sub_B616`).
    pub dec_flags: bool,
    /// Раскладка галактики — имя текущей системы в шапке.
    pub galaxy: GalaxyLayout,
}

/// Раскладка версии Кладова (`zx_bin/kladov.bin`).
pub const KLADOV_SHOP: ShopLayout = ShopLayout {
    pilot: 0xD054,
    prices: 0x705F,
    in_danger: 0x7104,
    fuel_derived: 0x70EC,
    hide_fuel_on_mission: false,
    dec_flags: true,
    galaxy: galaxy::KLADOV_GALAXY,
};

/// Раскладка оригинальной версии (`zx_bin/elite.bin`).
pub const ELITE_SHOP: ShopLayout = ShopLayout {
    pilot: 0xA816,
    prices: 0x9236,
    in_danger: 0x7062,
    fuel_derived: 0x704A,
    hide_fuel_on_mission: true,
    dec_flags: false,
    galaxy: galaxy::ELITE_GALAXY,
};

// ————— Цвета HD-слоя (непрозрачные, `0xFFRRGGBB`) —————
const BG: u32 = 0xFF00_0008;
const LIST_BG: u32 = 0xFF0A_1218;
const PANEL_BG: u32 = 0xFF08_0E14;
const BORDER: u32 = 0xFF30_6070;
const TEXT: u32 = 0xFFE0_E8F0;
const TEXT_DIM: u32 = 0xFF80_9098;
const HOVER_BG: u32 = 0xFF14_2838;
const CURSOR_BG: u32 = 0xFF1E_4660;
const FITTED_FG: u32 = 0xFF50_C080;
const PRICE_FG: u32 = 0xFFD0_C060;
const WARN_BG: u32 = 0xFF50_1810;
const WARN_FG: u32 = 0xFFFF_C0A0;

/// «327986.8» — десятые без десятичной мусорины.
fn fmt_tenths(t: u32) -> String {
    format!("{}.{}", t / 10, t % 10)
}

/// Топливо в десятых долях светового года: `70` → «7.0» (как в игре,
/// где `Pilot_Fuel` хранит `0x46` = 7.0 св. года).
fn fmt_fuel(tenths: u8) -> String {
    format!("{}.{}", tenths / 10, tenths % 10)
}

/// Разбить текст на строки не длиннее `cols` символов по словам.
fn wrap(text: &str, cols: usize) -> Vec<String> {
    let cols = cols.max(1);
    let mut out = Vec::new();
    let mut line = String::new();
    for word in text.split_whitespace() {
        if !line.is_empty() && line.chars().count() + 1 + word.chars().count() > cols {
            out.push(std::mem::take(&mut line));
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() {
        out.push(line);
    }
    if out.is_empty() {
        out.push(String::new());
    }
    out
}

/// Геометрия окна для отрисовки и попадания мышью (считается из размера
/// окна и числа строк списка).
#[derive(Clone, Copy)]
struct Geo {
    /// Масштаб шрифта.
    ts: usize,
    /// Ширина символа 8x8, увеличенного в `ts` раз.
    ch: i32,
    /// Высота строки текста (= `ch`).
    line: i32,
    /// Левое поле содержимого.
    x0: i32,
    /// Низ верхней полосы (шапка).
    top: i32,
    /// Верх строк списка.
    list_y: i32,
    /// Высота строки списка.
    row_h: i32,
    /// Ширина панели списка.
    list_w: usize,
    x_num: i32,
    x_name: i32,
    /// Правая граница колонки цены (цена/«FITTED» выравниваются вправо).
    x_price: i32,
    /// Ширина колонки цены в символах.
    price_w: i32,
    /// Панель корабля справа.
    panel_x: i32,
    panel_w: usize,
    /// Строка помощи внизу.
    help_y: i32,
    /// Низ области содержимого.
    content_bottom: i32,
}

/// Прямоугольник `(x, y, w, h)` — для заливки и проверки попадания.
type Rect = (i32, i32, usize, usize);

fn inside(r: Rect, x: f64, y: f64) -> bool {
    x >= r.0 as f64
        && x < (r.0 + r.2 as i32) as f64
        && y >= r.1 as f64
        && y < (r.1 + r.3 as i32) as f64
}

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

    fn pilot(&self) -> usize {
        self.layout.pilot as usize
    }

    /// Прочитать состояние командира и построить список товаров.
    fn refresh(&mut self, mem: &[u8]) {
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
        // Баг оригинала Elite: строка FUEL не печатается при выданной миссии.
        if self.layout.hide_fuel_on_mission && mem[p + OFF_MISSION1] != 0 {
            self.rows.retain(|&x| x != 1);
        }
        for n in 1..=MAX_ITEMS as u8 {
            self.fitted[n as usize] = self.flag_fitted(mem, n);
        }
        self.cursor = self.cursor.min(self.rows.len().saturating_sub(1));
        // Наведение переживает перерисовку (после клика список обновляется,
        // но указатель остаётся на той же строке).
        self.hover = self.hover.filter(|&i| i < self.rows.len());
    }

    /// Ссылка на байт флага товара `number` (1..12) в блоке командира.
    fn flag_addr(&self, number: u8) -> usize {
        self.pilot() + FLAG_BASE + number as usize
    }

    /// «Установлено» ли оборудование товара `number` (только флаговые).
    ///
    /// У Кладова (`sub_EC4B`): `(flag|0x80)+1 == byte_D08E`; для грузового
    /// отсека `sub_6FFB` сравнивает сырое значение (`flag+1 == byte_D08E`),
    /// и отдельно обрабатывает миссионную метку `0x59` (товар считается
    /// «не установленным», при покупке метка конвертируется в настоящий
    /// отсек). В оригинале (`sub_B616`): `flag != 0`.
    fn flag_fitted(&self, mem: &[u8], number: u8) -> bool {
        let flag = mem[self.flag_addr(number)];
        if self.layout.dec_flags {
            let cobra = mem[self.pilot() + OFF_COBRA];
            if number == 3 {
                flag.wrapping_add(1) == cobra
            } else {
                (flag | 0x80).wrapping_add(1) == cobra
            }
        } else {
            flag != 0
        }
    }

    /// Отметить товар купленным (см. [`Self::flag_fitted`]).
    fn mark_fitted(&mut self, mem: &mut [u8], number: u8) {
        let a = self.flag_addr(number);
        if self.layout.dec_flags {
            // `loc_EA75`: `dec (hl)` — с нуля получается 0xFF.
            mem[a] = mem[a].wrapping_sub(1);
        } else {
            // `loc_B64F`: `ld (hl),0FFh`.
            mem[a] = 0xFF;
        }
    }

    /// Цена товара `number` (1..14) из таблицы, десятые.
    ///
    /// `sub_EB2F`/`sub_B70A`: запись `(number-1)*3` от базы: `b0*10000 +
    /// b2*256 + b1`. Для FUEL средний байт — динамическая цена недостающего
    /// топлива (`byte_7060`/`byte_9237`).
    fn price(&self, mem: &[u8], number: u8) -> u32 {
        let base = self.layout.prices as usize + (number as usize - 1) * 3;
        let b0 = mem[base] as u32;
        let b1 = mem[base + 1] as u32;
        let b2 = mem[base + 2] as u32;
        b0 * 10000 + b2 * 256 + b1
    }

    /// Цена для **отображения** — с сохранением сдвига бага оригинала Elite
    /// (см. комментарий в начале файла): когда строка FUEL скрыта, все
    /// строки показывают цену предыдущей записи.
    fn price_shown(&self, mem: &[u8], number: u8) -> u32 {
        let idx = if self.layout.hide_fuel_on_mission && self.rows.first() != Some(&1) {
            number as usize - 2
        } else {
            number as usize - 1
        };
        let base = self.layout.prices as usize + idx * 3;
        let b0 = mem[base] as u32;
        let b1 = mem[base + 1] as u32;
        let b2 = mem[base + 2] as u32;
        b0 * 10000 + b2 * 256 + b1
    }

    /// Списать деньги (`sub_D3EF` + `sub_D3FD` — то же самое вычитание).
    fn pay(&mut self, mem: &mut [u8], price: u32) -> bool {
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
    fn refund(&mut self, mem: &mut [u8], sum: u32) {
        self.cash += sum;
        self.write_cash(mem);
    }

    fn write_cash(&mut self, mem: &mut [u8]) {
        let p = self.pilot();
        let (c2, c1) = ((self.cash / 10000) as u16, (self.cash % 10000) as u16);
        mem[p + OFF_CASH2..p + OFF_CASH2 + 2].copy_from_slice(&c2.to_le_bytes());
        mem[p + OFF_CASH1..p + OFF_CASH1 + 2].copy_from_slice(&c1.to_le_bytes());
    }

    fn set_toast(&mut self, msg: impl Into<String>) {
        self.toast = Some(msg.into());
        self.toast_frames = TOAST_FRAMES;
    }

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
                if self.layout.dec_flags && mem[self.flag_addr(3)] == 0x59 {
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

    /// Обновить производную переменную `Fuel` (`SetPilotFuel`:
    /// `Fuel = 2*fuel + fuel/4` — дальность прыжка в шагах `fuel>>2`).
    fn write_fuel_derived(&self, mem: &mut [u8]) {
        let f = self.fuel;
        mem[self.layout.fuel_derived as usize] = f.wrapping_mul(2).wrapping_add(f / 4);
    }

    /// Навести указатель: строка списка или вариант крепления.
    pub fn point_at(&mut self, win_x: f64, win_y: f64) {
        self.hover = None;
        self.mount_hover = None;
        if self.mount.is_some() {
            for i in 0..4u8 {
                if inside(self.mount_rect(i), win_x, win_y) {
                    self.mount_hover = Some(i);
                }
            }
            return;
        }
        let g = self.geo();
        if win_x < g.x0 as f64 || win_x >= (g.x0 + g.list_w as i32) as f64 {
            return;
        }
        let i = ((win_y - g.list_y as f64) / g.row_h as f64).floor();
        if i >= 0.0 && i < self.rows.len() as f64 {
            let row = i as usize;
            if g.list_y + (row as i32 + 1) * g.row_h <= g.content_bottom {
                self.hover = Some(row);
            }
        }
    }

    // ————— Геометрия —————

    fn geo(&self) -> Geo {
        // Масштаб подбирается и по высоте (строки), и по ширине: список
        // занимает ровно свои колонки, всё остальное — панель корабля.
        let ts_h = (self.win_h as i32 / 300).max(1);
        let ts_w = (self.win_w as i32 / ((LIST_CHARS + PANEL_MIN_CHARS + 6) * 8)).max(1);
        let ts = ts_h.min(ts_w) as usize;
        let ch = 8 * ts as i32;
        let line = ch;
        let x0 = 2 * ch;
        // Шапка: заголовок + строка SYSTEM/TECH/CASH.
        let title_ts = ts + 1;
        let top = 4 * ts as i32 + 8 * title_ts as i32 + line + 8 * ts as i32;
        let help_y = self.win_h as i32 - line - 6 * ts as i32;
        let content_bottom = help_y - 4 * ts as i32;
        // Слева — только колонки списка, остальная ширина уходит панели.
        let list_w = (LIST_CHARS * ch).max(0) as usize;
        let panel_x = x0 + list_w as i32 + 2 * ch;
        let panel_w = (self.win_w as i32 - panel_x - x0).max(0) as usize;
        let list_y = top + 2 * ts as i32;
        let row_h = line * 5 / 2;
        Geo {
            ts,
            ch,
            line,
            x0,
            top,
            list_y,
            row_h,
            list_w,
            x_num: x0 + 1 * ch,
            x_name: x0 + 4 * ch,
            x_price: x0 + (4 + NAME_CHARS + 1 + PRICE_CHARS) * ch,
            price_w: PRICE_CHARS,
            panel_x,
            panel_w,
            help_y,
            content_bottom,
        }
    }

    /// Прямоугольник варианта крепления `i` в окне модалки.
    fn mount_rect(&self, i: u8) -> Rect {
        let g = self.geo();
        let mw = (44 * g.ch).max(0) as usize;
        let opt_h = (g.line + 4 * g.ts as i32).max(1);
        let mh = 2 * g.line + 4 * opt_h + g.line + 4 * g.ts as i32;
        let mx = ((self.win_w as i32 - mw as i32) / 2).max(0);
        let my = ((g.top + g.content_bottom - mh as i32) / 2).max(0);
        (mx + 2 * g.ch, my + 2 * g.line + i as i32 * opt_h, mw - 4 * g.ch as usize, opt_h as usize)
    }

    // ————— Отрисовка —————

    /// Нарисовать экран во весь слой. `mem` — живая память игры: из неё
    /// берутся цены (в том числе динамическая цена топлива).
    pub fn draw(&mut self, f: &mut Frame, font: &[u8], mem: &[u8]) {
        self.win_w = f.width as u32;
        self.win_h = f.height as u32;
        if self.toast_frames > 0 {
            self.toast_frames -= 1;
        } else {
            self.toast = None;
        }
        let g = self.geo();
        let ts = g.ts;
        let ch = g.ch;
        let line = g.line;
        let x0 = g.x0;
        f.clear(BG);

        // ——— Шапка: заголовок и строки данных командира ———
        let title_ts = ts + 1;
        let title = "EQUIP SHIP";
        let tw = title.len() as i32 * 8 * title_ts as i32;
        f.draw_text(
            font,
            ((f.width as i32 - tw) / 2).max(0),
            4 * ts as i32,
            title,
            TEXT,
            title_ts,
        );
        let info = format!(
            "SYSTEM {:<10} TECH LEVEL {:>2}",
            self.system, self.tech
        );
        let iy = 4 * ts as i32 + 8 * title_ts as i32 + 4 * ts as i32;
        f.draw_text(font, x0, iy, &info, TEXT_DIM, ts);
        let cash_s = format!("CASH {} CR", fmt_tenths(self.cash));
        let cx = (f.width as i32 - x0 - cash_s.len() as i32 * ch).max(0);
        f.draw_text(font, cx, iy, &cash_s, PRICE_FG, ts);

        // ——— Список товаров ———
        let content_h = (g.content_bottom - g.list_y).max(1) as usize;
        f.fill_rect(x0, g.list_y, g.list_w, content_h, LIST_BG);
        f.hline(x0, g.list_y, g.list_w, BORDER);
        f.hline(x0, g.list_y + content_h as i32 - 1, g.list_w, BORDER);
        f.vline(x0, g.list_y, content_h, BORDER);
        f.vline(x0 + g.list_w as i32 - 1, g.list_y, content_h, BORDER);
        f.set_clip(x0, g.list_y, g.list_w, content_h);
        for (i, &number) in self.rows.iter().enumerate() {
            let ry = g.list_y + i as i32 * g.row_h;
            let sel = i == self.cursor;
            let hov = self.hover == Some(i);
            if sel {
                f.fill_rect(x0 + 1, ry, g.list_w - 2, g.row_h as usize, CURSOR_BG);
            } else if hov {
                f.fill_rect(x0 + 1, ry, g.list_w - 2, g.row_h as usize, HOVER_BG);
            }
            let ty = ry + (g.row_h - line) / 2;
            let item = &ITEMS[(number - 1) as usize];
            // «Установлено» есть только у флаговых товаров: у лазеров свой
            // слот крепления, а байт «флага» номера лазера — чужое поле
            // командира (в оригинале лазеры «PRESENT» не проверяют).
            let fitted = matches!(item.kind, Kind::Flag | Kind::Bay)
                && self.fitted[number as usize];
            let num_s = format!("{number:>2}");
            f.draw_text(font, g.x_num, ty, &num_s, TEXT_DIM, ts);
            let name_color = if fitted { FITTED_FG } else { TEXT };
            f.draw_text(font, g.x_name, ty, item.name, name_color, ts);
            // На месте цены у установленного товара — «FITTED» (иначе
            // название и цена налезали бы друг на друга на узком списке).
            // Иначе цена показывается «как в оригинале» (со сдвигом бага Elita).
            let (price_s, price_color) = if fitted {
                ("FITTED".to_string(), FITTED_FG)
            } else {
                (
                    format!("{:>w$}", fmt_tenths(self.price_shown(mem, number)), w = g.price_w as usize),
                    PRICE_FG,
                )
            };
            let px = g.x_price - price_s.len() as i32 * ch;
            f.draw_text(font, px, ty, &price_s, price_color, ts);
        }
        f.clear_clip();

        // ——— Панель корабля справа ———
        if g.panel_w > 0 {
            let ph = (g.content_bottom - g.list_y).max(1) as usize;
            f.fill_rect(g.panel_x, g.list_y, g.panel_w, ph, PANEL_BG);
            f.hline(g.panel_x, g.list_y, g.panel_w, BORDER);
            f.hline(g.panel_x, g.list_y + ph as i32 - 1, g.panel_w, BORDER);
            f.vline(g.panel_x, g.list_y, ph, BORDER);
            f.vline(g.panel_x + g.panel_w as i32 - 1, g.list_y, ph, BORDER);
            let px = g.panel_x + 2 * ch;
            // Ширина панели в символах (за вычетом рамки и боковых отступов).
            let pch = (g.panel_w as i32 / ch - 4).max(1) as usize;
            let step = line + 2 * ts as i32;
            let mut texts: Vec<(String, u32)> = vec![
                ("YOUR SHIP".to_string(), TEXT),
                (
                    format!("{:<9}{}/{} LY", "FUEL", fmt_fuel(self.fuel), fmt_fuel(FUEL_MAX)),
                    TEXT_DIM,
                ),
                (
                    format!("{:<9}{}/{}", "MISSILES", self.missiles, MISSILES_MAX),
                    TEXT_DIM,
                ),
                (
                    format!("{:<9}{} T", "CARGO", self.free_cargo),
                    TEXT_DIM,
                ),
                ("LASERS".to_string(), TEXT),
            ];
            for (i, m) in MOUNTS.iter().enumerate() {
                texts.push((
                    format!("{:<7}{}", m, laser_name(self.lasers[i])),
                    TEXT_DIM,
                ));
            }
            // Описание выбранного/наведённого товара; длинные подсказки
            // переносятся по словам под ширину панели.
            if let Some(i) = self.hover.or(Some(self.cursor)) {
                if let Some(&number) = self.rows.get(i) {
                    let item = &ITEMS[(number - 1) as usize];
                    texts.push((String::new(), TEXT_DIM));
                    texts.push((item.name.to_string(), TEXT));
                    texts.push((
                        format!("{} CR", fmt_tenths(self.price(mem, number))),
                        PRICE_FG,
                    ));
                    for l in wrap(item.desc, pch) {
                        texts.push((l, TEXT_DIM));
                    }
                }
            }
            f.set_clip(g.panel_x, g.list_y, g.panel_w, ph);
            let mut py = g.list_y + 2 * ts as i32;
            for (s, c) in &texts {
                f.draw_text(font, px, py, s, *c, ts);
                py += step;
            }
            f.clear_clip();
        }

        // ——— Окно выбора крепления лазера ———
        if let Some(number) = self.mount {
            self.draw_mount_modal(f, font, number);
        }

        // ——— Баннер «в полёте» ———
        if self.in_flight {
            let msg = "DOCK AT A STATION TO BUY EQUIPMENT";
            let scale = title_ts;
            let mw = msg.len() as i32 * 8 * scale as i32 + 6 * ch;
            let mh = 8 * scale as i32 + 6 * ts as i32;
            let mx = ((f.width as i32 - mw) / 2).max(0);
            let my = (g.list_y + g.content_bottom) / 2 - mh / 2;
            f.fill_rect(mx, my, mw as usize, mh as usize, WARN_BG);
            f.hline(mx, my, mw as usize, WARN_FG);
            f.hline(mx, my + mh - 1, mw as usize, WARN_FG);
            f.draw_text(
                font,
                mx + 3 * ch,
                my + 3 * ts as i32,
                msg,
                WARN_FG,
                scale,
            );
        }

        // ——— Тост ———
        if let Some(msg) = &self.toast {
            let mw = msg.len() as i32 * ch + 6 * ch;
            let mx = ((f.width as i32 - mw) / 2).max(0);
            let my = g.help_y - 2 * line - 2 * ts as i32;
            f.fill_rect(mx, my, mw as usize, (line + 4 * ts as i32) as usize, WARN_BG);
            f.draw_text(font, mx + 3 * ch, my + 2 * ts as i32, msg, WARN_FG, ts);
        }

        // ——— Строка помощи ———
        let help = if let Some(d) = self.pending {
            format!("ITEM? {d}_    UP/DOWN SELECT  ENTER BUY  F4/ESC CLOSE")
        } else {
            "ENTER/CLICK BUY   UP/DOWN SELECT   1-9 ITEM NO.   F4/ESC CLOSE".to_string()
        };
        f.draw_text(font, x0, g.help_y, &help, TEXT_DIM, ts);
    }

    /// Окно выбора крепления (как в оригинале: `0 FRONT .. 3 RIGHT`).
    fn draw_mount_modal(&self, f: &mut Frame, font: &[u8], number: u8) {
        let g = self.geo();
        let ts = g.ts;
        let ch = g.ch;
        let line = g.line;
        let Kind::Laser(laser) = ITEMS[(number - 1) as usize].kind else {
            return;
        };
        let title = format!("MOUNT {} LASER WHERE?", LASER_NAMES[laser as usize]);
        let mw = (44 * ch).max(0) as usize;
        let opt_h = (line + 4 * ts as i32).max(1);
        let mh = (2 * line + 4 * opt_h + line + 4 * ts as i32) as usize;
        let mx = ((f.width as i32 - mw as i32) / 2).max(0);
        let my = ((g.top + g.content_bottom - mh as i32) / 2).max(0);
        f.fill_rect(mx, my, mw, mh, 0xFF00_0000);
        f.hline(mx, my, mw, BORDER);
        f.hline(mx, my + mh as i32 - 1, mw, BORDER);
        f.vline(mx, my, mh, BORDER);
        f.vline(mx + mw as i32 - 1, my, mh, BORDER);
        f.draw_text(
            font,
            mx + 2 * ch,
            my + line / 2 + 2 * ts as i32,
            &title,
            TEXT,
            ts,
        );
        for i in 0..4u8 {
            let r = self.mount_rect(i);
            if self.mount_hover == Some(i) {
                f.fill_rect(r.0, r.1, r.2, r.3, CURSOR_BG);
            }
            let cur = laser_name(self.lasers[i as usize]);
            let s = format!("{}  {:<6} {}", i, MOUNTS[i as usize], cur);
            let ty = r.1 + (r.3 as i32 - line) / 2;
            f.draw_text(font, r.0 + ch, ty, &s, TEXT, ts);
        }
        let hy = my + mh as i32 - line - 2 * ts as i32;
        f.draw_text(
            font,
            mx + 2 * ch,
            hy,
            "0-3 SELECT   ESC CANCEL",
            TEXT_DIM,
            ts,
        );
    }
}

impl Default for Shop {
    fn default() -> Self {
        Shop::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            ELITE_SHOP.pilot as usize,
        );
        let commander = mem[kp..kp + 104].to_vec();
        mem[ep..ep + 104].copy_from_slice(&commander);
        let (kpr, epr) = (
            KLADOV_SHOP.prices as usize,
            ELITE_SHOP.prices as usize,
        );
        let prices = mem[kpr..kpr + 42].to_vec();
        mem[epr..epr + 42].copy_from_slice(&prices);
        let gl = ELITE_SHOP.galaxy;
        mem[gl.seed..gl.seed + 6].copy_from_slice(&[0x4A, 0x5A, 0x48, 0x02, 0x53, 0xB7]);
        mem[gl.sys_y] = 173;
        mem[gl.sys_x] = 20;
        mem[gl.digraphs..gl.digraphs + 64]
            .copy_from_slice(b"ALLEXEGEZACEBISOUSESARMAINDIREA?ERATENBERALAVETIEDORQUANTEISRION");
        mem[ELITE_SHOP.in_danger as usize] = 0;
        // Выдана 1-я миссия — строка FUEL должна скрыться.
        mem[ep + OFF_MISSION1] = 1;
        let mut shop = Shop::new();
        shop.open(&mem, ELITE_SHOP);
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
        let before = cash_at(&mem, ELITE_SHOP.pilot);
        shop.buy_number(&mut mem, 11);
        assert_eq!(mem[ep + 45 + 11], 0xFF);
        assert_eq!(cash_at(&mem, ELITE_SHOP.pilot), before - 15000);
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
        let font = &mem[0x3D00..0x3D00 + 0x300];
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
}
