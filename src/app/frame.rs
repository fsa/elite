//! Один кадр игры: ввод, эмуляция, звук, отрисовка и показ в окне.

use std::num::NonZeroU32;
use std::rc::Rc;

use winit::window::Window;

use super::App;
use crate::cockpit;
use crate::display;
use crate::flight;
use crate::font::ZX_FONT;
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
        if !self.flight.active {
            return;
        }
        // Под картой игра специально находится на экране локальной карты
        // (`SHORT RANGE CHART`), поэтому не закрываем нативный полёт по её
        // состоянию — он должен пережить открытие и закрытие карты.
        if !self.hdmap.active {
            self.follow_flight_screen();
        }
        self.flight.sync_from_z80(&self.machine.mem);
        // Повтор текста в этом кадре продлевает показ сообщения.
        self.flight.advance_messages(frames);
        self.flight.sync_game_msg(&mut self.machine.game_msg);
        self.flight.tick(frames as f32 / 50.0);
    }

    /// Показывать ли нативный экран капитана вместо оригинального экрана
    /// `L`: отрисовка включается, пока Z80 находится в режиме показа
    /// капитана (`Z80_SCREEN_PILOT_INFO`).
    fn show_native_status(&self) -> bool {
        self.status.active
            && !self.machine.intro_seen
            && self.machine.mem[flight::Z80_WHAT_ON_SCREEN] == flight::Z80_SCREEN_PILOT_INFO
    }

    /// Показывать ли нативный экран информации о планете вместо оригинального
    /// экрана `P` (`Z80_SCREEN_INFO_PLANET`).
    fn show_native_sysinfo(&self) -> bool {
        self.sysinfo.active
            && !self.machine.intro_seen
            && self.machine.mem[flight::Z80_WHAT_ON_SCREEN] == flight::Z80_SCREEN_INFO_PLANET
    }

    /// Показывать ли нативный экран цен вместо оригинального экрана рынка.
    /// Экран доступен и в полёте (оригинальная команда `K` тоже работает не
    /// только на станции); торговать можно только на станции — это проверяет
    /// сам [`crate::market::Market`].
    fn show_native_market(&self) -> bool {
        self.market.native
            && !self.machine.intro_seen
            && self.machine.mem[flight::Z80_WHAT_ON_SCREEN] == flight::Z80_SCREEN_MARKET
    }

    /// Полукруг индикатора опасности в правом верхнем углу вспомогательного
    /// экрана с рамкой (карта, экран капитана). В оригинале он есть только в
    /// полёте: на станции индикатор погашен.
    fn draw_danger_half(&mut self) {
        let mem = &self.machine.mem;
        if crate::flight::z80_is_docked(mem) {
            return;
        }
        let (w, h) = (self.machine.hd.width as u32, self.machine.hd.height as u32);
        let margin = cockpit::panel_geometry(w, h).margin;
        cockpit::draw_danger_half(&mut self.machine.hd, mem, margin, cockpit::LINE);
    }

    /// Один кадр: ввод, эмуляция, звук, отрисовка и показ.
    pub(super) fn frame(&mut self) {
        let Some(window) = self.window.clone() else {
            return;
        };
        // Перехват игровых сообщений — только пока открыт нативный вид
        // полёта (режимы View). На заставке, карте, станции и в меню он
        // выключен. Смена состояния сбрасывает буфер и очереди, чтобы на
        // новом экране не висело сообщение с прошлого.
        if self.market.native && self.machine.intro_seen {
            self.market.disable();
        }
        // На заставке нативные экраны капитана и планеты не показываются;
        // сбрасываем и их состояние, чтобы новый старт не открылся поверх.
        if self.machine.intro_seen {
            self.close_status();
            self.close_sysinfo();
        }
        let show_market = self.show_native_market();
        let show_status = self.show_native_status();
        let show_sysinfo = self.show_native_sysinfo();

        let capture = self.flight.active
            && !self.hdmap.active
            && !self.shop.active
            && !show_status
            && !show_sysinfo
            && !show_market
            && !self.machine.intro_seen
            && !self.menu;
        self.machine.game_msg.set_capture(capture);
        // Область показа и HD-слой нативного рендера — в разрешении окна.
        // Слой очищается каждый кадр: нативные хуки рисуют в него заново
        // во время эмуляции, показ накладывает слой поверх ZX-кадра.
        let size = window.inner_size();
        let layout = self.zx_layout(size.width, size.height);
        // Нативные экраны рисуются в игровой области (без отладочной полосы).
        let (game_w, game_h) = self.game_size(size.width, size.height);
        if self.hdmap.active
            || self.shop.active
            || show_status
            || show_sysinfo
            || show_market
            || self.machine.intro_seen
            || self.flight.active
        {
            // Отдельный нативный/альтернативный экран: HD-слой во всё окно.
            self.machine.hd.resize(game_w as usize, game_h as usize);
            self.machine.hd.clear(display::TRANSPARENT);
            if self.hdmap.active {
                self.machine.release_all_keys();
                self.machine.release_joystick();
                if !self.menu {
                    let frames = self.emulation_frames();
                    self.machine.run_frames(frames);
                    self.open_pending_sysinfo();
                    self.sync_flight_after_frames(frames);
                    self.play_frame_audio();
                }
            } else if self.shop.active {
                self.machine.release_all_keys();
                self.machine.release_joystick();
            } else if show_status || show_sysinfo {
                // Нативные экраны капитана и информации о планете: только
                // альтернативная отрисовка, ввод получает эмулятор.
                if !self.menu {
                    self.apply_input();
                    let frames = self.emulation_frames();
                    self.machine.run_frames(frames);
                    self.open_pending_sysinfo();
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
            } else if show_market {
                // Нативная отрисовка экрана рынка: эмулятор остаётся в режиме
                // `K` и получает обычный ввод (экран без собственного интерактива).
                if !self.menu {
                    self.apply_input();
                    let frames = self.emulation_frames();
                    self.machine.run_frames(frames);
                    self.open_pending_sysinfo();
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
            } else if self.flight.active {
                if !self.menu {
                    let frames = self.emulation_frames();
                    self.forward_input();
                    self.machine.run_frames(frames);
                    self.sync_flight_after_frames(frames);
                    self.play_frame_audio();
                } else {
                    self.machine.release_all_keys();
                    self.machine.release_joystick();
                }
            } else if self.machine.intro_seen {
                self.splash.tick();
                self.machine.release_all_keys();
                self.machine.release_joystick();
                if !self.menu {
                    self.machine.run_frames(self.emulation_frames());
                }
            } else {
                self.machine.release_all_keys();
                self.machine.release_joystick();
            }
        } else {
            // Игровой режим: HD-слой в разрешении области показа; нативный
            // кокпит рисуется на заставке и на экране полёта.
            self.machine.hd.resize(layout.w as usize, layout.h as usize);
            self.machine.hd.clear(display::TRANSPARENT);
            if self.menu {
                // В меню игра не выполняется, клавиши не попадают в эмулятор.
                self.machine.release_all_keys();
                self.machine.release_joystick();
            } else {
                self.apply_input();
                self.machine.run_frames(self.emulation_frames());
                self.open_pending_shop();
                self.open_pending_sysinfo();
                self.open_pending_flight();
                self.follow_native_screens();
                if self.machine.save_requested {
                    self.machine.save_requested = false;
                    storage::do_save(&mut self.machine, self.opts, &mut self.last_path);
                }
                self.play_frame_audio();
            }
        }
        // Сбрасываем однократные события мыши
        self.mouse_wheel_up = false;
        self.mouse_wheel_down = false;
        self.machine.render_rgba(&mut self.buf);
        if self.hdmap.active {
            self.hdmap.draw(&mut self.machine.hd, ZX_FONT);
            self.draw_danger_half();
        } else if self.flight.active {
            // Экран полёта: нативный вид в верхней части окна (по данным Z80)
            // и тот же кокпит, что на заставке, — внизу.
            let g = cockpit::panel_geometry(game_w, game_h);
            self.flight.draw(&mut self.machine.hd, &g, ZX_FONT);
            let values = {
                let mut values = cockpit::KLADOV_COCKPIT.read(&self.machine.mem);
                values.blink = (self.machine.frames / 16) % 2 == 0;
                if matches!(self.flight.transition_kind(), Some(crate::flight::TransitionKind::Launch)) {
                    values.radar = cockpit::RadarContacts::empty();
                    values.compass = cockpit::Compass::centered();
                }
                values
            };
            let style = cockpit::CockpitStyle {
                line: self.flight.cockpit_color(),
                bar: self.flight.cockpit_bar_color(),
            };
            self.cockpit
                .draw_with_geom(&mut self.machine.hd, &g, ZX_FONT, style, &values);
            if self.menu {
                draw_pause_menu(&mut self.machine.hd, ZX_FONT, self.menu_sel, false);
            }
        } else if self.shop.active {
            self.shop
                .draw(&mut self.machine.hd, ZX_FONT, &self.machine.mem);
        } else if show_status {
            self.status.draw(&mut self.machine.hd, ZX_FONT);
            self.draw_danger_half();
            if self.menu {
                draw_pause_menu(&mut self.machine.hd, ZX_FONT, self.menu_sel, true);
            }
        } else if show_sysinfo {
            self.sysinfo.draw(&mut self.machine.hd, ZX_FONT);
            if self.menu {
                draw_pause_menu(&mut self.machine.hd, ZX_FONT, self.menu_sel, true);
            }
        } else if show_market {
            self.market
                .refresh(&self.machine.mem, crate::market::KLADOV_MARKET);
            self.market.draw(&mut self.machine.hd, ZX_FONT);
        } else if self.machine.intro_seen {
            // Нативная заставка; внизу — тот же кокпит, что в полёте.
            // Заголовок с фигурой занимают верхнюю область, поэтому не
            // заходят под панель.
            let g = cockpit::panel_geometry(game_w, game_h);
            self.splash.draw_in(&mut self.machine.hd, ZX_FONT, &g);
            self.cockpit.draw_with_geom(
                &mut self.machine.hd,
                &g,
                ZX_FONT,
                cockpit::CockpitStyle::COBRA_MK3,
                &self.splash_values,
            );
            if self.menu {
                draw_pause_menu(&mut self.machine.hd, ZX_FONT, self.menu_sel, true);
            }
        } else if self.menu {
            // Меню паузы — в HD-слой, чтобы текст был чётким в любом окне.
            draw_pause_menu(&mut self.machine.hd, ZX_FONT, self.menu_sel, false);
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
        let show_market = self.show_native_market();
        let show_status = self.show_native_status();
        let show_sysinfo = self.show_native_sysinfo();
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
        // Отдельный нативный/альтернативный экран показывается вместо
        // ZX-кадра: слой непрозрачный и покрывает всё окно.
        if self.hdmap.active
            || self.shop.active
            || show_status
            || show_sysinfo
            || show_market
            || self.machine.intro_seen
            || self.flight.active
        {
            self.machine.hd.composite_onto(&mut out, ow, oh, 0, 0);
        } else {
            // Целочисленное масштабирование: каждый пиксель ZX — квадрат
            // scale x scale, картинка строго без смаза.
            blit_zx_frame(src, &mut out, ow, oh, l);
            // HD-слой нативного рендера поверх масштабированного ZX-кадра.
            self.machine
                .hd
                .composite_onto(&mut out, ow, oh, l.x as usize, l.y as usize);
        }
        // Режим отладки: справа — кадр эмулятора 1:1 и дамп блока сохранения.
        if self.opts.debug {
            let layout = world::save_layout_kladov();
            let save = native::commander_block(&self.machine, layout);
            draw_debug_panel(src, &mut out, ow, oh, save);
        }
        window.pre_present_notify();
        let _ = out.present();
    }
}
