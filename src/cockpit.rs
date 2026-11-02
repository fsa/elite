//! Нативный кокпит (приборная панель) корабля.
//!
//! Кокпит используется на двух экранах:
//!
//! * в полёте (`F1`) — вокруг вида из кабины и под ним;
//! * на заставке — вокруг поля с вращающимся кораблём и под ним.
//!
//! Вся «обвязка» кокпита — главная рамка, рамка нижней части, таблица и
//! индикаторы — живёт здесь, а экраны только рисуют своё содержимое.
//!
//! В оригинале панель — статический растр в видеопамяти ZX (`DataCocpit`,
//! оригинал `0xC800`, Кладов `0xC000`), нижние 64 строки экрана; индикаторы
//! уложены в 7 строк по высоте, на которые поверх допечатываются значения:
//! шесть полос (щиты, топливо, температура, лазер, высота), скорость и четыре
//! энергоблока, компас и радиолокатор (`UpdateAllIndicators`, `PrintCompass`,
//! `PrintObjScaner`).
//!
//! Сейчас панель — таблица с рамками: слева и справа по семь строк, в центре
//! одна большая клетка на всю высоту. В каждой боковой строке две клетки:
//! короткая подпись/значок на чёрном фоне и длинная клетка-полоса. Полосы
//! залиты цветом типа корабля игрока: у Cobra Mk III красным, у Cobra Mk IV
//! фиолетовым; на заставке всегда красным. Подписи повторяют обозначения
//! оригинала: слева `FS`, `AS`, `FU`, `CT`, `LT`, `AL` и значок скорости,
//! справа `SP`, `RL`, `DC` и номера энергоблоков `1`..`4`. Они нарисованы
//! тем же шрифтом ПЗУ ZX, что и заголовок `ELITE` на заставке.
//!
//! Длинная клетка-полоса задаётся в процентах от ширины панели
//! ([`geometry::BAR_WIDTH_FRAC`]) и растягивается вместе с окном. Клетка подписи
//! фиксирована по высоте строки ([`geometry::LABEL_WIDTH_PER_ROW_PCT`]) и при
//! смене соотношения сторон не растягивается.
//!
//! Значения панели приходят одним снимком [`CockpitValues`]: кокпит не
//! читает память Z80 сам. [`layout::CockpitLayout::read`] превращает байты живой
//! памяти в [`values::Indicator`] для каждой строки. Статическая часть панели и
//! значения рисуются раздельно, но вызывающий код делает один вызов
//! [`Cockpit::draw`].
//!
//! Геометрия ([`CockpitGeom`]) отдаёт прямоугольники вида, панели, клеток и
//! разделителей, чтобы следующие шаги рисовали внутри неё, а не заново
//! считали отступы.
//!
//! Подсистемы модуля:
//!
//! - [`geometry`] — прямоугольники и раскладка панели по размеру окна;
//! - [`layout`] — адреса переменных Z80 и чтение показаний из памяти;
//! - [`values`] — модель показаний (индикаторы, опасность, компас);
//! - [`palette`] — цвета линий, полос и индикатора опасности;
//! - [`frame`] — главная рамка и рамка нижней части;
//! - [`table`] — статические разделители таблицы;
//! - [`labels`] — подписи коротких клеток и значок скорости;
//! - [`indicators`] — шкалы, отклонения и запас ракет;
//! - [`danger`] — индикатор опасности;
//! - [`compass`] — кольцо и указатель компаса;
//! - [`radar`] — классический овальный сканер (радар Кладова временно отключён);
//! - [`status`] — символы `E` (ECM) и `S` (близость станции).

use crate::display::Frame;

pub use self::geometry::{CockpitGeom, LINE, Rect, panel_geometry};
pub use self::layout::KLADOV_COCKPIT;
pub use self::palette::{
    BACKGROUND, BAR_MAGENTA, BAR_RED, CockpitStyle, DANGER_COLOR, FRAME_COLOR,
};
pub use self::values::CockpitValues;

// Элементы ниже нужны только тестам модуля: боевой код обращается к
// подмодулям напрямую.
#[cfg(test)]
pub use self::geometry::{BAR_WIDTH_FRAC, LABEL_WIDTH_PER_ROW_PCT};
#[cfg(test)]
pub use self::labels::{LEFT_LABELS, RIGHT_LABELS};
#[cfg(test)]
pub use self::palette::{DANGER_GREEN, DANGER_YELLOW};
#[cfg(test)]
pub use self::values::{
    Compass, DEFLECTION_MAX, Danger, GAUGE_MAX, Indicator, MISSILE_ROW, MISSILE_SLOTS, PANEL_ROWS,
    RadarContact, RadarContactKind, RadarContacts,
};

mod compass;
mod danger;
mod frame;
mod geometry;
mod indicators;
mod labels;
mod layout;
mod palette;
mod radar;
mod status;
mod table;
#[cfg(test)]
mod tests;
mod values;

/// Нативное оформление кокпита. Состояния нет — рамки статичны, а экраны
/// рисуют своё содержимое до вызова [`Cockpit::draw`].
pub struct Cockpit;

impl Cockpit {
    pub fn new() -> Self {
        Cockpit
    }

    /// Нарисовать кокпит: рамки, таблицу и текущие показания.
    ///
    /// `font` — знакогенератор ПЗУ ZX (коды 0x20..0x7F) для подписей,
    /// `style` — цвета линий и полос, `values` — снимок показаний на кадр.
    ///
    /// Удобная обёртка для тестов и простых вызовов; экраны заранее считают
    /// геометрию и зовут [`Cockpit::draw_with_geom`].
    #[allow(dead_code)]
    pub fn draw(&self, f: &mut Frame, font: &[u8], style: CockpitStyle, values: &CockpitValues) {
        let g = panel_geometry(f.width as u32, f.height as u32);
        self.draw_with_geom(f, &g, font, style, values);
    }

    /// Нарисовать кокпит по заранее вычисленной геометрии.
    ///
    /// Содержимое вида (космос, заставка) должно быть уже нарисовано:
    /// кокпит только заливает фон нижней части, рисует две рамки и таблицу
    /// с индикаторами в нижней части.
    pub fn draw_with_geom(
        &self,
        f: &mut Frame,
        g: &CockpitGeom,
        font: &[u8],
        style: CockpitStyle,
        values: &CockpitValues,
    ) {
        // Нижняя часть непрозрачна и закрывает всё, что под ней.
        if g.panel_h > 0 && g.width > 0 {
            f.fill_rect(
                0,
                g.panel_y,
                g.width as usize,
                g.panel_h as usize,
                BACKGROUND,
            );
        }
        // Главная рамка вокруг вида и отдельная рамка нижней части с
        // отступом от неё.
        frame::draw(f, g, style);
        table::draw(f, g, style);
        labels::draw(f, g, font, style.line);
        indicators::draw(f, g, style, values);
        danger::draw(f, g, values);
        compass::draw(f, g, style, values);
        radar::draw(f, g, style, values);
        status::draw(f, g, style, values);
    }
}

impl Default for Cockpit {
    fn default() -> Self {
        Self::new()
    }
}
