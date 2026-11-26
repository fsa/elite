//! Контуры граней и их непрозрачная заливка.

use super::occlusion::fit_plane;
use super::*;

/// Контур обращённой к камере грани в экранных координатах.
pub(super) struct FaceShape {
    /// Индекс грани в модели, для детерминированного порядка при равной глубине.
    pub(super) fi: usize,
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
    /// Внешний вид грани из данных модели.
    pub(super) style: FaceStyle,
    /// Средняя обратная глубина вершин — для заливки от дальней к ближней.
    pub(super) invz: f32,
}

/// Убрать из контура рёбра, ведущие в «усики» со степенью вершины 1.
///
/// В данных моделей часть граней имеет декоративные рёбра (детали на
/// поверхности), которые не образуют замкнутый контур. Для каркаса это
/// нормально, а сканлайнная заливка по правилу «чёт-нечёт» из-за таких
/// рёбер даёт «протёки». Убираем только висячие ветви, не трогая замкнутые
/// петли и вырезы двигателей.
fn prune_spurs(edges: &mut Vec<(usize, usize)>) {
    loop {
        let mut deg: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for &(a, b) in edges.iter() {
            *deg.entry(a).or_insert(0) += 1;
            *deg.entry(b).or_insert(0) += 1;
        }
        let before = edges.len();
        edges.retain(|&(a, b)| {
            deg.get(&a).copied().unwrap_or(0) >= 2 && deg.get(&b).copied().unwrap_or(0) >= 2
        });
        if edges.len() == before {
            break;
        }
    }
}

fn engine_glow_color(phase: f32) -> u32 {
    // Пульсация яркости: быстрое, но плавное мерцание, не «мигание».
    let pulse = 0.60 + 0.25 * phase.sin();
    let (r, g, b) = super::ENGINE_GLOW_BASE;
    let ch = |v: f32| ((v * pulse).clamp(0.0, 1.0) * 255.0) as u32;
    0xFF00_0000 | (ch(r) << 16) | (ch(g) << 8) | ch(b)
}

/// Маленькая, но яркая сердцевина сопла. Мерцает быстрее грани и не гаснет
/// полностью: тёмная грань остаётся фоном, а «работа» двигателя видна по ядру.
pub(super) fn engine_core_color(phase: f32, engine: f32) -> u32 {
    let level = 0.55 + 0.45 * engine.clamp(0.0, 1.0);
    let pulse = (0.75 + 0.20 * (phase * 1.7).sin()) * level;
    let (r, g, b) = super::ENGINE_CORE_BASE;
    let ch = |v: f32| ((v * pulse).clamp(0.0, 1.0) * 255.0) as u32;
    0xFF00_0000 | (ch(r) << 16) | (ch(g) << 8) | ch(b)
}

/// Какая доля поперечника грани приходится на горячую сердцевину.
const CORE_RADIUS_RATIO: f32 = 0.20;

/// Залить маленький ромб в центре грани двигателя.
///
/// Сердцевина рисуется сразу после своей грани: более близкие поверхности
/// всё равно перекроют её при общей сортировке граней от дальней к ближней.
fn fill_engine_core(
    frame: &mut Frame,
    shape: &FaceShape,
    scr: &[(f32, f32, f32)],
    glow_phase: f32,
    engine: f32,
) {
    let mut vs: Vec<usize> = shape.edges.iter().flat_map(|&(a, b)| [a, b]).collect();
    vs.sort_unstable();
    vs.dedup();
    // Сердцевина имеет смысл только для простого замкнутого контура.
    if !(3..=6).contains(&vs.len())
        || !vs.iter().all(|&v| {
            shape
                .edges
                .iter()
                .filter(|&&(a, b)| a == v || b == v)
                .count()
                == 2
        })
    {
        return;
    }
    let (mut sx, mut sy) = (0.0f32, 0.0f32);
    for &i in &vs {
        sx += scr[i].0;
        sy += scr[i].1;
    }
    let cx = sx / vs.len() as f32;
    let cy = sy / vs.len() as f32;
    let span = (shape.x1 - shape.x0).min(shape.y1 - shape.y0);
    let r = (span * CORE_RADIUS_RATIO).max(0.75);
    let color = engine_core_color(glow_phase, engine);
    let w = frame.width as i32;
    let h = frame.height as i32;
    let y_lo = ((cy - r - 0.5).ceil() as i32).max(0);
    let y_hi = ((cy + r - 0.5).ceil() as i32 - 1).min(h - 1);
    for y in y_lo..=y_hi {
        let yc = y as f32 + 0.5;
        let dy = (yc - cy).abs();
        if dy > r {
            continue;
        }
        let half = r - dy;
        let x_lo = ((cx - half - 0.5).ceil() as i32).max(0);
        let x_hi = ((cx + half - 0.5).ceil() as i32 - 1).min(w - 1);
        if x_lo <= x_hi {
            frame.hline(x_lo, y, (x_hi - x_lo + 1) as usize, color);
        }
    }
}

/// Убрать внутренние диагонали, разбивающие грань на подтреугольники.
///
/// Некоторые грани (например, задние грани Python) содержат общее ребро
/// между двумя треугольниками одной грани. Для заливки это внутреннее
/// ребро: оно создаёт вершины степени 3 и заставляет scanline по правилу
/// «чёт-нечёт» выключать заливку на диагонали. Удаляем рёбра, оба конца
/// которых имеют степень > 2; после этого возможные усики дочищает
/// `prune_spurs`. Применяем результат только если он стал набором
/// замкнутых контуров, иначе оставляем исходный контур.
fn remove_internal_edges(edges: &mut Vec<(usize, usize)>) {
    let original = edges.clone();
    loop {
        let mut deg: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for &(a, b) in edges.iter() {
            *deg.entry(a).or_insert(0) += 1;
            *deg.entry(b).or_insert(0) += 1;
        }
        let before = edges.len();
        edges.retain(|&(a, b)| {
            !(deg.get(&a).copied().unwrap_or(0) > 2 && deg.get(&b).copied().unwrap_or(0) > 2)
        });
        if edges.len() == before {
            break;
        }
        prune_spurs(edges);
    }
    let clean = edges.iter().all(|&(a, b)| {
        let deg = |v: usize| edges.iter().filter(|&&(x, y)| x == v || y == v).count();
        deg(a) == 2 && deg(b) == 2
    });
    if !clean {
        *edges = original;
    }
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
        let mut edges = face_edges[fi].clone();
        prune_spurs(&mut edges);
        remove_internal_edges(&mut edges);
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
        let style = ship.face_style(fi);
        // Непланарный четырёхугольник даёт мерцание painter's sort: делим на
        // два треугольника для заливки, сохраняя общий стиль/грань.
        if vs.len() == 4 && plane_error > 0.002 {
            if let Some(cycle) = trace_cycle(&edges) {
                if cycle.len() == 4 {
                    for tri in [ [cycle[0], cycle[1], cycle[2]], [cycle[0], cycle[2], cycle[3]] ] {
                        let tedges = vec![(tri[0], tri[1]), (tri[1], tri[2]), (tri[2], tri[0])];
                        push_shape(&mut out, fi, tedges, scr, style);
                    }
                    continue;
                }
            }
        }
        push_shape_with(&mut out, fi, edges, scr, style, plane, plane_error);
    }
    out
}

fn trace_cycle(edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    if edges.is_empty() {
        return None;
    }
    let mut adj: std::collections::HashMap<usize, Vec<usize>> = std::collections::HashMap::new();
    for &(a, b) in edges {
        adj.entry(a).or_default().push(b);
        adj.entry(b).or_default().push(a);
    }
    if adj.values().any(|n| n.len() != 2) {
        return None;
    }
    let start = *adj.keys().min()?;
    let mut cycle = vec![start];
    let mut prev = usize::MAX;
    let mut cur = start;
    for _ in 0..adj.len() {
        let nexts = adj.get(&cur)?;
        let &next = nexts.iter().find(|&&n| n != prev)?;
        if next == start {
            break;
        }
        cycle.push(next);
        prev = cur;
        cur = next;
    }
    (cycle.len() == adj.len()).then_some(cycle)
}

fn push_shape(
    out: &mut Vec<FaceShape>,
    fi: usize,
    edges: Vec<(usize, usize)>,
    scr: &[(f32, f32, f32)],
    style: FaceStyle,
) {
    let mut vs: Vec<usize> = edges.iter().flat_map(|&(a, b)| [a, b]).collect();
    vs.sort_unstable();
    vs.dedup();
    let pts: Vec<(f32, f32, f32)> = vs.iter().map(|&i| scr[i]).collect();
    let plane = fit_plane(&pts);
    let plane_error = match plane {
        Some((aa, bb, cc)) => pts
            .iter()
            .fold(0.0f32, |e, &(x, y, iz)| e.max((aa * x + bb * y + cc - iz).abs() / iz)),
        None => f32::MAX,
    };
    push_shape_with(out, fi, edges, scr, style, plane, plane_error);
}

fn push_shape_with(
    out: &mut Vec<FaceShape>,
    fi: usize,
    edges: Vec<(usize, usize)>,
    scr: &[(f32, f32, f32)],
    style: FaceStyle,
    plane: Option<(f32, f32, f32)>,
    plane_error: f32,
) {
    let mut vs: Vec<usize> = edges.iter().flat_map(|&(a, b)| [a, b]).collect();
    vs.sort_unstable();
    vs.dedup();
    let pts: Vec<(f32, f32, f32)> = vs.iter().map(|&i| scr[i]).collect();
    let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    for &(i, j) in &edges {
        for k in [i, j] {
            x0 = x0.min(scr[k].0);
            x1 = x1.max(scr[k].0);
            y0 = y0.min(scr[k].1);
            y1 = y1.max(scr[k].1);
        }
    }
    let invz = pts.iter().map(|&(_, _, i)| i).sum::<f32>() / pts.len() as f32;
    // Небольшой сдвиг вперёд для выделенных граней: они часто лежат в том
    // же контуре/плоскости, что и корпус, и при равной средней глубине
    // painter's sort может мерцать.
    let invz = if matches!(style, FaceStyle::Hull) {
        invz
    } else {
        invz * 1.0001
    };
    out.push(FaceShape {
        fi,
        edges,
        x0,
        x1,
        y0,
        y1,
        plane,
        plane_error,
        style,
        invz,
    });
}

/// Цвет заливки грани с учётом оттенка корабля.
///
/// Оттенок накладывается на корпус и на все выделенные поверхности, но не на
/// двигатели: их цвет задан эффектом и должен оставаться прежним.
pub(super) fn face_color(style: FaceStyle, tint: Option<ShipTint>, glow_phase: f32) -> u32 {
    let tinted = |color| tint.map_or(color, |t| t.apply(color));
    match style {
        FaceStyle::Hull => tinted(SHIP_FACE_COLOR),
        FaceStyle::Color(color) => tinted(color),
        FaceStyle::Flame => FLAME_COLOR,
        FaceStyle::EngineGlow => engine_glow_color(glow_phase),
    }
}

/// Залить видимые грани непрозрачным цветом.
///
/// Каждый контур заполняется построчно по правилу «чёт-нечёт»: это корректно
/// и для нескольких петель (вырезы двигателей), и для невыпуклых граней.
/// Грани идут от дальней к ближней: у выделенных поверхностей свой цвет, и
/// они должны перекрывать корпус, оказавшийся позади них на экране. У грани
/// двигателя поверх основной заливки рисуется маленькая яркая сердцевина.
/// `glow_phase` — фаза [0, 2π) для пульсации [`FaceStyle::EngineGlow`],
/// `tint` — оттенок корабля или `None` для нейтрального металла,
/// `engine` — интенсивность сердцевины двигателя.
pub(super) fn fill_faces(
    frame: &mut Frame,
    shapes: &[FaceShape],
    scr: &[(f32, f32, f32)],
    glow_phase: f32,
    tint: Option<ShipTint>,
    engine: f32,
) {
    let w = frame.width as i32;
    let h = frame.height as i32;
    if w == 0 || h == 0 {
        return;
    }
    let mut order: Vec<&FaceShape> = shapes.iter().collect();
    order.sort_by(|a, b| {
        a.invz.total_cmp(&b.invz).then_with(|| {
            fn prio(s: FaceStyle) -> u8 {
                match s {
                    FaceStyle::Hull => 0,
                    FaceStyle::Color(_) | FaceStyle::Flame => 1,
                    FaceStyle::EngineGlow => 2,
                }
            }
            prio(a.style)
                .cmp(&prio(b.style))
                .then_with(|| a.fi.cmp(&b.fi))
        })
    });
    for shape in order {
        let color = face_color(shape.style, tint, glow_phase);
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
                    frame.hline(x_lo, y, (x_hi - x_lo + 1) as usize, color);
                }
            }
        }
        if shape.style == FaceStyle::EngineGlow {
            fill_engine_core(frame, shape, scr, glow_phase, engine);
        }
    }
}
