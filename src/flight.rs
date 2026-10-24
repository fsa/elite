//! Нативный движок космического полёта (клавиша F1).
//!
//! Это альтернатива Z80-полёту: пока движок активен, эмулятор стоит, а окно
//! целиком занимает нативный рендер. Возврат к Z80 — клавиша `1` (или `F1`).
//! Оформление повторяет титульный экран: чёрный фон, заголовок
//! `---- E L I T E ----` шрифтом сетки ZX, жёлтая подсказка внизу.
//! Объекты (корабли, станция) рисуются тем же каркасным рендером, что и
//! вращающийся корабль на заставке ([`crate::render3d`]).
//!
//! Мир — простая ньютоновская модель: у корабля есть ориентация
//! ([`Rotation`]), скорость вдоль носа и положение. Управление повторяет
//! привычные клавиши оригинала (стрелки/Kempston и `X/S/N/M`), поэтому
//! переключение между движками не меняет мышечную память.
//!
//! Что уже есть в модели:
//!
//! * звёздное поле: точки летят навстречу, и чем выше скорость, тем быстрее;
//! * непрозрачная планета с фазой освещения от солнца;
//! * солнце с ореолом;
//! * станция и несколько кораблей с собственным вращением;
//! * лазеры с перегревом и попаданием по цели;
//! * щиты, топливо, скорость и температура лазера на нижней панели.
//!
//! Движок не пишет в память Z80: возврат по `1` продолжает игру с того же
//! места, где она была поставлена на паузу. Скорость/топливо/щиты при входе
//! читаются из памяти игры как начальные значения.

use crate::display::Frame;
use crate::render3d::{self, Rotation, Screen};
use crate::shipdata::{self, Ship};

/// Цвет фона (непрозрачный чёрный).
const BG: u32 = 0xFF00_0000;
/// Белый цвет заголовка и рамок — как на заставке.
const FG: u32 = 0xFFD7_D7D7;
/// Жёлтый цвет подсказок.
const HINT: u32 = 0xFFFF_FF00;
/// Циан — линии объектов.
const CYAN: u32 = 0xFF00_FFFF;
/// Красный — лазеры и опасность.
const RED: u32 = 0xFFFF_4040;
/// Тусклый серый — второстепенные подписи.
const DIM: u32 = 0xFF80_8080;

/// Максимальная скорость в игровых единицах в секунду.
const MAX_SPEED: f32 = 1400.0;
/// Угловая скорость управления (радиан в секунду).
const ROT_SPEED: f32 = 1.15;
/// Ускорение/торможение (доля максимальной скорости в секунду).
const ACCEL: f32 = 0.42;
const DECEL: f32 = 0.65;
/// Перегрев лазера: прирост за выстрел и остывание в секунду.
const LASER_HEAT: f32 = 0.16;
const LASER_COOL: f32 = 0.30;
/// Сколько секунд виден луч лазера.
const LASER_TIME: f32 = 0.10;
/// Ближняя плоскость отсечения.
const NEAR: f32 = 0.5;

/// Начальные значения, снятые с Z80-игры при входе в нативный полёт.
#[derive(Clone, Copy, Debug)]
pub struct FlightSeed<'a> {
    /// Имя текущей системы (для строки состояния).
    pub system: &'a str,
    /// Скорость из памяти игры (`Speed`, 0..0xA0).
    pub speed: u8,
    /// Топливо (`Fuel`, 0..0xA0).
    pub fuel: u8,
    /// Передний щит (`ShieldFront`, 0..0xA0).
    pub shield: u8,
    /// Цвет планеты текущей системы (из [`crate::galaxy::color`]).
    pub planet_color: u32,
}

/// Состояние клавиш, важных для нативного полёта.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Controls {
    pub pitch_up: bool,
    pub pitch_down: bool,
    pub roll_left: bool,
    pub roll_right: bool,
    pub throttle_up: bool,
    pub throttle_down: bool,
    pub fire: bool,
}

impl Controls {
    /// Собрать управление из ZX-матрицы и Kempston.
    ///
    /// Клавиши взяты по физическим позициям (раскладка не важна):
    /// `S`/`X` или стрелки вверх/вниз — тангаж, `N`/`M` или стрелки
    /// влево/вправо — крен, `Space` — увеличить скорость, `RightShift` —
    /// уменьшить, `A`/`RightCtrl` — огонь.
    pub fn from_matrix(matrix: &[[bool; 5]; 8], kempston: &[bool; 5]) -> Self {
        let key = |row: usize, bit: usize| matrix[row][bit];
        Controls {
            // Оригинал: ControlKeyMove 3=S Up, 2=X Down, 1=M Right, 0=N Left
            pitch_up: key(1, 1) || kempston[3],    // S / Up
            pitch_down: key(0, 2) || kempston[2],  // X / Down
            roll_left: key(7, 3) || kempston[1],   // N / Left
            roll_right: key(7, 2) || kempston[0],  // M / Right
            // Оригинал: Space увеличить скорость, Symbol Shift (RightShift) уменьшить
            throttle_up: key(7, 0),                // Space
            throttle_down: key(7, 1),              // Symbol Shift
            // Оригинал: ControlKey3, бит 0 — A (Fire Laser)
            fire: key(1, 0) || kempston[4],        // A / RightCtrl
        }
    }
}

/// Тип объекта в сцене.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BodyKind {
    Ship,
    Station,
    Planet,
    Sun,
}

/// Объект сцены.
struct Body {
    kind: BodyKind,
    /// Геометрия (для кораблей и станции).
    model: Option<&'static Ship>,
    /// Положение в мировых координатах.
    pos: [f32; 3],
    /// Ориентация модели относительно мира.
    rot: Rotation,
    /// Ось собственного вращения (в мире).
    spin: [f32; 3],
    /// Скорость собственного вращения, радиан/с.
    spin_rate: f32,
    /// Радиус для проекции и попадания.
    radius: f32,
    /// Цвет каркаса или диска.
    color: u32,
    /// Таймер выстрела (для противников).
    fire_timer: f32,
}

/// Звезда в космическом пространстве.
struct Star {
    /// Положение относительно камеры/мира в упрощённой модели.
    pos: [f32; 3],
    mag: u8,
}

/// Нативный движок полёта.
pub struct Flight {
    /// Показывать ли нативный экран вместо Z80-игры.
    pub active: bool,
    started: bool,
    /// Ориентация камеры: мир → вид.
    rot: Rotation,
    /// Положение корабля в мире.
    pos: [f32; 3],
    /// Текущая скорость `0..1` от [`MAX_SPEED`].
    speed: f32,
    /// Угловые скорости с инерцией (радиан/с).
    ang_pitch: f32,
    ang_roll: f32,
    shield: f32,
    fuel: f32,
    laser_heat: f32,
    laser_timer: f32,
    damage_flash: f32,
    message_timer: f32,
    respawn_timer: f32,
    message: &'static str,
    kills: u32,
    tick: u64,
    bodies: Vec<Body>,
    stars: Vec<Star>,
    system: String,
    planet_color: u32,
    rng: u32,
}

impl Flight {
    pub fn new() -> Self {
        let mut f = Flight {
            active: false,
            started: false,
            rot: Rotation::identity(),
            pos: [0.0; 3],
            speed: 0.22,
            ang_pitch: 0.0,
            ang_roll: 0.0,
            shield: 1.0,
            fuel: 1.0,
            laser_heat: 0.0,
            laser_timer: 0.0,
            damage_flash: 0.0,
            message_timer: 0.0,
            respawn_timer: 0.0,
            message: "",
            kills: 0,
            tick: 0,
            bodies: Vec::new(),
            stars: Vec::new(),
            system: String::new(),
            planet_color: 0xFF4A80B0,
            rng: 0x1234_5678,
        };
        f.reset_scene();
        f
    }

    /// Войти в нативный полёт, подхватив начальные показания Z80-игры.
    pub fn open(&mut self, seed: FlightSeed<'_>) {
        self.active = true;
        self.system = if seed.system.is_empty() {
            "UNKNOWN".to_string()
        } else {
            seed.system.to_uppercase()
        };
        if seed.planet_color & 0xFF00_0000 != 0 {
            self.planet_color = seed.planet_color;
        }
        if !self.started {
            self.started = true;
            self.reset_scene();
        } else {
            // Цвет планеты мог смениться вместе с системой — обновляем диск.
            for b in &mut self.bodies {
                if b.kind == BodyKind::Planet {
                    b.color = self.planet_color;
                }
            }
        }
        // Скорость на станции равна нулю — нативный движок всё равно должен
        // уметь лететь, поэтому даём небольшой стартовый импульс.
        self.speed = (seed.speed as f32 / 0xA0 as f32).clamp(0.15, 1.0);
        self.fuel = (seed.fuel as f32 / 0xA0 as f32).clamp(0.0, 1.0);
        self.shield = (seed.shield as f32 / 0xA0 as f32).clamp(0.0, 1.0);
    }

    /// Вернуться к Z80-версии (клавиша `1` или повторный `F1`).
    pub fn close(&mut self) {
        self.active = false;
    }

    /// Сбросить сцену в исходное положение.
    fn reset_scene(&mut self) {
        self.rot = Rotation::identity();
        self.pos = [0.0; 3];
        self.ang_pitch = 0.0;
        self.ang_roll = 0.0;
        self.stars = make_stars(&mut self.rng, 280);
        self.bodies.clear();

        let station = shipdata::SHIPS
            .iter()
            .find(|s| s.name == "d_Coriolis")
            .or_else(|| shipdata::SHIPS.first());
        self.bodies.push(Body {
            kind: BodyKind::Station,
            model: station,
            pos: [1800.0, -420.0, 4200.0],
            rot: Rotation::identity(),
            spin: [0.0, 0.0, 1.0],
            spin_rate: 0.25,
            radius: 260.0,
            color: 0xFFA0_FFFF,
            fire_timer: 0.0,
        });

        for (name, pos, spin_rate) in [
            ("d_Viper", [-980.0, 160.0, 2500.0], 0.9),
            ("d_Cobra1", [1250.0, -260.0, 3900.0], -0.7),
            ("dSidewinderH", [420.0, 520.0, 6200.0], 0.5),
        ] {
            let model = shipdata::SHIPS.iter().find(|s| s.name == name);
            self.bodies.push(Body {
                kind: BodyKind::Ship,
                model,
                pos,
                rot: Rotation::axis_angle([0.0, 1.0, 0.0], spin_rate),
                spin: [0.2, 1.0, 0.15],
                spin_rate,
                radius: 110.0,
                color: CYAN,
                fire_timer: 2.0 + (self.rng % 100) as f32 / 40.0,
            });
        }

        // Планета и солнце — далёкие объекты, их размеры в мировых единицах
        // подобраны так, чтобы на экране они читались как диски, а не точки.
        self.bodies.push(Body {
            kind: BodyKind::Planet,
            model: None,
            pos: [-1600.0, 2300.0, 65000.0],
            rot: Rotation::identity(),
            spin: [0.0, 1.0, 0.0],
            spin_rate: 0.02,
            radius: 9000.0,
            color: self.planet_color,
            fire_timer: 0.0,
        });
        self.bodies.push(Body {
            kind: BodyKind::Sun,
            model: None,
            pos: [-90000.0, 26000.0, 130000.0],
            rot: Rotation::identity(),
            spin: [0.0; 3],
            spin_rate: 0.0,
            radius: 17000.0,
            color: 0xFFFF_D040,
            fire_timer: 0.0,
        });
    }

    /// Один шаг симуляции.
    pub fn tick(&mut self, c: Controls, dt: f32) {
        if !self.active {
            return;
        }
        self.tick += 1;
        let dt = dt.clamp(0.0, 0.1);

        // Ориентация с инерцией: набираем угловую скорость при нажатии,
        // затем затухаем (как в оригинальном Elite).
        let pitch_in = (c.pitch_down as i32 - c.pitch_up as i32) as f32;
        let roll_in = (c.roll_right as i32 - c.roll_left as i32) as f32;
        let accel = ROT_SPEED * 3.2;
        let damp = 1.0 - (4.5 * dt).min(0.9);
        self.ang_pitch += pitch_in * accel * dt;
        self.ang_roll += roll_in * accel * dt;
        self.ang_pitch *= damp;
        self.ang_roll *= damp;
        self.ang_pitch = self.ang_pitch.clamp(-ROT_SPEED * 1.4, ROT_SPEED * 1.4);
        self.ang_roll = self.ang_roll.clamp(-ROT_SPEED * 1.4, ROT_SPEED * 1.4);
        if self.ang_pitch.abs() > 1e-4 {
            self.rot = Rotation::axis_angle([1.0, 0.0, 0.0], self.ang_pitch * dt)
                .mul(&self.rot);
        }
        if self.ang_roll.abs() > 1e-4 {
            self.rot = Rotation::axis_angle([0.0, 0.0, 1.0], self.ang_roll * dt)
                .mul(&self.rot);
        }

        // Тяга. Без топлива ускорения нет, но инерция сохраняется.
        let throttle = (c.throttle_up as i32 - c.throttle_down as i32) as f32;
        if throttle > 0.0 && self.fuel > 0.0 {
            self.speed = (self.speed + ACCEL * dt).min(1.0);
            self.fuel = (self.fuel - 0.020 * dt).max(0.0);
        } else if throttle < 0.0 {
            self.speed = (self.speed - DECEL * dt).max(0.0);
        }

        // Лазер: выстрел, попадание и остывание.
        self.laser_timer = (self.laser_timer - dt).max(0.0);
        self.laser_heat = (self.laser_heat - LASER_COOL * dt).max(0.0);
        if c.fire && self.laser_heat < 0.95 && self.laser_timer <= 0.0 {
            self.fire();
        }
        self.damage_flash = (self.damage_flash - dt).max(0.0);
        self.message_timer = (self.message_timer - dt).max(0.0);
        self.respawn_timer = (self.respawn_timer - dt).max(0.0);

        // Движение вперёд вдоль носа. transpose возвращает мировое
        // направление взгляда из матрицы «мир → вид».
        let forward = self.rot.transpose().apply([0.0, 0.0, 1.0]);
        for i in 0..3 {
            self.pos[i] += forward[i] * self.speed * MAX_SPEED * dt;
        }
        self.advance_stars(forward, dt);

        // Собственное вращение и простая тактика противников.
        for b in &mut self.bodies {
            if b.spin_rate != 0.0 {
                b.rot = Rotation::axis_angle(b.spin, b.spin_rate * dt).mul(&b.rot);
            }
            if b.kind != BodyKind::Ship {
                continue;
            }
            let mut d = [
                self.pos[0] - b.pos[0],
                self.pos[1] - b.pos[1],
                self.pos[2] - b.pos[2],
            ];
            let dist = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt().max(1.0);
            let sign = if dist > 2200.0 { 1.0 } else { -1.0 };
            let step = 36.0 * dt * sign;
            for v in &mut d {
                *v /= dist;
            }
            for i in 0..3 {
                b.pos[i] += d[i] * step;
            }
            // Противник изредка стреляет, если смотрит в сторону игрока.
            b.fire_timer -= dt;
            if b.fire_timer <= 0.0 && dist < 6500.0 {
                let dot = (-d[0]) * forward[0] + (-d[1]) * forward[1] + (-d[2]) * forward[2];
                if dot > 0.35 {
                    self.shield = (self.shield - 0.045).max(0.02);
                    self.damage_flash = 0.25;
                }
                b.fire_timer = 2.2 + (next(&mut self.rng) % 180) as f32 / 50.0;
            }
        }

        if self.respawn_timer <= 0.0 && !self.bodies.iter().any(|b| b.kind == BodyKind::Ship) {
            self.spawn_enemy();
        }
    }

    /// Выстрел из лазера: попадание по кораблю рядом с перекрестием.
    fn fire(&mut self) {
        self.laser_timer = LASER_TIME;
        self.laser_heat = (self.laser_heat + LASER_HEAT).min(1.0);
        let forward = self.rot.transpose().apply([0.0, 0.0, 1.0]);
        let mut hit = None;
        let mut best = 0.0f32;
        for (i, b) in self.bodies.iter().enumerate() {
            if b.kind != BodyKind::Ship {
                continue;
            }
            let d = [
                b.pos[0] - self.pos[0],
                b.pos[1] - self.pos[1],
                b.pos[2] - self.pos[2],
            ];
            let dist = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt().max(1.0);
            let dot = (d[0] * forward[0] + d[1] * forward[1] + d[2] * forward[2]) / dist;
            // Угловой радиус цели; минимум 2° — прощаем неточное прицеливание.
            let limit = (b.radius / dist).clamp(0.0, 1.0).asin().max(0.035);
            if dot >= limit.cos() && (hit.is_none() || dist < best) {
                hit = Some(i);
                best = dist;
            }
        }
        if let Some(i) = hit {
            self.bodies.remove(i);
            self.kills += 1;
            self.message = "TARGET DESTROYED";
            self.message_timer = 1.5;
            self.respawn_timer = 4.0;
        }
    }

    fn spawn_enemy(&mut self) {
        let models = ["d_Viper", "d_Cobra1", "dSidewinderH", "d_Krait"];
        let name = models[(self.next_rng() as usize) % models.len()];
        let model = shipdata::SHIPS.iter().find(|s| s.name == name);
        let x = (self.next_rng() % 4000) as f32 - 2000.0;
        let y = (self.next_rng() % 2400) as f32 - 1200.0;
        self.bodies.push(Body {
            kind: BodyKind::Ship,
            model,
            pos: [x, y, 6500.0],
            rot: Rotation::axis_angle([0.2, 1.0, 0.1], 1.0),
            spin: [0.2, 1.0, 0.15],
            spin_rate: 0.8,
            radius: 110.0,
            color: CYAN,
            fire_timer: 2.0,
        });
    }

    fn next_rng(&mut self) -> u32 {
        // xorshift32 — детерминированный и без зависимостей.
        let mut x = self.rng.max(1);
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.rng = x;
        x
    }

    /// Нарисовать полёт на весь кадр.
    pub fn draw(&mut self, frame: &mut Frame, font: &[u8]) {
        let w = frame.width as i32;
        let h = frame.height as i32;
        frame.fill_rect(0, 0, frame.width, frame.height, BG);

        // Сетка ZX 256x192, как на заставке: заголовок и подсказка остаются
        // в тех же местах, поэтому экран полёта выглядит её продолжением.
        let s = ((frame.width / 256).min(frame.height / 192)).max(1) as i32;
        let ox = (w - 256 * s) / 2;
        let title_y = 4 * s;
        frame.draw_text(
            font,
            ox + 40 * s,
            title_y,
            "---- E L I T E ----",
            FG,
            s as usize,
        );
        let sub = "NATIVE FLIGHT";
        frame.draw_text(font, ox + 8 * s, 14 * s, sub, HINT, s as usize);
        // Имя системы — в правой части той же строки (не длиннее 11 знаков,
        // чтобы не налезть на «NATIVE FLIGHT»).
        let system: String = self.system.chars().take(11).collect();
        let sx = ox + (256 - system.len() as i32 * 8) * s;
        frame.draw_text(font, sx, 14 * s, &system, HINT, s as usize);
        // Число побед — коротко в правом верхнем углу, рядом с заголовком.
        let kills = format!("K:{}", self.kills);
        let kx = ox + (256 - kills.len() as i32 * 8) * s;
        frame.draw_text(font, kx, title_y, &kills, DIM, s as usize);

        let hud_top = h - 34 * s;
        // Строка состояния — 14-я строка сетки (14..22); вид начинается
        // ниже неё, иначе звёзды закрашивают нижнюю часть букв.
        let view_top = 24 * s;
        let view_bottom = (hud_top - 4 * s).max(view_top + 8 * s);
        let cx = w as f32 / 2.0;
        let cy = (view_top + view_bottom) as f32 / 2.0;
        let focal = ((view_bottom - view_top) as f32 * 1.15).max(200.0);
        let screen = Screen {
            cx,
            cy,
            k: focal,
            bx: 0.0,
            by: 0.0,
        };

        frame.set_clip(
            0,
            view_top,
            frame.width,
            (view_bottom - view_top).max(1) as usize,
        );
        self.draw_stars(frame, &screen);
        self.draw_bodies(frame, &screen);
        self.draw_crosshair(frame, cx, cy, s);
        if self.laser_timer > 0.0 {
            self.draw_laser(frame, cx, cy, view_bottom, s);
        }
        if self.damage_flash > 0.0 {
            frame.hline(0, view_top, frame.width, RED);
            frame.hline(0, view_bottom - 1, frame.width, RED);
            frame.vline(0, view_top, (view_bottom - view_top) as usize, RED);
            frame.vline(w - 1, view_top, (view_bottom - view_top) as usize, RED);
        }
        frame.clear_clip();

        if self.message_timer > 0.0 {
            let tw = self.message.len() as i32 * 8 * s;
            frame.draw_text(
                font,
                (w - tw) / 2,
                view_top + 4 * s,
                self.message,
                RED,
                s as usize,
            );
        }
        self.draw_hud(frame, font, s, hud_top, ox);
        let hint = "1 Z80   F1 NATIVE   ESC PAUSE";
        frame.draw_text(font, ox + 8 * s, h - 12 * s, hint, HINT, s as usize);
    }

    /// Сдвинуть звёздное поле назад относительно корабля.
    ///
    /// Звёзды хранятся в координатах, связанных с кораблём. При полёте они
    /// смещаются в направлении, противоположном движению, поэтому чем выше
    /// скорость, тем быстрее они проносятся мимо. Ушедшие за камеру звёзды
    /// рождаются заново впереди — поле остаётся бесконечным.
    fn advance_stars(&mut self, forward: [f32; 3], dt: f32) {
        let step = self.speed * MAX_SPEED * dt;
        if step.abs() < 1e-9 {
            return;
        }
        // Базис для разброса новых звёзд вокруг направления полёта.
        let up = if forward[1].abs() < 0.9 {
            [0.0, 1.0, 0.0]
        } else {
            [1.0, 0.0, 0.0]
        };
        let right = normalize(cross(up, forward));
        let top = cross(forward, right);
        let mut rng = self.rng;
        for star in &mut self.stars {
            for (i, p) in star.pos.iter_mut().enumerate() {
                *p -= forward[i] * step;
            }
            let v = self.rot.apply(star.pos);
            if v[2] > NEAR {
                continue;
            }
            // Новая звезда впереди, в конусе вокруг курса.
            let dist = 1200.0 + (next(&mut rng) as f32 / u32::MAX as f32) * 4800.0;
            let around = next(&mut rng) as f32 / u32::MAX as f32 * std::f32::consts::TAU;
            let spread = (next(&mut rng) as f32 / u32::MAX as f32).sqrt() * 0.55;
            let (sa, ca) = around.sin_cos();
            for i in 0..3 {
                star.pos[i] = forward[i] * dist
                    + (right[i] * ca + top[i] * sa) * spread * dist;
            }
            star.mag = (next(&mut rng) >> 24) as u8;
        }
        self.rng = rng;
    }

    fn draw_stars(&self, frame: &mut Frame, screen: &Screen) {
        let forward = self.rot.transpose().apply([0.0, 0.0, 1.0]);
        let speed = self.speed * MAX_SPEED;
        for star in &self.stars {
            let v = self.rot.apply(star.pos);
            if v[2] <= NEAR {
                continue;
            }
            let x = screen.cx + v[0] / v[2] * screen.k;
            let y = screen.cy - v[1] / v[2] * screen.k;
            if x < 0.0 || y < 0.0 || x >= frame.width as f32 || y >= frame.height as f32 {
                continue;
            }
            let bright = 0.25 + star.mag as f32 / 255.0 * 0.75;
            let c = 0xFF00_0000
                | ((bright * 210.0) as u32) << 16
                | ((bright * 225.0) as u32) << 8
                | (bright * 255.0) as u32;
            let (xi, yi) = (x.round() as i32, y.round() as i32);
            // Небольшой след от движения: на большой скорости звёзды
            // вытягиваются вдоль курса, как при движении камеры.
            if speed > 150.0 && star.mag > 160 {
                let age = 0.035;
                let tail = [
                    star.pos[0] + forward[0] * speed * age,
                    star.pos[1] + forward[1] * speed * age,
                    star.pos[2] + forward[2] * speed * age,
                ];
                let tv = self.rot.apply(tail);
                if tv[2] > NEAR {
                    let tx = screen.cx + tv[0] / tv[2] * screen.k;
                    let ty = screen.cy - tv[1] / tv[2] * screen.k;
                    frame.line(tx.round() as i32, ty.round() as i32, xi, yi, c & 0x80FF_FFFF);
                }
            }
            frame.plot(xi, yi, c);
            if star.mag > 210 {
                frame.plot(xi + 1, yi, c);
                frame.plot(xi - 1, yi, c);
                frame.plot(xi, yi + 1, c);
                frame.plot(xi, yi - 1, c);
            }
        }
    }

    fn draw_bodies(&self, frame: &mut Frame, screen: &Screen) {
        // Дальние объекты рисуются первыми, ближние перекрывают их.
        let mut order: Vec<(f32, usize)> = self
            .bodies
            .iter()
            .enumerate()
            .map(|(i, b)| {
                let rel = [
                    b.pos[0] - self.pos[0],
                    b.pos[1] - self.pos[1],
                    b.pos[2] - self.pos[2],
                ];
                (self.rot.apply(rel)[2], i)
            })
            .collect();
        order.sort_by(|a, b| b.0.total_cmp(&a.0));
        for (z, i) in order {
            if z <= NEAR {
                continue;
            }
            let b = &self.bodies[i];
            match b.kind {
                BodyKind::Planet => self.draw_planet(frame, b, screen),
                BodyKind::Sun => self.draw_sun(frame, b, screen),
                BodyKind::Ship | BodyKind::Station => {
                    let Some(model) = b.model else {
                        continue;
                    };
                    let rel = [
                        b.pos[0] - self.pos[0],
                        b.pos[1] - self.pos[1],
                        b.pos[2] - self.pos[2],
                    ];
                    let pos = self.rot.apply(rel);
                    let cam_rot = self.rot.mul(&b.rot);
                    render3d::draw_ship(frame, model, &cam_rot, pos, screen, b.color);
                }
            }
        }
    }

    /// Проекция центра объекта: экранные координаты и радиус диска.
    fn project(&self, b: &Body, screen: &Screen) -> Option<(f32, f32, f32, f32)> {
        let rel = [
            b.pos[0] - self.pos[0],
            b.pos[1] - self.pos[1],
            b.pos[2] - self.pos[2],
        ];
        let v = self.rot.apply(rel);
        if v[2] <= NEAR {
            return None;
        }
        let x = screen.cx + v[0] / v[2] * screen.k;
        let y = screen.cy - v[1] / v[2] * screen.k;
        let r = b.radius / v[2] * screen.k;
        Some((x, y, r, v[2]))
    }

    fn draw_planet(&self, frame: &mut Frame, b: &Body, screen: &Screen) {
        let Some((x, y, r, _)) = self.project(b, screen) else {
            return;
        };
        if r < 0.8
            || x + r < 0.0
            || y + r < 0.0
            || x - r > frame.width as f32
            || y - r > frame.height as f32
        {
            return;
        }
        let (cx, cy, r) = (x.round() as i32, y.round() as i32, r.round() as i32);
        // Освещение: направление от планеты к солнцу в экранных координатах.
        let sun = self
            .bodies
            .iter()
            .find(|s| s.kind == BodyKind::Sun)
            .map(|s| {
                let l = self
                    .rot
                    .apply([s.pos[0] - b.pos[0], s.pos[1] - b.pos[1], s.pos[2] - b.pos[2]]);
                let n = (l[0] * l[0] + l[1] * l[1]).sqrt().max(1e-3);
                (l[0] / n, -l[1] / n)
            })
            .unwrap_or((0.0, 0.0));
        let base = darken(b.color, 0.22);
        let lit = b.color;
        let bright = lighten(b.color, 1.35);
        frame.disc(cx, cy, r, base);
        // Смещённый освещённый диск оставляет ночную часть планеты тёмной.
        let ox = (sun.0 * r as f32 * 0.38).round() as i32;
        let oy = (sun.1 * r as f32 * 0.38).round() as i32;
        frame.disc(cx + ox, cy + oy, (r as f32 * 0.86) as i32, lit);
        frame.disc(
            cx + (ox as f32 * 1.25) as i32,
            cy + (oy as f32 * 1.25) as i32,
            (r as f32 * 0.30) as i32,
            bright,
        );
        frame.circle(cx, cy, r, lighten(b.color, 1.6));
    }

    fn draw_sun(&self, frame: &mut Frame, b: &Body, screen: &Screen) {
        let Some((x, y, r, _)) = self.project(b, screen) else {
            return;
        };
        if r < 0.8
            || x + r * 2.2 < 0.0
            || y + r * 2.2 < 0.0
            || x - r * 2.2 > frame.width as f32
            || y - r * 2.2 > frame.height as f32
        {
            return;
        }
        let (cx, cy, r) = (x.round() as i32, y.round() as i32, r.round().max(1.0) as i32);
        // Ореол: несколько непрозрачных дисков от тёмно-оранжевого к белому.
        frame.disc(cx, cy, (r as f32 * 2.0) as i32, 0xFF3A_1800);
        frame.disc(cx, cy, (r as f32 * 1.45) as i32, 0xFF9A_4A00);
        frame.disc(cx, cy, r, 0xFFFF_C040);
        frame.disc(cx, cy, (r as f32 * 0.55) as i32, 0xFFFF_FFF0);
    }

    fn draw_crosshair(&self, frame: &mut Frame, cx: f32, cy: f32, s: i32) {
        let (x, y) = (cx as i32, cy as i32);
        let gap = 3 * s;
        let len = 9 * s;
        frame.hline(x - gap - len, y, len as usize, CYAN);
        frame.hline(x + gap, y, len as usize, CYAN);
        frame.vline(x, y - gap - len, len as usize, CYAN);
        frame.vline(x, y + gap, len as usize, CYAN);
    }

    fn draw_laser(&self, frame: &mut Frame, cx: f32, cy: f32, bottom: i32, s: i32) {
        let x = cx as i32;
        let y = cy as i32;
        let off = 12 * s;
        frame.line(x - off, bottom, x - 2, y, RED);
        frame.line(x + off, bottom, x + 2, y, RED);
        frame.line(x - off + 2, bottom, x, y, 0xFFFF_A000);
        frame.line(x + off - 2, bottom, x, y, 0xFFFF_A000);
    }

    fn draw_hud(&self, frame: &mut Frame, font: &[u8], s: i32, top: i32, ox: i32) {
        // Рамка панели и разделительная линия — как у приборной панели
        // оригинала, но в графике заставки (белые линии по чёрному).
        frame.hline(0, top, frame.width, FG);
        // Четыре прибора в две колонки по две строки: при 16:10 и масштабе
        // x5 в нижнюю полосу иначе не помещаются четыре строки шрифта.
        let rows = [
            [
                ("SPEED", self.speed, CYAN),
                (
                    "SHIELD",
                    self.shield,
                    if self.shield < 0.25 { RED } else { CYAN },
                ),
            ],
            [
                ("FUEL", self.fuel, if self.fuel < 0.2 { RED } else { HINT }),
                (
                    "LASER",
                    1.0 - self.laser_heat,
                    if self.laser_heat > 0.75 { RED } else { HINT },
                ),
            ],
        ];
        let col = |c: i32| ox + c * 8 * s;
        for (row, pair) in rows.iter().enumerate() {
            let y = top + (2 + row as i32 * 8) * s;
            for (half, (label, frac, fill)) in pair.iter().enumerate() {
                let base = if half == 0 { 1 } else { 17 };
                frame.draw_text(font, col(base), y, label, FG, s as usize);
                // Между самым длинным названием (SHIELD, 6 знаков) и шкалой
                // остаётся пустая колонка.
                let bar_x = col(base + 7);
                let bar_w = 4 * 8 * s;
                draw_bar(frame, *frac, bar_x, y + s, bar_w, 6 * s, *fill);
                let val_x = col(base + 12);
                let pct = format!("{:3}", (*frac * 100.0).round().clamp(0.0, 100.0) as i32);
                frame.draw_text(font, val_x, y, &pct, FG, s as usize);
            }
        }
        // Тонкая разделительная черта между колонками приборов.
        frame.vline(col(16), top + 2 * s, 20 * s as usize, DIM);
    }
}

impl Default for Flight {
    fn default() -> Self {
        Self::new()
    }
}

/// Горизонтальная шкала: белая рамка, заливка по значению `0..1`.
fn draw_bar(frame: &mut Frame, frac: f32, x: i32, y: i32, w: i32, h: i32, color: u32) {
    if w <= 2 || h <= 2 {
        return;
    }
    frame.fill_rect(x, y, w as usize, h as usize, FG);
    frame.fill_rect(x + 1, y + 1, (w - 2) as usize, (h - 2) as usize, BG);
    let fill = ((w - 2) as f32 * frac.clamp(0.0, 1.0)).round() as i32;
    if fill > 0 {
        frame.fill_rect(x + 1, y + 1, fill as usize, (h - 2) as usize, color);
    }
}

/// Осветлить цвет `0xFFRRGGBB` (множитель > 1).
fn lighten(c: u32, k: f32) -> u32 {
    let ch = |shift: u32| {
        ((((c >> shift) & 0xFF) as f32 * k).round().clamp(0.0, 255.0) as u32) << shift
    };
    0xFF00_0000 | ch(16) | ch(8) | ch(0)
}

/// Затемнить цвет `0xFFRRGGBB` (множитель < 1).
fn darken(c: u32, k: f32) -> u32 {
    lighten(c, k)
}

/// Детерминированное звёздное поле: звёзды вокруг корабля, движущиеся назад при полёте.
fn make_stars(rng: &mut u32, count: usize) -> Vec<Star> {
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let a = next(rng) as f32 / u32::MAX as f32 * std::f32::consts::TAU;
        let z = next(rng) as f32 / u32::MAX as f32 * 2.0 - 1.0;
        let r = (1.0 - z * z).sqrt();
        let d = 1200.0 + (next(rng) as f32 / u32::MAX as f32) * 4800.0;
        out.push(Star {
            pos: [r * a.cos() * d, r * a.sin() * d, z * d],
            mag: (next(rng) >> 24) as u8,
        });
    }
    out
}

/// Векторное произведение.
fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Нормировать вектор (нулевой остаётся нулевым).
fn normalize(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len < 1e-6 {
        [0.0; 3]
    } else {
        [v[0] / len, v[1] / len, v[2] / len]
    }
}

/// Шаг xorshift32 для генератора звёзд.
fn next(rng: &mut u32) -> u32 {
    let mut x = (*rng).max(1);
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    *rng = x;
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    fn open() -> Flight {
        let mut f = Flight::new();
        f.open(FlightSeed {
            system: "Lave",
            speed: 0x50,
            fuel: 0xA0,
            shield: 0xA0,
            planet_color: 0xFF40_80C0,
        });
        f
    }

    fn keys(pressed: &[(usize, usize)]) -> ([[bool; 5]; 8], [bool; 5]) {
        let mut m = [[false; 5]; 8];
        for &(r, b) in pressed {
            m[r][b] = true;
        }
        (m, [false; 5])
    }

    #[test]
    fn controls_follow_original_keys_and_arrows() {
        // Оригинал: S — вверх, X — вниз, N/M — крен, Space/Shift — скорость,
        // A — огонь (ControlKeyMove и ControlKey3).
        let (m, k) = keys(&[(1, 1), (0, 2), (7, 3), (7, 2), (7, 0), (7, 1), (1, 0)]);
        let c = Controls::from_matrix(&m, &k);
        assert!(c.pitch_up && c.pitch_down);
        assert!(c.roll_left && c.roll_right);
        assert!(c.throttle_up && c.throttle_down);
        assert!(c.fire);
        // Q/E в оригинале — не рыскание (Escape Pod / ECM), полёт их не трогает.
        let (m, _) = keys(&[(2, 0), (2, 2)]);
        let c = Controls::from_matrix(&m, &[false; 5]);
        assert_eq!(c, Controls::default());
        // Kempston: вверх/вниз/влево/вправо/огонь.
        let c = Controls::from_matrix(&[[false; 5]; 8], &[true, true, true, true, true]);
        assert!(c.roll_right && c.roll_left && c.pitch_down && c.pitch_up && c.fire);
    }

    #[test]
    fn pitch_changes_orientation_and_forward_motion() {
        let mut f = open();
        let before = f.rot;
        f.tick(
            Controls {
                pitch_up: true,
                throttle_up: true,
                ..Controls::default()
            },
            1.0 / 50.0,
        );
        assert!(f.rot.m != before.m, "тангаж не изменил ориентацию");
        assert!(f.speed > 0.22, "тяга не увеличила скорость");
        let start = f.pos;
        for _ in 0..50 {
            f.tick(Controls::default(), 1.0 / 50.0);
        }
        assert!(f.pos != start, "корабль не движется вперёд");
    }

    #[test]
    fn stars_move_faster_with_speed() {
        let mut fast = open();
        fast.speed = 1.0;
        if let Some(s) = fast.stars.first_mut() {
            s.pos = [0.0, 0.0, 2000.0];
        }
        let start = fast.stars[0].pos;
        fast.tick(Controls::default(), 0.1);
        let moved = fast.stars[0].pos;
        let step = (0..3).map(|i| (moved[i] - start[i]).abs()).sum::<f32>();
        assert!(step > 1.0, "на полной скорости звёзды не движутся");
        assert!(moved[2] < start[2], "звёзды должны двигаться навстречу камере");
    }

    #[test]
    fn flight_screen_is_opaque_and_styled_like_splash() {
        let mut f = open();
        let font = vec![0xFFu8; 0x300];
        let mut frame = Frame::new(1280, 960);
        f.draw(&mut frame, &font);
        // Фон полностью непрозрачный, как на заставке.
        assert!(
            frame.pixels().iter().all(|&p| p & 0xFF00_0000 != 0),
            "на экране полёта есть прозрачные пиксели"
        );
        assert_eq!(frame.pixels()[0], BG);
        assert!(
            frame.pixels().iter().any(|&p| p == FG),
            "нет заголовка/приборов"
        );
        assert!(
            frame.pixels().iter().any(|&p| p == HINT),
            "нет жёлтой подсказки"
        );
        assert!(
            frame.pixels().iter().any(|&p| p == CYAN),
            "объекты и перекрестие не нарисованы"
        );
        assert!(
            frame.pixels().iter().any(|&p| p == 0xFF40_80C0),
            "цвет планеты не взят из начальных данных системы"
        );
        if let Ok(path) = std::env::var("ELITE_DUMP_FLIGHT") {
            write_ppm(&path, frame.pixels(), frame.width, frame.height);
        }
    }

    /// Сохранить кадр в PPM (P6) для визуальной проверки.
    fn write_ppm(path: &str, px: &[u32], w: usize, h: usize) {
        let mut out = format!("P6\n{w} {h}\n255\n").into_bytes();
        for &p in px {
            out.extend_from_slice(&[(p >> 16) as u8, (p >> 8) as u8, p as u8]);
        }
        if let Some(dir) = std::path::Path::new(path).parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        std::fs::write(path, out).unwrap();
    }

    #[test]
    fn firing_destroys_target_ahead() {
        let mut f = open();
        // Ставим первый корабль точно по курсу, чтобы попадание было
        // предсказуемым (сцена сдвигается только в тесте).
        if let Some(b) = f.bodies.iter_mut().find(|b| b.kind == BodyKind::Ship) {
            b.pos = [0.0, 0.0, 2500.0];
        }
        let before = f.bodies.iter().filter(|b| b.kind == BodyKind::Ship).count();
        f.tick(
            Controls {
                fire: true,
                ..Controls::default()
            },
            1.0 / 50.0,
        );
        let after = f.bodies.iter().filter(|b| b.kind == BodyKind::Ship).count();
        assert_eq!(after, before - 1, "выстрел по цели прямо по курсу не попал");
        assert_eq!(f.kills, 1);
        assert!(f.laser_heat > 0.0);
    }

    #[test]
    fn closing_returns_to_z80() {
        let mut f = open();
        assert!(f.active);
        f.close();
        assert!(!f.active);
    }
}
