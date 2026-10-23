//! Справочный дамп титульных экранов оригинальной игры и версии Кладова.
//!
//! Инструмент для сверки нативной заставки ([`crate::splash`]) с оригиналом:
//! запускает оригинальный Z80-код, снимает экран (256x192) и печатает его
//! как текст, распознавая символы по шрифту ПЗУ, скопированному игрой в ОЗУ.
//!
//! Тесты помечены `#[ignore]` (они долгие и печатают много), запускаются
//! вручную:
//!
//! ```text
//! cargo test --release dumptitle -- --ignored --nocapture
//! ```
//!
//! Результат: `tmp/orig_title.ppm` и `tmp/kladov_title.ppm` — снимки экранов.
//!
//! Тест `dump_intro_rotation` дополнительно печатает СКО интро-объекта
//! (ориентацию корабля на заставке) по кадрам и замеряет скорость её
//! поворота — по ней сверена стартовая поза в [`crate::splash`].

use crate::machine::Spectrum;

fn write_ppm(path: &str, buf: &[u32], w: usize, h: usize) {
    let mut out = format!("P6\n{w} {h}\n255\n").into_bytes();
    for &p in buf {
        out.push((p >> 16) as u8);
        out.push((p >> 8) as u8);
        out.push(p as u8);
    }
    let _ = std::fs::write(path, out);
}

/// Распознать символ по битовому рисунку глифа (шрифт ПЗУ в ОЗУ игры).
fn glyph(font: &[u8], rows: &[u8; 8]) -> char {
    if rows.iter().all(|&b| b == 0) {
        return ' ';
    }
    for c in 0x20u8..0x7B {
        let off = (c as usize - 0x20) * 8;
        let g = &font[off..off + 8];
        if g == rows {
            return c as char;
        }
    }
    '#'
}

/// Распечатать экран как текст (8x8 ячейки, распознавание по шрифту).
fn dump_text(m: &Spectrum, font: &[u8]) {
    for cy in 0..24usize {
        let mut line = String::new();
        for cx in 0..32usize {
            let mut rows = [0u8; 8];
            for y in 0..8usize {
                let mut b = 0u8;
                for x in 0..8usize {
                    let (addr, bit) =
                        crate::machine::pixel_addr((cx * 8 + x) as u16, (cy * 8 + y) as u16);
                    if m.mem[addr as usize] & bit != 0 {
                        b |= 0x80 >> x;
                    }
                }
                rows[y] = b;
            }
            line.push(glyph(font, &rows));
        }
        println!("{cy:2}: |{line}|");
    }
}

#[test]
#[ignore = "справочный дамп оригинала (запускать вручную)"]
fn dump_original_title() {
    let base = env!("CARGO_MANIFEST_DIR");
    let game = std::fs::read(format!("{base}/zx_bin/elite.bin")).unwrap();
    let mut m = Spectrum::new(None);
    m.load(crate::GAME_LOAD_ADDR, &game);
    crate::install_im2_table(&mut m);
    m.cpu.pc = crate::GAME_ENTRY;
    m.cpu.sp = crate::GAME_SP;
    for _ in 0..120 {
        m.run_frame();
    }
    let mut buf = vec![0u32; 256 * 192];
    m.render_rgba(&mut buf);
    write_ppm("tmp/orig_title.ppm", &buf, 256, 192);
    let font = m.mem[0x5D70..0x5D70 + 0x2D8].to_vec();
    println!("=== ОРИГИНАЛ ===");
    dump_text(&m, &font);
    // СКО объекта «корабль на заставке»: слот objMissile = 0x6159, СКО = +0x0A.
    // Девять знаковых байт (шаг 2): 0x6163 — ось Z модели, 0x6169 — ось Y,
    // 0x616F — ось X (в мире).
    let mut prev: Option<[[f64; 3]; 3]> = None;
    for f in 0..12 {
        for _ in 0..5 {
            m.run_frame();
        }
        let c: Vec<f64> = (0..9)
            .map(|i| {
                let b = m.mem[0x6163 + 2 * i];
                let v = (b & 0x7F) as f64;
                if b & 0x80 != 0 { -v } else { v }
            })
            .collect();
        let axes = [[c[6], c[7], c[8]], [c[3], c[4], c[5]], [c[0], c[1], c[2]]];
        println!("кадр {} СКО (оси модели X,Y,Z): {:?}", 125 + f * 5, axes);
        if let Some(p) = prev {
            // Угол между ориентациями: tr(R) = 1 + 2cos(theta) для матриц поворота.
            let mut tr = 0.0;
            for i in 0..3 {
                for k in 0..3 {
                    tr += p[k][i] * axes[k][i];
                }
            }
            let cos = ((tr - 1.0) / 2.0).clamp(-1.0, 1.0);
            println!("   угол за 5 кадров: {:.1}°", cos.acos().to_degrees());
        }
        prev = Some(axes);
    }
    for n in 0..3 {
        for _ in 0..50 {
            m.run_frame();
        }
        println!("--- кадр {}", 170 + n * 50);
        dump_text(&m, &font);
    }
}

#[test]
#[ignore = "справочный дамп версии Кладова (запускать вручную)"]
fn dump_kladov_title() {
    let base = env!("CARGO_MANIFEST_DIR");
    let game = std::fs::read(format!("{base}/zx_bin/kladov.bin")).unwrap();
    let mut m = Spectrum::new(None);
    m.load(crate::KLADOV_LOAD_ADDR, &game);
    m.cpu.a = crate::KLADOV_STATE_A;
    m.cpu.f = crate::KLADOV_STATE_F;
    m.cpu.b = (crate::KLADOV_STATE_BC >> 8) as u8;
    m.cpu.c = (crate::KLADOV_STATE_BC & 0xFF) as u8;
    m.cpu.d = (crate::KLADOV_STATE_DE >> 8) as u8;
    m.cpu.e = (crate::KLADOV_STATE_DE & 0xFF) as u8;
    m.cpu.h = (crate::KLADOV_STATE_HL >> 8) as u8;
    m.cpu.l = (crate::KLADOV_STATE_HL & 0xFF) as u8;
    m.cpu.ix = crate::KLADOV_STATE_IX;
    m.cpu.iy = crate::KLADOV_STATE_IY;
    m.cpu.sp = crate::KLADOV_STATE_SP;
    m.cpu.pc = crate::KLADOV_STATE_PC;
    m.cpu.i = crate::KLADOV_I;
    m.cpu.r = crate::KLADOV_STATE_R;
    m.cpu.im = 2;
    m.cpu.iff1 = true;
    m.cpu.iff2 = true;
    for _ in 0..120 {
        m.run_frame();
    }
    let mut buf = vec![0u32; 256 * 192];
    m.render_rgba(&mut buf);
    write_ppm("tmp/kladov_title.ppm", &buf, 256, 192);
    let font = m.mem[0x5D70..0x5D70 + 0x2D8].to_vec();
    println!("=== КЛАДОВ ===");
    dump_text(&m, &font);
    for n in 0..4 {
        for _ in 0..50 {
            m.run_frame();
        }
        println!("--- кадр {}", 170 + n * 50);
        dump_text(&m, &font);
    }
    // Сырые пиксели верхних строк.
    for cy in 0..4usize {
        for y in 0..8usize {
            let mut line = String::new();
            for x in 0..256usize {
                let (addr, bit) = crate::machine::pixel_addr(x as u16, (cy * 8 + y) as u16);
                line.push(if m.mem[addr as usize] & bit != 0 { '#' } else { '.' });
            }
            println!("{cy}.{y} {line}");
        }
    }
}

/// Слот объекта «корабль на заставке» (`objMissile` = 0x6159).
///
/// СКО объекта лежит по адресу 0x6163 и состоит из 9 знаковых байт
/// (знак в бите 7), записанных с шагом 2 (между ними — байты знаков
/// для 16-битных вычислений). Каждая тройка задаёт одну ось модели в мире:
/// `0x6163` — ось Z, `0x6169` — ось Y, `0x616F` — ось X. Третью тройку игра
/// пересчитывает сама (векторное произведение двух первых, делённое на
/// 128), поэтому ортогональность сохраняется.
#[test]
#[ignore = "замер скорости вращения СКО интро-объекта"]
fn dump_intro_rotation() {
    let base = env!("CARGO_MANIFEST_DIR");
    let game = std::fs::read(format!("{base}/zx_bin/elite.bin")).unwrap();
    let mut m = Spectrum::new(None);
    m.load(crate::GAME_LOAD_ADDR, &game);
    crate::install_im2_table(&mut m);
    m.cpu.pc = crate::GAME_ENTRY;
    m.cpu.sp = crate::GAME_SP;
    let mut prev: Option<Vec<i32>> = None;
    let mut prev_frame = 0usize;
    for f in 0..800 {
        let comp: Vec<i32> = (0..9)
            .map(|i| {
                let b = m.mem[0x6163 + 2 * i];
                let v = (b & 0x7F) as i32;
                if b & 0x80 != 0 {
                    -v
                } else {
                    v
                }
            })
            .collect();
        if comp.iter().any(|&v| v != 0) {
            if let Some(p) = prev {
                if p != comp {
                    // Оси модели в мире: X = 0x616F.., Y = 0x6169.., Z = 0x6163..
                    let axes = |c: &Vec<i32>| {
                        [
                            [c[6], c[7], c[8]],
                            [c[3], c[4], c[5]],
                            [c[0], c[1], c[2]],
                        ]
                    };
                    let n = |a: [[i32; 3]; 3]| {
                        let mut o = [[0.0f64; 3]; 3];
                        for (i, row) in a.iter().enumerate() {
                            let l = ((row[0] * row[0] + row[1] * row[1] + row[2] * row[2]) as f64)
                                .sqrt();
                            for j in 0..3 {
                                o[i][j] = row[j] as f64 / l;
                            }
                        }
                        o
                    };
                    let a1 = n(axes(&p));
                    let a2 = n(axes(&comp));
                    // Раскладка: a[ось модели][компонента мира] = M[мир][ось],
                    // то есть строки матрицы поворота читаются столбцами `a`.
                    // Поворот между кадрами в мире: Rw = M2 * M1^T.
                    let mut r = [[0.0f64; 3]; 3];
                    for i in 0..3 {
                        for j in 0..3 {
                            for k in 0..3 {
                                r[i][j] += a2[k][i] as f64 * a1[k][j] as f64;
                            }
                        }
                    }
                    // Поворот в кадре модели: Rm = M1^T * M2. Он тоже
                    // постоянен (движение — одно параметрическое подгруппа:
                    // при M_n = Rw^n·M0 обе величины телескопируются) и
                    // связан с Rw сопряжением через стартовую позу. Кадр —
                    // просто другое описание того же шага; в порте
                    // используется мировой вариант.
                    let mut rm = [[0.0f64; 3]; 3];
                    for i in 0..3 {
                        for j in 0..3 {
                            for k in 0..3 {
                                rm[i][j] += a1[i][k] as f64 * a2[j][k] as f64;
                            }
                        }
                    }
                    let tr = r[0][0] + r[1][1] + r[2][2];
                    let ang = ((tr - 1.0) / 2.0).clamp(-1.0, 1.0).acos();
                    let sn = 2.0 * ang.sin();
                    let v = if sn.abs() < 1e-9 {
                        [0.0, 0.0, 0.0]
                    } else {
                        [
                            (r[2][1] - r[1][2]) / sn,
                            (r[0][2] - r[2][0]) / sn,
                            (r[1][0] - r[0][1]) / sn,
                        ]
                    };
                    let nframes = (f - prev_frame) as f64;
                    let vm = [
                        (rm[2][1] - rm[1][2]) / sn,
                        (rm[0][2] - rm[2][0]) / sn,
                        (rm[1][0] - rm[0][1]) / sn,
                    ];
                    println!(
                        "кадры {prev_frame}->{f}: мир ({:.2},{:.2},{:.2})°, модель ({:.2},{:.2},{:.2})°, всего {:.2}°",
                        v[0] * ang.to_degrees() / nframes,
                        v[1] * ang.to_degrees() / nframes,
                        v[2] * ang.to_degrees() / nframes,
                        vm[0] * ang.to_degrees() / nframes,
                        vm[1] * ang.to_degrees() / nframes,
                        vm[2] * ang.to_degrees() / nframes,
                        ang.to_degrees()
                    );
                }
            }
            prev = Some(comp);
            prev_frame = f;
        }
        m.run_frame();
    }
}

/// Матрица поворота, построенная игрой для интро-объекта
/// (`MatrixRotate` = 0xE2BC): 3 строки по 3 элемента, элемент = 2 байта
/// (нечётный — модуль со знаком в бите 7, чётный — байт знака 0x00/0xFF).
#[test]
#[ignore = "дамп матрицы поворота интро-объекта"]
fn dump_intro_matrix() {
    let base = env!("CARGO_MANIFEST_DIR");
    let game = std::fs::read(format!("{base}/zx_bin/elite.bin")).unwrap();
    let mut m = Spectrum::new(None);
    m.load(crate::GAME_LOAD_ADDR, &game);
    crate::install_im2_table(&mut m);
    m.cpu.pc = crate::GAME_ENTRY;
    m.cpu.sp = crate::GAME_SP;
    for f in 0..300 {
        m.run_frame();
        let sko: Vec<u8> = (0x6162..0x6174).map(|a| m.mem[a]).collect();
        let mat: Vec<u8> = (0xE2BC..0xE2CE).map(|a| m.mem[a]).collect();
        let sko_hex: String = sko.iter().map(|b| format!("{b:02X}")).collect();
        let mat_hex: String = mat.iter().map(|b| format!("{b:02X}")).collect();
        println!("SKO {f} {sko_hex} MAT {mat_hex}");
    }
}

/// Силуэт корабля на экране оригинала: центр, направление главной оси и
/// размахы — по кадрам (для оценки видимого движения на заставке).
#[test]
#[ignore = "замер силуэта корабля на заставке оригинала"]
fn dump_intro_silhouette() {
    let base = env!("CARGO_MANIFEST_DIR");
    let game = std::fs::read(format!("{base}/zx_bin/elite.bin")).unwrap();
    let mut m = Spectrum::new(None);
    m.load(crate::GAME_LOAD_ADDR, &game);
    crate::install_im2_table(&mut m);
    m.cpu.pc = crate::GAME_ENTRY;
    m.cpu.sp = crate::GAME_SP;
    let mut buf = vec![0u32; 256 * 192];
    let mut prev: Option<(f64, f64)> = None;
    for f in 0..240 {
        m.run_frame();
        m.render_rgba(&mut buf);
        // Область корабля: пиксельные строки 20..105 (между заголовком и
        // нижней строкой), весь экран по X.
        let mut n = 0.0f64;
        let (mut sx, mut sy) = (0.0f64, 0.0f64);
        for y in 20..105usize {
            for x in 16..240usize {
                if buf[y * 256 + x] != 0 {
                    n += 1.0;
                    sx += x as f64;
                    sy += y as f64;
                }
            }
        }
        if n < 10.0 {
            continue;
        }
        let (cx, cy) = (sx / n, sy / n);
        // Главная ось распределения точек.
        let (mut sxx, mut syy, mut sxy) = (0.0f64, 0.0f64, 0.0f64);
        for y in 20..105usize {
            for x in 16..240usize {
                if buf[y * 256 + x] != 0 {
                    let dx = x as f64 - cx;
                    let dy = y as f64 - cy;
                    sxx += dx * dx;
                    syy += dy * dy;
                    sxy += dx * dy;
                }
            }
        }
        let (sxx, syy, sxy) = (sxx / n, syy / n, sxy / n);
        let theta = 0.5 * (2.0 * sxy).atan2(sxx - syy);
        let ang = theta.to_degrees();
        // Размах вдоль главной и поперечной осей (по экстремумам проекций).
        let (mut t0, mut t1, mut u0, mut u1) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
        for y in 20..105usize {
            for x in 16..240usize {
                if buf[y * 256 + x] != 0 {
                    let dx = x as f64 - cx;
                    let dy = y as f64 - cy;
                    let t = dx * theta.cos() + dy * theta.sin();
                    let u = -dx * theta.sin() + dy * theta.cos();
                    t0 = t0.min(t);
                    t1 = t1.max(t);
                    u0 = u0.min(u);
                    u1 = u1.max(u);
                }
            }
        }
        if let Some((pa, _)) = prev {
            let mut d = ang - pa;
            if d > 90.0 {
                d -= 180.0;
            }
            if d < -90.0 {
                d += 180.0;
            }
            println!(
                "кадр {f:3}: точек {n:5.0}  центр ({cx:5.1},{cy:5.1})  ось {ang:7.2}° (Δ {d:+6.2}°)  длина {:5.1}  ширина {:5.1}",
                t1 - t0,
                u1 - u0
            );
        } else {
            println!(
                "кадр {f:3}: точек {n:5.0}  центр ({cx:5.1},{cy:5.1})  ось {ang:7.2}°  длина {:5.1}  ширина {:5.1}",
                t1 - t0,
                u1 - u0
            );
        }
        prev = Some((ang, n));
    }
}

/// Снимок экрана оригинала на заданном кадре (для сверки ракурса).
/// Номер кадра — переменная окружения `FRAME` (по умолчанию 16).
#[test]
#[ignore = "снимок экрана оригинала"]
fn dump_orig_frame() {
    let base = env!("CARGO_MANIFEST_DIR");
    let game = std::fs::read(format!("{base}/zx_bin/elite.bin")).unwrap();
    let mut m = Spectrum::new(None);
    m.load(crate::GAME_LOAD_ADDR, &game);
    crate::install_im2_table(&mut m);
    m.cpu.pc = crate::GAME_ENTRY;
    m.cpu.sp = crate::GAME_SP;
    let frame: u64 = std::env::var("FRAME").ok().and_then(|v| v.parse().ok()).unwrap_or(16);
    for f in 0..=frame {
        m.run_frame();
        if f == frame {
            let mut buf = vec![0u32; 256 * 192];
            m.render_rgba(&mut buf);
            write_ppm(&format!("tmp/emul_f{frame}.ppm"), &buf, 256, 192);
        }
    }
}

/// Проверка раскладки СКО интро-объекта.
///
/// Печатает обе тройки СКО и проверяет, что третья (по адресу 0x616F) равна
/// векторному произведению двух первых, делённому на 128: именно её игра
/// пересчитывает при построении матрицы поворота. Тем самым подтверждается,
/// что каждая тройка задаёт одну ось модели в мире.
#[test]
#[ignore = "проверка раскладки СКО интро-объекта"]
fn check_sko_layout() {
    let base = env!("CARGO_MANIFEST_DIR");
    let game = std::fs::read(format!("{base}/zx_bin/elite.bin")).unwrap();
    let mut m = Spectrum::new(None);
    m.load(crate::GAME_LOAD_ADDR, &game);
    crate::install_im2_table(&mut m);
    m.cpu.pc = crate::GAME_ENTRY;
    m.cpu.sp = crate::GAME_SP;
    for f in 0..120 {
        m.run_frame();
        if f % 20 != 0 || f < 10 {
            continue;
        }
        // 9 знаковых байт СКО: 0x6163, 0x6165, ... (шаг 2).
        let c: Vec<f64> = (0..9)
            .map(|i| {
                let b = m.mem[0x6163 + 2 * i];
                let v = (b & 0x7F) as f64;
                if b & 0x80 != 0 { -v } else { v }
            })
            .collect();
        // Тройки в памяти: T1 = 0x6163.., T2 = 0x6169.., T3 = 0x616F..
        let t1 = [c[0], c[1], c[2]];
        let t2 = [c[3], c[4], c[5]];
        let t3 = [c[6], c[7], c[8]];
        // Вариант A: тройки = оси модели (T3 = ось X, T2 = ось Y, T1 = ось Z).
        let cross = |a: [f64; 3], b: [f64; 3]| {
            [
                a[1] * b[2] - a[2] * b[1],
                a[2] * b[0] - a[0] * b[2],
                a[0] * b[1] - a[1] * b[0],
            ]
        };
        // В варианте A третья тройка = ось X; игра пишет туда векторное
        // произведение остальных двух (с точностью до знака и /128).
        let cxa = cross(t2, t1);
        // В варианте B (тройки = компоненты X,Y,Z осей) в третью тройку
        // пишется X-компонента произведения осей Y и Z.
        let ax = [c[6], c[3], c[0]];
        let ay = [c[7], c[4], c[1]];
        let az = [c[8], c[5], c[2]];
        let cxb = cross(ay, az)[0];
        // Ортогональность осей (вариант A: сами тройки; вариант B: сборка).
        let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
        println!(
            "кадр {f:3}: A: t3=({:.0},{:.0},{:.0}) T1xT2=({:.0},{:.0},{:.0})/128  |  B: X-комп=({:.1}) t3[0]={:.0}  |  ортогональность A: {:.1},{:.1}  B: {:.1},{:.1}",
            t3[0], t3[1], t3[2],
            cxa[0] / 128.0, cxa[1] / 128.0, cxa[2] / 128.0,
            cxb / 128.0, t3[0],
            dot(t1, t2), dot(t2, t3),
            dot(ax, ay), dot(ay, az),
        );
    }
}
