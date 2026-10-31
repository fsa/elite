//! Контуры граней и их непрозрачная заливка.

use super::occlusion::fit_plane;
use super::*;

/// Контур обращённой к камере грани в экранных координатах.
pub(super) struct FaceShape {
    /// Рёбра контура в индексах вершин модели (петель может быть несколько,
    /// например вырезы двигателей).
    pub(super) edges: Vec<(usize, usize)>,
    pub(super) x0: f32,
    pub(super) x1: f32,
    pub(super) y0: f32,
    pub(super) y1: f32,
    /// Плоскость `1/z = aa*x + bb*y + cc` и её ошибка по вершинам.
    pub(super) plane: Option<(f32, f32, f32)>,
    pub(super) plane_error: f32,
}

/// Собрать контуры всех видимых граней.
pub(super) fn face_shapes(ship: &Ship, scr: &[(f32, f32, f32)], vis: &[bool]) -> Vec<FaceShape> {
    let mut face_edges: Vec<Vec<(usize, usize)>> = vec![Vec::new(); ship.faces.len()];
    for e in ship.edges {
        let (a, b) = (e.v[0] as usize, e.v[1] as usize);
        if a >= scr.len() || b >= scr.len() {
            continue;
        }
        let f0 = e.faces & 0x0F;
        let f1 = e.faces >> 4;
        // 0xF — признак «грани нет»; одинаковые полубайты не должны
        // добавлять ребро в контур дважды (иначе правило «чёт-нечёт»
        // посчитает его дважды и оставит дырку).
        if f0 != 0xF {
            if let Some(list) = face_edges.get_mut(f0 as usize) {
                list.push((a, b));
            }
        }
        if f1 != f0 && f1 != 0xF {
            if let Some(list) = face_edges.get_mut(f1 as usize) {
                list.push((a, b));
            }
        }
    }
    let mut out = Vec::new();
    for (fi, _) in ship.faces.iter().enumerate() {
        if !vis.get(fi).copied().unwrap_or(false) {
            continue;
        }
        let edges = &face_edges[fi];
        if edges.len() < 3 {
            continue;
        }
        let mut vs: Vec<usize> = edges.iter().flat_map(|&(a, b)| [a, b]).collect();
        vs.sort_unstable();
        vs.dedup();
        if vs.len() < 3 {
            continue;
        }
        let pts: Vec<(f32, f32, f32)> = vs.iter().map(|&i| scr[i]).collect();
        let plane = fit_plane(&pts);
        let plane_error = match plane {
            Some((aa, bb, cc)) => pts.iter().fold(0.0f32, |e, &(x, y, iz)| {
                e.max((aa * x + bb * y + cc - iz).abs() / iz)
            }),
            None => f32::MAX,
        };
        let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        for &(i, j) in edges {
            for k in [i, j] {
                x0 = x0.min(scr[k].0);
                x1 = x1.max(scr[k].0);
                y0 = y0.min(scr[k].1);
                y1 = y1.max(scr[k].1);
            }
        }
        out.push(FaceShape {
            edges: edges.clone(),
            x0,
            x1,
            y0,
            y1,
            plane,
            plane_error,
        });
    }
    out
}

/// Залить видимые грани непрозрачным цветом.
///
/// Каждый контур заполняется построчно по правилу «чёт-нечёт»: это корректно
/// и для нескольких петель (вырезы двигателей), и для невыпуклых граней.
/// Все грани заливаются одним цветом, поэтому порядок заливки не важен —
/// перекрывающиеся видимые грани всё равно дают один и тот же тёмный тон.
pub(super) fn fill_faces(frame: &mut Frame, shapes: &[FaceShape], scr: &[(f32, f32, f32)]) {
    let w = frame.width as i32;
    let h = frame.height as i32;
    if w == 0 || h == 0 {
        return;
    }
    for shape in shapes {
        let y_lo = ((shape.y0 - 0.5).ceil() as i32).max(0);
        let y_hi = ((shape.y1 - 0.5).ceil() as i32 - 1).min(h - 1);
        if y_lo > y_hi {
            continue;
        }
        let mut xs: Vec<f32> = Vec::with_capacity(shape.edges.len());
        for y in y_lo..=y_hi {
            let yc = y as f32 + 0.5;
            xs.clear();
            for &(i, j) in &shape.edges {
                let (xi, yi, _) = scr[i];
                let (xj, yj, _) = scr[j];
                if (yi <= yc) != (yj <= yc) {
                    xs.push(xi + (yc - yi) / (yj - yi) * (xj - xi));
                }
            }
            xs.sort_by(f32::total_cmp);
            for pair in xs.chunks_exact(2) {
                let x_lo = ((pair[0] - 0.5).ceil() as i32).max(0);
                let x_hi = ((pair[1] - 0.5).ceil() as i32 - 1).min(w - 1);
                if x_lo <= x_hi {
                    frame.hline(x_lo, y, (x_hi - x_lo + 1) as usize, SHIP_FACE_COLOR);
                }
            }
        }
    }
}
