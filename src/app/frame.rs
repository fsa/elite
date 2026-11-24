//! Один кадр игры: ввод, эмуляция, звук, отрисовка и показ в окне.

use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::Instant;

use winit::window::Window;

use super::{App, Z80_FRAME_TIME};
use crate::cockpit;
use crate::display;
use crate::flight;
use crate::font::ZX_FONT;
use crate::galaxy;
use crate::state::Screen;
use crate::storage;
use crate::ui::menu::draw_pause_menu;
use crate::ui::present::{blit_zx_frame, draw_debug_panel};
use crate::{native, world};

/// Длительность одной фазы наложения солнца и планеты на компасе.
///
/// Отсчёт идёт по настенным часам, а не по кадрам Z80: в паузе эмулятор стоит,
/// а наложение должно продолжать мигать. 250 мс на фазу — полный цикл за
/// полсекунды, как удобно различать глазами; частота не зависит от FPS.
pub(super) const COMPASS_BLINK_MS: u128 = 250;

impl App<'_> {
    /// Сколько фиксированных 50 Гц кадров Z80 накопилось за реальное время.
    ///
    /// Эмулятор привязан к настенным часам, а не к частоте отрисовки. Это
    /// позволяет рендерить чаще 50 Гц и не дёргать игру при пропусках кадров
    /// рендера: Z80 просто догонит накопленное время несколькими шагами.
    pub(super) fn take_emulation_frames(&mut self, now: std::time::Instant) -> u64 {
        let mut dt = now.saturating_duration_since(self.last_tick);
        self.last_tick = now;
        // При паузе/подвисании не копим огромный долг.
        let cap = std::time::Duration::from_millis(100);
        if dt > cap {
            dt = cap;
        }
        self.z80_accum += dt;
        let mut frames = 0u64;
        while self.z80_accum >= Z80_FRAME_TIME {
            self.z80_accum -= Z80_FRAME_TIME;
            frames += 1;
        }
        if self.speed_multiplier > 1 {
            frames = frames.saturating_mul(self.speed_multiplier.into());
        }
        // Не даём одному кадру рендера превращаться в минутный догон.
        frames.min(10)
    }

    /// Синтезировать и отдать звук последнего кадра (эмулятор + эффекты).
    ///
    /// Аудио выдаётся на каждый кадр рендера фиксированным буфером 20 мс,
    /// чтобы вывод не зависал при пропусках/избытке шагов Z80. Если Z80 не
    /// выполнялся, держим текущий уровень бипера.
    fn play_frame_audio(&mut self, run_emulator: bool) {
        if self.audio.is_none() && self.wav_samples.is_none() && !self.sfx.is_playing() {
            return;
        }
        self.audio_buf.clear();
        if run_emulator {
            self.machine.render_audio(&mut self.audio_buf);
        } else {
            self.machine.render_audio_hold(&mut self.audio_buf);
        }
        self.sfx.mix(&mut self.audio_buf);
        if let Some(audio) = &self.audio {
            audio.push(&self.audio_buf);
        }
        if let Some(wav) = &mut self.wav_samples {
            wav.extend_from_slice(&self.audio_buf);
        }
    }

    /// Синхронизировать нативный полёт с состоянием Z80 после кадров
    /// эмулятора. Под нативными информационными экранами полёт не рисуется,
    /// но симуляция продолжает жить; кнопки управления при этом в игру не
    /// уходят (см. `App::forward_input`).
    pub(super) fn sync_flight_after_frames(&mut self, frames: u64) {
        let in_space_flight = !self.machine.intro_seen
            && !flight::z80_is_docked(&self.machine.mem)
            && (flight::z80_in_cockpit(&self.machine.mem)
                || flight::z80_in_transition(&self.machine.mem));
        if !self.flight.active && self.flight.transition_kind().is_none() && !in_space_flight {
            return;
        }
        // Под нативными информационными экранами в полёте эмулятор остаётся
        // в режиме кабины и продолжает симуляцию. В этом случае не закрываем
        // нативный полёт по экрану Z80.
        let info_overlay = self.hdmap.active
            || self.status.active
            || self.sysinfo.active
            || self.market.native
            || self.shop.active;
        if !(info_overlay && in_space_flight) {
            self.follow_flight_screen();
        }
        if flight::z80_player_dead(&self.machine.mem) {
            // Оригинал блокирует управление, но оставляет последнюю угловую
            // скорость: без этого обломки продолжают ехать в сторону последнего
            // поворота корабля вместо того, чтобы лететь от игрока.
            flight::z80_stop_rotation(&mut self.machine.mem);
        }
        // Флаг J-драйва живёт меньше одного прохода игры, поэтому ловим его в
        // живой памяти. Всё остальное берём из снимка на входе в `GameMain`:
        // граница кадра эмулятора может пройти в середине обновления объектов,
        // и прямой опрос памяти перемешал бы старые и новые координаты.
        self.flight.observe_live_fast_scroll(&self.machine.mem);
        self.flight.sync_from_z80(self.machine.game_state());
        // Повтор текста в этом кадре продлевает показ сообщения.
        self.flight.advance_messages(frames);
        self.flight.sync_game_msg(&mut self.machine.game_msg);
        self.flight.tick(frames as f32 / 50.0);
    }

    /// Индикатор ускорения эмуляции (`F12`) слева от радара.
    ///
    /// Множитель рисуется тем же шрифтом ПЗУ, что подписи панели, в клетке,
    /// зеркальной паре `ES` (`x2` слева, `ES` справа). Обычно множитель равен
    /// единице и ничего не рисуется, чтобы не засорять кадр. Индикатор
    /// относится к панели кокпита, поэтому он не должен появляться на карте,
    /// информационных экранах, заставке или экране эмулятора.
    pub(super) fn draw_speed_indicator(&mut self) {
        if !matches!(self.state.screen, Screen::Flight | Screen::Animation(_)) {
            return;
        }
        if self.speed_multiplier == 1 {
            return;
        }
        let (w, h) = (self.machine.hd.width as u32, self.machine.hd.height as u32);
        let g = cockpit::panel_geometry(w, h);
        let cell = g.speed_cell();
        if cell.is_empty() {
            return;
        }
        let scale = (cell.h / 8).min(cell.w / 16).max(1) as usize;
        let x = cell.x + (cell.w - 16 * scale as i32) / 2;
        let y = cell.y + (cell.h - 8 * scale as i32) / 2;
        let text = format!("x{}", self.speed_multiplier);
        self.machine
            .hd
            .draw_text(ZX_FONT, x, y, &text, cockpit::FRAME_COLOR, scale);
    }

    /// Полукруг индикатора опасности в правом верхнем углу вспомогательного
    /// экрана (не вид из кабины). В оригинале он есть только в полёте: на
    /// станции индикатор погашен.
    fn draw_danger_half(&mut self) {
        if self.state.is_docked() {
            return;
        }
        let mem = &self.machine.mem;
        let (w, h) = (self.machine.hd.width as u32, self.machine.hd.height as u32);
        let margin = cockpit::panel_geometry(w, h).margin;
        cockpit::draw_danger_half(&mut self.machine.hd, mem, margin, cockpit::LINE);
    }

    /// Отсчёт гиперпрыжка в левом верхнем углу нативного информационного экрана.
    ///
    /// В виде из кабины число рисует сам `flight::render`. Под нативными
    /// экранами ZX-кадр с родным отсчётом не показан, поэтому здесь выводится
    /// только то же число в той же клетке у левой и верхней рамок. Пункт
    /// назначения и подпись вида остаются только в кабине.
    pub(super) fn draw_jump_countdown(&mut self) {
        let raw = self
            .machine
            .mem
            .get(flight::Z80_JUMP_COUNTDOWN)
            .copied()
            .unwrap_or(0);
        if raw == 0
            || self.machine.intro_seen
            || flight::z80_is_docked(&self.machine.mem)
            || !flight::z80_in_transition(&self.machine.mem)
        {
            return;
        }
        let (w, h) = (self.machine.hd.width as u32, self.machine.hd.height as u32);
        let g = cockpit::panel_geometry(w, h);
        let scale = (h as f32 / 300.0).floor().max(1.0) as usize;
        let frame = cockpit::LINE.max(1);
        let text = flight::countdown_number(raw).to_string();
        self.machine.hd.draw_text(
            ZX_FONT,
            g.margin + frame + 8 * scale as i32,
            g.margin + frame + 8 * scale as i32,
            &text,
            flight::countdown_color(),
            scale,
        );
    }

    /// Шаг плавного перемещения курсора карты по удерживаемым клавишам
    /// направления с постепенным наращиванием скорости.
    pub(super) fn advance_hdmap_cursor(&mut self) {
        if self.hdmap.active
            && !self.menu
            && !self.docked.is_active()
            && !self.hdmap.is_searching()
        {
            let left = self.kempston[1] || self.matrix[7][3];
            let right = self.kempston[0] || self.matrix[7][2];
            let up = self.kempston[3] || self.matrix[1][1];
            let down = self.kempston[2] || self.matrix[0][2];
            let dx_dir = (if right { 1.0 } else { 0.0 }) - (if left { 1.0 } else { 0.0 });
            let dy_dir = (if down { 1.0 } else { 0.0 }) - (if up { 1.0 } else { 0.0 });
            if dx_dir != 0.0 || dy_dir != 0.0 {
                if self.hdmap_move_speed <= 0.0 {
                    self.hdmap_move_speed = 0.25;
                } else {
                    self.hdmap_move_speed = (self.hdmap_move_speed + 0.06).min(2.0);
                }
                self.hdmap_move_rem_x += dx_dir * self.hdmap_move_speed;
                self.hdmap_move_rem_y += dy_dir * self.hdmap_move_speed;
                let dx = self.hdmap_move_rem_x.trunc() as i32;
                let dy = self.hdmap_move_rem_y.trunc() as i32;
                if dx != 0 || dy != 0 {
                    self.hdmap_move_rem_x -= dx as f32;
                    self.hdmap_move_rem_y -= dy as f32;
                    self.hdmap.move_by(dx, dy);
                    self.sync_hdmap_target();
                }
            } else {
                self.hdmap_move_speed = 0.0;
                self.hdmap_move_rem_x = 0.0;
                self.hdmap_move_rem_y = 0.0;
            }
        } else {
            self.hdmap_move_speed = 0.0;
            self.hdmap_move_rem_x = 0.0;
            self.hdmap_move_rem_y = 0.0;
        }
    }

    /// На заставке нативные альтернативы и захват сообщений не нужны.
    /// Редактор сохранений закрывается только здесь: в игре его открывает
    /// `Alt+E` или кнопка, и сбрасывать флаг каждый кадр нельзя.
    pub(super) fn sync_intro_state(&mut self) {
        if !self.machine.intro_seen {
            return;
        }
        if self.market.native {
            self.market.disable();
        }
        self.close_intro_screens();
        self.save_editor_open = false;
    }

    /// Один кадр: ввод, эмуляция, звук, отрисовка и показ.
    ///
    /// Выбор того, что отображается, определяется централизованным
    /// состоянием игры ([`GameState`]).
    pub(super) fn frame(&mut self) {
        let Some(window) = self.window.clone() else {
            return;
        };

        self.sync_intro_state();

        // Станция по умолчанию принадлежит нативным экранам: как только игра
        // пристыковалась и управление не отдано эмулятору явно, замораживаем
        // Z80 и открываем нативную страницу капитана.
        self.maybe_enter_native_station();
        self.ensure_native_flight_in_space();
        // Текущее состояние до шага кадра — от него зависит ввод, эмуляция
        // и перехват сообщений.
        self.sync_state();
        // Заглушка `DOCKED` живёт примерно секунду, затем экран возвращается
        // к тому, что был открыт до неё. Во время паузы время игры стоит.
        if !self.pause_active {
            self.docked.expire(Instant::now());
        }
        let capture = matches!(self.state.screen, Screen::Flight | Screen::Animation(_))
            && !self.state.menu
            && !self.save_editor_open;
        self.machine.game_msg.set_capture(capture);

        let size = window.inner_size();
        let layout = self.zx_layout(size.width, size.height);
        let (game_w, game_h) = self.game_size(size.width, size.height);
        let is_alt_screen = self.station_native
            || self.pending_station_native.is_some()
            || self.docked.is_active()
            || self.save_editor_open
            || !matches!(self.state.screen, Screen::Emulator(_));

        // HD-слой нативного рендера очищается каждый кадр.
        if is_alt_screen {
            self.machine.hd.resize(game_w as usize, game_h as usize);
            self.machine.hd.clear(display::TRANSPARENT);
        } else {
            self.machine.hd.resize(layout.w as usize, layout.h as usize);
            self.machine.hd.clear(display::TRANSPARENT);
        }

        self.update_speed_hold();
        let z80_frames = self.take_emulation_frames(std::time::Instant::now());

        // Эмуляция и ввод зависят от текущего экрана. В нативном режиме
        // станции эмулятор стоит на экране капитана и больше не исполняется:
        // все данные уже доступны нативным экранам, ввод в Z80 не уходит.
        let mut emulator_ran = false;
        if self.pause_active {
            // Z80 не исполняется и не получает ввод: экран заморожен, а
            // сервисные клавиши уже разобраны в `key_event`.
            self.machine.release_all_keys();
            self.machine.release_joystick();
            self.pending_key = None;
            self.play_frame_audio(false);
        } else if self.save_editor_open {
            self.machine.release_all_keys();
            self.machine.release_joystick();
            self.pending_key = None;
            self.play_frame_audio(false);
        } else if self.station_native {
            self.machine.release_all_keys();
            self.machine.release_joystick();
            self.pending_key = None;
            self.play_frame_audio(false);
        // Пока показана заглушка `DOCKED`, эмулятор продолжает работать, но
        // не получает нажатий клавиш, а нативные экраны под заглушкой не
        // меняются.
        } else if self.docked.is_active() {
            self.machine.release_all_keys();
            self.machine.release_joystick();
            self.pending_key = None;
            if !self.state.menu && z80_frames > 0 {
                self.machine.run_frames(z80_frames);
                emulator_ran = true;
                self.play_frame_audio(true);
            }
        } else {
            match self.state.screen {
                Screen::GalacticMap => {
                    if !self.state.menu {
                        let in_flight = !self.state.intro
                            && self.state.location == crate::state::Location::Space;
                        if self.pending_station_native.is_some() {
                            self.forward_input();
                        } else if self.pending_key.is_some() {
                            self.forward_pending_key();
                        } else if in_flight {
                            self.apply_input();
                        } else {
                            self.machine.release_all_keys();
                            self.machine.release_joystick();
                        }
                        if z80_frames > 0 {
                            self.machine.run_frames(z80_frames);
                            emulator_ran = true;
                            self.switch_to_flight_on_transition();
                            self.open_pending_status();
                            self.open_pending_sysinfo();
                            self.open_pending_market();
                            self.sync_flight_after_frames(z80_frames);
                            self.play_frame_audio(true);
                        }
                    } else {
                        self.machine.release_all_keys();
                        self.machine.release_joystick();
                    }
                }
                Screen::EquipShop => {
                    if !self.state.menu {
                        if self.pending_station_native.is_some() {
                            self.forward_input();
                            if z80_frames > 0 {
                                self.machine.run_frames(z80_frames);
                                emulator_ran = true;
                                self.play_frame_audio(true);
                            }
                        } else {
                            self.machine.release_all_keys();
                            self.machine.release_joystick();
                        }
                    } else {
                        self.machine.release_all_keys();
                        self.machine.release_joystick();
                    }
                }
                Screen::PilotStatus | Screen::PlanetInfo | Screen::Market => {
                    if !self.state.menu {
                        self.apply_input();
                        if z80_frames > 0 {
                            self.machine.run_frames(z80_frames);
                            emulator_ran = true;
                            self.switch_to_flight_on_transition();
                            self.open_pending_status();
                            self.open_pending_sysinfo();
                            self.open_pending_market();
                            self.open_pending_shop();
                            self.open_pending_flight();
                            self.follow_native_screens();
                            if self.machine.save_requested {
                                self.machine.save_requested = false;
                                storage::do_save(&mut self.machine, self.opts, &mut self.last_path);
                            }
                            self.sync_flight_after_frames(z80_frames);
                            self.play_frame_audio(true);
                        }
                    } else {
                        self.machine.release_all_keys();
                        self.machine.release_joystick();
                    }
                }
                Screen::Flight | Screen::Animation(_) => {
                    if !self.state.menu {
                        self.forward_input();
                        if z80_frames > 0 {
                            self.machine.run_frames(z80_frames);
                            emulator_ran = true;
                            self.sync_flight_after_frames(z80_frames);
                            self.play_frame_audio(true);
                        }
                    } else {
                        self.machine.release_all_keys();
                        self.machine.release_joystick();
                    }
                }
                Screen::Intro => {
                    self.splash.tick();
                    self.machine.release_all_keys();
                    self.machine.release_joystick();
                    if !self.state.menu && z80_frames > 0 {
                        self.machine.run_frames(z80_frames);
                        emulator_ran = true;
                        self.open_pending_status();
                    }
                }
                Screen::Emulator(_) => {
                    if self.state.menu {
                        self.machine.release_all_keys();
                        self.machine.release_joystick();
                    } else {
                        self.apply_input();
                        if z80_frames > 0 {
                            self.machine.run_frames(z80_frames);
                            emulator_ran = true;
                            self.switch_to_flight_on_transition();
                            self.open_pending_status();
                            self.open_pending_sysinfo();
                            self.open_pending_market();
                            self.open_pending_shop();
                            self.open_pending_flight();
                            self.follow_native_screens();
                            if self.machine.save_requested {
                                self.machine.save_requested = false;
                                storage::do_save(&mut self.machine, self.opts, &mut self.last_path);
                            }
                            self.play_frame_audio(true);
                        }
                    }
                }
            }
        }

        // `L` был выставлен в матрицу на этом кадре: пора заморозить Z80 и
        // передать управление нативному экрану станции.
        if emulator_ran {
            self.finish_pending_station_native();
            self.track_jump_countdown();
        }

        // Хуки старта одноразовые: как только Z80 вошёл в игру, повторное
        // нажатие `Space` на вернувшейся заставке должно ставить их заново.
        // Выход в эмулятор с титра тоже одноразовый: новая заставка начинается
        // с нативного рендера.
        if !self.machine.intro_seen {
            self.start_requested = false;
            self.splash_emulator = false;
        }
        // Переход закончился — блокировка родного ролика больше не нужна.
        self.sync_emulator_transition_lock();

        self.mouse_wheel_up = false;
        self.mouse_wheel_down = false;
        self.machine.render_rgba(&mut self.buf);

        // Состояние могло измениться за шаг кадра (открытие/закрытие экранов,
        // новая стыковка). После него снова проверяем станционный режим.
        self.maybe_enter_native_station();
        self.ensure_native_flight_in_space();
        self.sync_state();

        self.advance_hdmap_cursor();

        // Отрисовка зависит от текущего экрана. Заглушка `DOCKED` временно
        // перекрывает нативный экран, который был открыт до неё.
        if self.docked.is_active() {
            self.docked.draw(&mut self.machine.hd, ZX_FONT);
            if self.state.menu {
                draw_pause_menu(&mut self.machine.hd, ZX_FONT, self.menu_sel, self.machine.intro_seen);
            }
        } else {
            match self.state.screen {
                Screen::GalacticMap => {
                    self.hdmap.draw(&mut self.machine.hd, ZX_FONT);
                    self.draw_danger_half();
                    self.draw_jump_countdown();
                    if self.state.menu {
                        draw_pause_menu(&mut self.machine.hd, ZX_FONT, self.menu_sel, self.machine.intro_seen);
                    }
                }
                Screen::Flight | Screen::Animation(_) => {
                    let g = cockpit::panel_geometry(game_w, game_h);
                    self.flight.draw(&mut self.machine.hd, &g, ZX_FONT);
                    let values = {
                        // Панель читает тот же снимок прохода, что и flight:
                        // иначе индикаторы и 3D-сцена показывают разные моменты.
                        let mut values = cockpit::KLADOV_COCKPIT.read(self.machine.game_state());
                        values.blink = (self.machine.frames / 16) % 2 == 0;
                        // В межпланетном пространстве мигают «стороны света»,
                        // внутри системы — только наложенные круги солнца и
                        // планеты. После вылета из межпланетного пространства
                        // фаза больше не нужна: компас возвращается к обычным
                        // цветам и не мигает.
                        values.compass_blink_phase = if values.in_free_space
                            || values.compass_bodies_overlap()
                        {
                            (self.visual_phase_start.elapsed().as_millis() / COMPASS_BLINK_MS) % 2
                                == 0
                        } else {
                            true
                        };
                        if matches!(
                            self.flight.transition_kind(),
                            Some(crate::flight::TransitionKind::Launch)
                        ) {
                            values.radar = cockpit::RadarContacts::empty();
                            values.compass = cockpit::Compass::centered();
                            values.compass_planet = cockpit::CompassCircle::hidden();
                            values.compass_station = cockpit::CompassCircle::hidden();
                            values.compass_sun = cockpit::CompassCircle::hidden();
                        }
                        values
                    };
                    let style = cockpit::CockpitStyle {
                        line: self.flight.cockpit_color(),
                        bar: self.flight.cockpit_bar_color(),
                    };
                    self.cockpit.draw_with_geom_views(
                        &mut self.machine.hd,
                        &g,
                        ZX_FONT,
                        style,
                        &values,
                        self.radar_view,
                        self.compass_view,
                    );
                    if self.state.menu {
                        draw_pause_menu(
                            &mut self.machine.hd,
                            ZX_FONT,
                            self.menu_sel,
                            self.machine.intro_seen,
                        );
                    }
                }
                Screen::EquipShop => {
                    self.shop
                        .draw(&mut self.machine.hd, ZX_FONT, &self.machine.mem);
                    self.draw_danger_half();
                    self.draw_jump_countdown();
                    if self.state.menu {
                        draw_pause_menu(&mut self.machine.hd, ZX_FONT, self.menu_sel, self.machine.intro_seen);
                    }
                }
                Screen::PilotStatus => {
                    self.status.open(&self.machine.mem, crate::status::KLADOV_STATUS);
                    self.status.draw(&mut self.machine.hd, ZX_FONT);
                    self.draw_danger_half();
                    self.draw_jump_countdown();
                    if self.state.menu {
                        draw_pause_menu(
                            &mut self.machine.hd,
                            ZX_FONT,
                            self.menu_sel,
                            self.machine.intro_seen,
                        );
                    }
                }
                Screen::PlanetInfo => {
                    self.sysinfo.refresh(&self.machine.mem, galaxy::KLADOV_GALAXY);
                    self.sysinfo.draw(&mut self.machine.hd, ZX_FONT);
                    self.draw_danger_half();
                    self.draw_jump_countdown();
                    if self.state.menu {
                        draw_pause_menu(
                            &mut self.machine.hd,
                            ZX_FONT,
                            self.menu_sel,
                            self.machine.intro_seen,
                        );
                    }
                }
                Screen::Market => {
                    self.market.refresh(
                        &self.machine.mem,
                        crate::market::KLADOV_MARKET,
                        self.state.is_docked(),
                        &self.state.system,
                    );
                    self.market.draw(&mut self.machine.hd, ZX_FONT);
                    self.draw_danger_half();
                    self.draw_jump_countdown();
                    if self.state.menu {
                        draw_pause_menu(&mut self.machine.hd, ZX_FONT, self.menu_sel, self.machine.intro_seen);
                    }
                }
                Screen::Intro => {
                    let g = cockpit::panel_geometry(game_w, game_h);
                    self.splash.draw_in(&mut self.machine.hd, ZX_FONT, &g);
                    self.cockpit.draw_with_geom(
                        &mut self.machine.hd,
                        &g,
                        ZX_FONT,
                        cockpit::CockpitStyle::COBRA_MK3,
                        &self.splash_values,
                    );
                    if self.state.menu {
                        draw_pause_menu(
                            &mut self.machine.hd,
                            ZX_FONT,
                            self.menu_sel,
                            self.machine.intro_seen,
                        );
                    }
                }
                Screen::Emulator(_) => {
                    if self.state.menu {
                        draw_pause_menu(
                            &mut self.machine.hd,
                            ZX_FONT,
                            self.menu_sel,
                            self.machine.intro_seen,
                        );
                    }
                }
            }
        }
        self.draw_speed_indicator();
        // В паузе поверх замороженного кадра показывается только короткая
        // надпись по центру; сам кадр и нативный слой остаются на месте.
        if self.pause_active {
            crate::ui::message::draw_banner(&mut self.machine.hd, ZX_FONT, "PAUSE");
        }

        self.present(&window, &layout);
    }

    /// Показать кадр: ZX-кадр 256x192 целым множителем `l.scale` по центру
    /// окна (letterbox), поверх — непрозрачные пиксели HD-слоя.
    fn present(&mut self, window: &Rc<Window>, l: &display::Layout) {
        let size = window.inner_size();
        let (w, h) = (size.width, size.height);
        if w == 0 || h == 0 {
            return;
        }
        let Some(surface) = self.surface.as_mut() else {
            return;
        };
        if surface
            .resize(NonZeroU32::new(w).unwrap(), NonZeroU32::new(h).unwrap())
            .is_err()
        {
            return;
        }
        let src = &self.buf;
        let Ok(mut out) = surface.buffer_mut() else {
            return;
        };
        let (ow, oh) = (w as usize, h as usize);
        // Фон окна — чёрные поля вокруг области 4:3.
        out.fill(0);
        // Альтернативный/нативный экран (включая заставку и нативный полёт)
        // показывается вместо ZX-кадра; сырой эмуляторный экран — с наложением
        // HD-слоя поверх масштабированного ZX-кадра.
        match self.state.screen {
            Screen::Emulator(_) if !self.station_native && !self.docked.is_active() => {
                blit_zx_frame(src, &mut out, ow, oh, l);
                self.machine.hd.composite_onto(
                    &mut out,
                    ow,
                    oh,
                    l.x as usize,
                    l.y as usize,
                );
            }
            _ => {
                self.machine.hd.composite_onto(&mut out, ow, oh, 0, 0);
            }
        }
        // Режим отладки: справа — кадр эмулятора 1:1, кнопки режима и
        // сводка/дамп блока сохранения.
        if self.opts.debug {
            let layout = world::save_layout_kladov();
            let save = native::commander_block(&self.machine, layout);
            // Кнопка редактора — только в игре: `intro_seen` взведён на заставке.
            draw_debug_panel(src, &mut out, ow, oh, save, self.debug_view, !self.machine.intro_seen);
        }
        if self.save_editor_open {
            let layout = world::save_layout_kladov();
            let save = native::commander_block(&self.machine, layout);
            crate::ui::present::draw_save_editor_fullscreen(
                &mut out,
                ow,
                oh,
                save,
                self.save_editor_sel,
                self.save_editor_editing,
                &self.save_editor_input,
                self.save_editor_name_editing,
                &self.save_editor_name_input,
            );
        }
        window.pre_present_notify();
        let _ = out.present();
    }
}
