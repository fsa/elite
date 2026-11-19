//! Отрисовка экрана цен. На станции это полноценный рынок с выбором строки и
//! подсказками торговли; в полёте — только таблица цен без подсветки и кнопок.
//!
//! Оформление в стиле экранов информации о планете и командира: чёрный фон,
//! белая рамка, голубые подписи/заголовки колонок и белые значения. Заголовок
//! стилизован как в оригинале: верхняя часть букв белая, нижняя — голубая.

use super::*;

/// Фон экрана — чёрный.
const BG: u32 = 0xFF00_0000;
/// Белая рамка вокруг всего экрана.
const FRAME: u32 = crate::cockpit::FRAME_COLOR;
/// Голубой цвет подписей.
const LABEL: u32 = 0xFF00_D7D7;
/// Значения — белые.
const VALUE: u32 = crate::cockpit::FRAME_COLOR;
/// Подсветка выбранной строки списка.
const SELECT_BG: u32 = 0xFF00_5458;

/// Геометрия экрана.
pub(super) struct Layout {
    pub(super) margin: i32,
    pub(super) frame: i32,
    pub(super) text: usize,
    pub(super) line: i32,
    pub(super) title_y: i32,
    pub(super) header_top_y: i32,
    pub(super) header_bottom_y: i32,
    /// Первая строка таблицы: отступ от заголовка равен шагу строк.
    pub(super) first_row_y: i32,
    /// Шаг строк таблицы, как на F7/F9.
    pub(super) row_h: i32,
}

/// Сосчитать геометрию окна `w` x `h`.
pub(super) fn layout(w: usize, h: usize) -> Layout {
    let margin = crate::cockpit::panel_geometry(w as u32, h as u32).margin;
    let frame = crate::cockpit::LINE.max(1);
    let text = (h as f32 / 300.0).floor().max(1.0) as usize;
    let line = 8 * text as i32;
    let title_y = margin + frame + line;
    // Две строки заголовка колонок остаются плотными: между ними ровно одна
    // строка текста. Первая строка таблицы отстоит от заголовка на тот же
    // шаг, что и остальные строки, — разреженный, как на экранах F7/F9.
    let header_top_y = title_y + 2 * line;
    let header_bottom_y = header_top_y + line;
    let row_h = line + line / 2;
    Layout {
        margin,
        frame,
        text,
        line,
        title_y,
        header_top_y,
        header_bottom_y,
        first_row_y: header_bottom_y + row_h,
        row_h,
    }
}

/// Нарисовать экран цен: на станции — с выбором строки и подсказками торговли,
/// в полёте — только таблицу цен.
pub(super) fn draw(hd: &mut Frame, font: &[u8; crate::font::ZX_FONT_LEN], market: &Market) {
    let w = hd.width;
    let h = hd.height;
    if w == 0 || h == 0 {
        return;
    }
    hd.clear(BG);
    let l = layout(w, h);
    let ts = l.text;
    let info = &market.info;

    let inner_x = l.margin + l.frame;
    let inner_w = (w as i32 - 2 * inner_x).max(0);
    let ch = 8 * ts as i32;

    // Заголовок — по центру, двухцветный: верх белый, низ голубой. Как на
    // экране F7, имя системы стоит слева и печатается заглавными буквами.
    let title = title(&market.system);
    let tw = title.chars().count() as i32 * ch;
    let tx = inner_x + ((inner_w - tw) / 2).max(0);
    draw_two_tone_text(hd, font, tx, l.title_y, &title, VALUE, LABEL, ts);

    // Колонки на всю ширину экрана. UNIT PRICE и QUANTITY FOR SALE — в две
    // строки; PRODUCT и UNIT опущены на вторую строку заголовка. IN HOLD —
    // новое поле с количеством товара в трюме корабля.
    let left = inner_x;
    // Отступ содержимого от рамки — одно знакоместо, как на F7/F9.
    let pad = ch;
    let x_name = left + pad;
    let x_unit = left + inner_w * 45 / 100;
    let x_price = left + inner_w * 55 / 100;
    let x_price_header = x_price + 3 * ch;
    let x_qty = left + inner_w * 72 / 100;
    let x_hold = left + inner_w * 90 / 100;

    hd.draw_text(font, x_name, l.header_bottom_y, "PRODUCT", LABEL, ts);
    hd.draw_text(font, x_unit, l.header_bottom_y, "UNIT", LABEL, ts);
    hd.draw_text(font, x_price_header, l.header_top_y, "UNIT", LABEL, ts);
    hd.draw_text(font, x_price_header, l.header_bottom_y, "PRICE", LABEL, ts);
    hd.draw_text(font, x_qty, l.header_top_y, "QUANTITY", LABEL, ts);
    hd.draw_text(font, x_qty, l.header_bottom_y, "FOR SALE", LABEL, ts);
    hd.draw_text(font, x_hold, l.header_top_y, "IN", LABEL, ts);
    hd.draw_text(font, x_hold, l.header_bottom_y, "HOLD", LABEL, ts);

    let table_bottom = h as i32 - l.margin - l.frame;
    for (i, item) in info.items.iter().enumerate() {
        let y = l.first_row_y + i as i32 * l.row_h;
        if y + l.line > table_bottom {
            break;
        }
        // В полёте нет выбора строки — экран только для чтения цен.
        if market.docked && i == market.cursor {
            // Подсветка выходит за пределы глифа на две строки пикселей с каждой
            // стороны: по высоте она почти заполняет шаг между строками.
            let pad = 2 * ts as i32;
            hd.fill_rect(
                left,
                y - pad,
                inner_w.max(0) as usize,
                (l.line + 2 * pad).max(1) as usize,
                SELECT_BG,
            );
        }
        hd.draw_text(font, x_name, y, item.name, VALUE, ts);
        hd.draw_text(font, x_unit, y, item.unit, VALUE, ts);

        let price = format!("{:>7.1}", item.price_tenths as f32 / 10.0);
        hd.draw_text(font, x_price, y, &price, VALUE, ts);

        // Число выравнивается по правому краю, единица идёт сразу после него.
        // Прочерк занимает позицию младшего разряда и единицы не получает.
        let qty = if item.quantity == 0 {
            format!("{:>3}", "-")
        } else {
            format!("{:>3}{}", item.quantity, item.unit)
        };
        hd.draw_text(font, x_qty, y, &qty, VALUE, ts);

        let hold = if item.in_hold == 0 {
            "-".to_string()
        } else {
            format!("{}{}", item.in_hold, item.unit)
        };
        hd.draw_text(font, x_hold, y, &hold, VALUE, ts);
    }

    // Одна нижняя строка: деньги и, на станции, подсказки управления.
    // Ниже её больше ничего не выводится. Между параметрами — два пробела,
    // независимо от длины суммы.
    let status_y = table_bottom - 3 * l.line;
    let mut x = x_name;
    for (i, field) in status_fields(info.cash_tenths, market.docked)
        .iter()
        .enumerate()
    {
        if i > 0 {
            x += 2 * ch;
        }
        x = draw_field(hd, font, x, status_y, field, ts);
    }

    draw_screen_frame(hd, &l);
}

/// Поле нижней строки: имя до двоеточия и двоеточие со значением рисуются
/// разными цветами, как в F7/F9.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Field {
    pub(super) name: &'static str,
    pub(super) value: String,
}

/// Параметры нижней строки. В полёте торговать нельзя, поэтому остаётся только
/// сумма на счёте; на станции добавляются подсказки выбора и торговли.
pub(super) fn status_fields(cash_tenths: u32, docked: bool) -> Vec<Field> {
    let mut fields = vec![Field {
        name: "Cash",
        value: format!("{}.{} Cr", cash_tenths / 10, cash_tenths % 10),
    }];
    if docked {
        fields.extend([
            Field {
                name: "Select",
                value: "Up/Down".into(),
            },
            Field {
                name: "Sell",
                value: "Left".into(),
            },
            Field {
                name: "Buy",
                value: "Right".into(),
            },
        ]);
    }
    fields
}

/// Текст нижней строки — для тестов и отладки.
#[cfg(test)]
pub(super) fn status_line(cash_tenths: u32, docked: bool) -> String {
    status_fields(cash_tenths, docked)
        .iter()
        .map(|f| format!("{}: {}", f.name, f.value))
        .collect::<Vec<_>>()
        .join("  ")
}

/// Нарисовать одно поле: имя голубое, двоеточие и значение белые.
fn draw_field(
    hd: &mut Frame,
    font: &[u8; crate::font::ZX_FONT_LEN],
    x: i32,
    y: i32,
    field: &Field,
    scale: usize,
) -> i32 {
    let cell = 8 * scale as i32;
    hd.draw_text(font, x, y, field.name, LABEL, scale);
    let vx = x + field.name.chars().count() as i32 * cell;
    let tail = format!(": {}", field.value);
    hd.draw_text(font, vx, y, &tail, VALUE, scale);
    vx + tail.chars().count() as i32 * cell
}

/// Заголовок экрана: имя системы слева заглавными буквами, как в F7.
pub(super) fn title(system: &str) -> String {
    if system.is_empty() {
        "MARKET PRICES".to_string()
    } else {
        format!("{} MARKET PRICES", system.to_uppercase())
    }
}

/// Нарисовать строку двухцветным шрифтом: верхняя половина глифов `top`,
/// нижняя — `bottom` (как в оригинальной игре).
fn draw_two_tone_text(
    hd: &mut Frame,
    font: &[u8; crate::font::ZX_FONT_LEN],
    x: i32,
    y: i32,
    s: &str,
    top: u32,
    bottom: u32,
    scale: usize,
) {
    let h = 8 * scale as i32;
    let mid = h / 2;
    let w = s.chars().count() as i32 * 8 * scale as i32;

    // Нижняя половина.
    hd.set_clip(x, y + mid, w.max(0) as usize, (h - mid).max(0) as usize);
    hd.draw_text(font, x, y, s, bottom, scale);
    hd.clear_clip();

    // Верхняя половина.
    hd.set_clip(x, y, w.max(0) as usize, mid.max(0) as usize);
    hd.draw_text(font, x, y, s, top, scale);
    hd.clear_clip();
}

/// Обвести экран белой рамкой.
fn draw_screen_frame(hd: &mut Frame, l: &Layout) {
    let w = hd.width as i32;
    let h = hd.height as i32;
    let m = l.margin.max(0);
    let fw = (w - 2 * m).max(0);
    let fh = (h - 2 * m).max(0);
    let t = l.frame.clamp(1, fw.min(fh).max(1)) as usize;
    if fw <= 0 || fh <= 0 {
        return;
    }
    hd.fill_rect(m, m, fw as usize, t, FRAME);
    hd.fill_rect(m, m + fh - t as i32, fw as usize, t, FRAME);
    let side_h = (fh - 2 * t as i32).max(0) as usize;
    if side_h > 0 {
        hd.fill_rect(m, m + t as i32, t, side_h, FRAME);
        hd.fill_rect(m + fw - t as i32, m + t as i32, t, side_h, FRAME);
    }
}
