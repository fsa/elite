//! Индикаторы `E` (ECM) и `S` (близость станции).
//!
//! В версии Кладова оба символа — статичные глифы в изображении кокпита:
//! `PrnDangerEcmDock` только переключает их экранный атрибут (видимый белый
//! или невидимый чёрный). Здесь глифы повторяют ту же форму и рисуются
//! поверх панели только в активном состоянии.
//!
//! `E` стоит в левом нижнем углу центральной клетки, `S` — в правом нижнем,
//! зеркально верхним кружкам опасности и компаса.

use crate::display::Frame;

use super::geometry::{CockpitGeom, Rect};
use super::palette::CockpitStyle;
use super::values::CockpitValues;

/// Растр символа `E` (ECM) из кокпита версии Кладова: 8×8.
pub(super) const ECM_ICON: [u8; 8] = [0x7F, 0x63, 0x60, 0x64, 0x7C, 0x60, 0x63, 0x7F];
/// Растр символа `S` (станция рядом) из кокпита версии Кладова: 8×8.
pub(super) const STATION_ICON: [u8; 8] = [0x3E, 0x63, 0x60, 0x38, 0x0E, 0x03, 0x63, 0x3E];

/// Нарисовать `E` и `S` в нижних углах центральной клетки.
///
/// `E` слева, `S` справа — зеркально верхним кружкам. Неактивный символ не
/// рисуется: фон панели уже чёрный, а в оригинале он становится невидимым
/// через атрибут.
pub(super) fn draw(f: &mut Frame, g: &CockpitGeom, style: CockpitStyle, values: &CockpitValues) {
    if values.ecm_on {
        draw_icon(f, g.ecm_cell(), &ECM_ICON, style.line);
    }
    if values.station_near {
        draw_icon(f, g.station_cell(), &STATION_ICON, style.line);
    }
}

/// Нарисовать 8×8 глиф по центру клетки, по возможности увеличив его.
fn draw_icon(f: &mut Frame, area: Rect, icon: &[u8; 8], color: u32) {
    if area.is_empty() {
        return;
    }
    let scale = (area.h / 8).min(area.w / 8).max(1);
    let w = 8 * scale;
    let h = 8 * scale;
    let x = area.x + (area.w - w) / 2;
    let y = area.y + (area.h - h) / 2;
    f.set_clip(
        area.x,
        area.y,
        area.w.max(0) as usize,
        area.h.max(0) as usize,
    );
    for (row, bits) in icon.iter().enumerate() {
        for col in 0..8 {
            if bits & (0x80 >> col) != 0 {
                f.fill_rect(
                    x + col * scale,
                    y + row as i32 * scale,
                    scale as usize,
                    scale as usize,
                    color,
                );
            }
        }
    }
    f.clear_clip();
}
