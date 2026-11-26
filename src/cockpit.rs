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
//! - [`radar`] — классический овальный сканер и альтернативный локатор Кладова;
//! - [`status`] — символы `E` (ECM) и `S` (близость станции).

use crate::display::Frame;

pub use self::geometry::{CockpitGeom, LINE, Rect, compass_hit, panel_geometry};
pub use self::layout::KLADOV_COCKPIT;
pub use self::palette::{
    BACKGROUND, BAR_MAGENTA, BAR_RED, CockpitStyle, DANGER_COLOR, DANGER_GREEN, FRAME_COLOR,
};
#[cfg(test)]
pub use self::palette::{COMPASS_PLANET, COMPASS_STATION, COMPASS_SUN};
pub use self::compass::CompassView;
pub use self::radar::RadarView;
pub use self::values::{CockpitValues, Compass, CompassCircle, RadarContacts};

// Элементы ниже нужны только тестам модуля: боевой код обращается к
// подмодулям напрямую.
#[cfg(test)]
pub use self::geometry::{BAR_WIDTH_FRAC, LABEL_WIDTH_PER_ROW_PCT};
#[cfg(test)]
pub use self::labels::{LEFT_LABELS, RIGHT_LABELS};
#[cfg(test)]
pub use self::palette::{DANGER_YELLOW, MISSILE_LOCK_RED, MISSILE_TARGET_GREEN};
#[cfg(test)]
pub use self::values::{
    DEFLECTION_MAX, Danger, GAUGE_MAX, Indicator, MISSILE_ROW, MISSILE_SLOTS, MissileMode,
    PANEL_ROWS, RadarContact, RadarContactKind,
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

/// Нарисовать нижнюю половину кружка опасности на верхней линии рамки
/// вспомогательного экрана (карта, экран капитана).
///
/// В оригинале такой полукруг есть только в полёте; на станции индикатор
/// погашен, поэтому вызывающий код сам решает, показывать ли его.
pub fn draw_danger_half(f: &mut Frame, mem: &[u8], margin: i32, frame: i32) {
    danger::draw_screen_half(f, mem, margin, frame);
}

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
        self.draw_with_geom_views(
            f,
            g,
            font,
            style,
            values,
            RadarView::Classic,
            CompassView::Pointer,
        );
    }

    /// Нарисовать кокпит с явным выбором видов радара и компаса.
    pub fn draw_with_geom_views(
        &self,
        f: &mut Frame,
        g: &CockpitGeom,
        font: &[u8],
        style: CockpitStyle,
        values: &CockpitValues,
        radar_view: RadarView,
        compass_view: CompassView,
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
        compass::draw(f, g, style, values, compass_view);
        radar::draw_view(f, g, style, values, radar_view);
        status::draw(f, g, style, values);
    }
}

impl Default for Cockpit {
    fn default() -> Self {
        Self::new()
    }
}
