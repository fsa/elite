//! Один кадр игры: ввод, эмуляция, звук, отрисовка и показ в окне.

use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::Instant;

use winit::window::Window;

use super::App;
use crate::cockpit;
use crate::display;
use crate::font::ZX_FONT;
use crate::state::Screen;
use crate::storage;
use crate::ui::menu::draw_pause_menu;
use crate::ui::present::{blit_zx_frame, draw_debug_panel};
use crate::{native, world};

impl App<'_> {
    /// Сколько кадров Z80 прогнать за один кадр показа. `F12` включает
    /// ускорение вдвое.
    pub(super) fn emulation_frames(&self) -> u64 {
        if self.accelerated { 2 } else { 1 }
    }

    /// Синтезировать и отдать звук последнего кадра эмулятора.
    fn play_frame_audio(&mut self) {
        if self.audio.is_none() && self.wav_samples.is_none() {
            return;
        }
        self.machine.render_audio(&mut self.audio_buf);
        if let Some(audio) = &self.audio {
            audio.push(&self.audio_buf);
        }
        if let Some(wav) = &mut self.wav_samples {
            wav.extend_from_slice(&self.audio_buf);
        }
    }

    /// Синхронизировать нативный полёт с состоянием Z80 после кадров
    /// эмулятора. Под картой полёт не рисуется, но продолжает жить: при
    /// закрытии карты не должно быть скачка звёзд и переходов.
    fn sync_flight_after_frames(&mut self, frames: u64) {
        if !self.flight.active && self.flight.transition_kind().is_none() {
            return;
        }
        // Под картой игра специально находится на экране локальной карты
        // (`SHORT RANGE CHART`), поэтому не закрываем нативный полёт по её
        // состоянию — он должен пережить открытие и закрытие карты.
        // Пока нативный вид скрыт (показ эмулятора во время анимации),
        // продолжаем вести состояние перехода, но не закрываем вид по экрану.
        if !self.hdmap.active {
            self.follow_flight_screen();
        }
        self.flight.sync_from_z80(&self.machine.mem);
        // Повтор текста в этом кадре продлевает показ сообщения.
        self.flight.advance_messages(frames);
        self.flight.sync_game_msg(&mut self.machine.game_msg);
        self.flight.tick(frames as f32 / 50.0);
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

    /// Один кадр: ввод, эмуляция, звук, отрисовка и показ.
    ///
    /// Выбор того, что отображается, определяется централизованным
    /// состоянием игры ([`GameState`]).
    pub(super) fn frame(&mut self) {
        let Some(window) = self.window.clone() else {
            return;
        };

        // На заставке нативные альтернативы и захват сообщений не нужны.
        if self.machine.intro_seen {
            if self.market.native {
                self.market.disable();
            }
            self.close_intro_screens();
        }

        // Текущее состояние до шага кадра — от него зависит ввод, эмуляция
        // и перехват сообщений.
        self.sync_state();
        // Заглушка `DOCKED` живёт примерно секунду, затем экран возвращается
        // к тому, что был открыт до неё.
        self.docked.expire(Instant::now());
        let capture = matches!(self.state.screen, Screen::Flight | Screen::Animation(_))
            && !self.state.menu;
        self.machine.game_msg.set_capture(capture);

        let size = window.inner_size();
        let layout = self.zx_layout(size.width, size.height);
        let (game_w, game_h) = self.game_size(size.width, size.height);
        let is_alt_screen = self.docked.is_active()
            || !matches!(self.state.screen, Screen::Emulator(_));

        // HD-слой нативного рендера очищается каждый кадр.
        if is_alt_screen {
            self.machine.hd.resize(game_w as usize, game_h as usize);
            self.machine.hd.clear(display::TRANSPARENT);
        } else {
            self.machine.hd.resize(layout.w as usize, layout.h as usize);
            self.machine.hd.clear(display::TRANSPARENT);
        }

        // Эмуляция и ввод зависят от текущего экрана. Пока показана заглушка
        // `DOCKED`, эмулятор продолжает работать и получает переданное ему
        // нажатие, но нативные экраны под заглушкой не меняются.
        if self.docked.is_active() {
            self.machine.release_all_keys();
            self.machine.release_joystick();
            if !self.state.menu {
                let frames = self.emulation_frames();
                self.forward_input();
                self.machine.run_frames(frames);
                self.play_frame_audio();
            }
        } else {
            match self.state.screen {
                Screen::GalacticMap => {
                    self.machine.release_all_keys();
                    self.machine.release_joystick();
                    if !self.state.menu {
                        let frames = self.emulation_frames();
                        self.machine.run_frames(frames);
                        self.open_pending_status();
                        self.open_pending_sysinfo();
                        self.open_pending_market();
                        self.sync_flight_after_frames(frames);
                        self.play_frame_audio();
                    }
                }
                Screen::EquipShop => {
                    self.machine.release_all_keys();
                    self.machine.release_joystick();
                }
                Screen::PilotStatus | Screen::PlanetInfo | Screen::Market => {
                    if !self.state.menu {
                        self.apply_input();
                        let frames = self.emulation_frames();
                        self.machine.run_frames(frames);
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
                        self.play_frame_audio();
                    } else {
                        self.machine.release_all_keys();
                        self.machine.release_joystick();
                    }
                }
                Screen::Flight | Screen::Animation(_) => {
                    if !self.state.menu {
                        let frames = self.emulation_frames();
                        self.forward_input();
                        self.machine.run_frames(frames);
                        self.sync_flight_after_frames(frames);
                        self.play_frame_audio();
                    } else {
                        self.machine.release_all_keys();
                        self.machine.release_joystick();
                    }
                }
                Screen::Intro => {
                    self.splash.tick();
                    self.machine.release_all_keys();
                    self.machine.release_joystick();
                    if !self.state.menu {
                        self.machine.run_frames(self.emulation_frames());
                        // `Space` запустил игру: эмулятор уже перешёл в режим
                        // капитана своей веткой старта. Проверяем это по состоянию
                        // Z80 и только после этого показываем нативный экран —
                        // пока идёт заставка, он был бы сразу закрыт.
                        self.open_pending_status();
                    }
                }
                Screen::Emulator(_) => {
                    if self.state.menu {
                        self.machine.release_all_keys();
                        self.machine.release_joystick();
                    } else {
                        self.apply_input();
                        self.machine.run_frames(self.emulation_frames());
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
                        self.play_frame_audio();
                    }
                }
            }
        }

        self.mouse_wheel_up = false;
        self.mouse_wheel_down = false;
        self.machine.render_rgba(&mut self.buf);

        // Состояние могло измениться за шаг кадра (открытие/закрытие экранов).
        self.sync_state();

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
                    if self.state.menu {
                        draw_pause_menu(&mut self.machine.hd, ZX_FONT, self.menu_sel, self.machine.intro_seen);
                    }
                }
                Screen::Flight | Screen::Animation(_) => {
                    let g = cockpit::panel_geometry(game_w, game_h);
                    self.flight.draw(&mut self.machine.hd, &g, ZX_FONT);
                    let values = {
                        let mut values = cockpit::KLADOV_COCKPIT.read(&self.machine.mem);
                        values.blink = (self.machine.frames / 16) % 2 == 0;
                        if matches!(
                            self.flight.transition_kind(),
                            Some(crate::flight::TransitionKind::Launch)
                        ) {
                            values.radar = cockpit::RadarContacts::empty();
                            values.compass = cockpit::Compass::centered();
                        }
                        values
                    };
                    let style = cockpit::CockpitStyle {
                        line: self.flight.cockpit_color(),
                        bar: self.flight.cockpit_bar_color(),
                    };
                    self.cockpit.draw_with_geom(
                        &mut self.machine.hd,
                        &g,
                        ZX_FONT,
                        style,
                        &values,
                    );
                    if self.state.menu {
                        draw_pause_menu(&mut self.machine.hd, ZX_FONT, self.menu_sel, false);
                    }
                }
                Screen::EquipShop => {
                    self.shop
                        .draw(&mut self.machine.hd, ZX_FONT, &self.machine.mem);
                    self.draw_danger_half();
                    if self.state.menu {
                        draw_pause_menu(&mut self.machine.hd, ZX_FONT, self.menu_sel, self.machine.intro_seen);
                    }
                }
                Screen::PilotStatus => {
                    self.status.open(&self.machine.mem, crate::status::KLADOV_STATUS);
                    self.status.draw(&mut self.machine.hd, ZX_FONT);
                    self.draw_danger_half();
                    if self.state.menu {
                        draw_pause_menu(&mut self.machine.hd, ZX_FONT, self.menu_sel, true);
                    }
                }
                Screen::PlanetInfo => {
                    self.sysinfo.refresh(&self.machine.mem, &self.state.planet);
                    self.sysinfo.draw(&mut self.machine.hd, ZX_FONT);
                    self.draw_danger_half();
                    if self.state.menu {
                        draw_pause_menu(&mut self.machine.hd, ZX_FONT, self.menu_sel, true);
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
                        draw_pause_menu(&mut self.machine.hd, ZX_FONT, self.menu_sel, true);
                    }
                }
                Screen::Emulator(orig) => {
                    if !orig.is_cockpit_view() {
                        self.draw_danger_half();
                    }
                    if self.state.menu {
                        draw_pause_menu(&mut self.machine.hd, ZX_FONT, self.menu_sel, false);
                    }
                }
            }
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
            Screen::Emulator(_) if !self.docked.is_active() => {
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
            draw_debug_panel(src, &mut out, ow, oh, save, self.debug_view);
        }
        window.pre_present_notify();
        let _ = out.present();
    }
}
