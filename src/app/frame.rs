//! Один кадр игры: ввод, эмуляция, звук, отрисовка и показ в окне.

use std::num::NonZeroU32;
use std::rc::Rc;

use winit::window::Window;

use super::App;
use crate::cockpit;
use crate::display;
use crate::font::ZX_FONT;
use crate::storage;
use crate::ui::menu::draw_pause_menu;
use crate::ui::present::blit_zx_frame;

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

    /// Один кадр: ввод, эмуляция, звук, отрисовка и показ.
    pub(super) fn frame(&mut self) {
        let Some(window) = self.window.clone() else {
            return;
        };
        // Перехват игровых сообщений — только пока открыт нативный вид
        // полёта. Переход `false → true` сбрасывает накопленную строку,
        // чтобы на новом открытии вида не висело старое сообщение.
        let capture = self.flight.active && !self.menu;
        if capture && !self.machine.game_msg.enabled {
            self.machine.game_msg.clear();
        }
        self.machine.game_msg.enabled = capture;
        // Область показа и HD-слой нативного рендера — в разрешении окна.
        // Слой очищается каждый кадр: нативные хуки рисуют в него заново
        // во время эмуляции, показ накладывает слой поверх ZX-кадра.
        let size = window.inner_size();
        let layout = display::layout(size.width, size.height);
        if self.hdmap.active || self.shop.active || self.machine.intro_seen || self.flight.active {
            // Отдельный нативный экран (карта/закупка/заставка/полёт): HD-слой
            // во всё окно. Карта, закупка и заставка ставят игру на паузу; под
            // экраном полёта эмулятор продолжает работать. Слой непрозрачный и
            // покрывает окно целиком, поэтому композитится в угол (0, 0).
            self.machine
                .hd
                .resize(size.width as usize, size.height as usize);
            self.machine.hd.clear(display::TRANSPARENT);
            if self.flight.active {
                // Экран полёта: рисует то, что посчитал Z80, поэтому эмулятор
                // продолжает работать, а ввод идёт в игру. На паузе (меню)
                // симуляция замирает.
                if !self.menu {
                    let frames = self.emulation_frames();
                    self.forward_input();
                    self.machine.run_frames(frames);
                    self.follow_flight_screen();
                    self.flight.sync_from_z80(&self.machine.mem);
                    self.flight.sync_game_msg(&self.machine.game_msg);
                    for _ in 0..frames {
                        self.machine.game_msg.next_frame();
                    }
                    self.flight.tick(frames as f32 / 50.0);
                    self.play_frame_audio();
                } else {
                    self.machine.release_all_keys();
                    self.machine.release_joystick();
                }
            } else if self.machine.intro_seen {
                // Нативная заставка: вращающаяся тестовая фигура, ZX-кадр не
                // показывается. Эмуляция продолжается, чтобы Z80 шёл по своему
                // циклу (запуск игры делает `native::start_game`).
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
                self.open_pending_flight();
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
        if self.flight.active {
            // Экран полёта: нативный вид в верхней части окна (по данным Z80)
            // и тот же кокпит, что на заставке, — внизу.
            let g = cockpit::panel_geometry(size.width, size.height);
            self.flight.draw(&mut self.machine.hd, &g, ZX_FONT);
            let values = {
                let mut values = cockpit::KLADOV_COCKPIT.read(&self.machine.mem);
                values.blink = (self.machine.frames / 16) % 2 == 0;
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
        } else if self.hdmap.active {
            self.hdmap.draw(&mut self.machine.hd, ZX_FONT);
        } else if self.shop.active {
            self.shop
                .draw(&mut self.machine.hd, ZX_FONT, &self.machine.mem);
        } else if self.machine.intro_seen {
            // Нативная заставка; внизу — тот же кокпит, что в полёте.
            // Заголовок с фигурой занимают верхнюю область, поэтому не
            // заходят под панель.
            let g = cockpit::panel_geometry(size.width, size.height);
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
        // Отдельный нативный экран (карта/закупка/заставка/полёт)
        // показывается вместо ZX-кадра: слой непрозрачный и покрывает всё окно.
        if self.hdmap.active || self.shop.active || self.machine.intro_seen || self.flight.active {
            self.machine.hd.composite_onto(&mut out, ow, oh, 0, 0);
            window.pre_present_notify();
            let _ = out.present();
            return;
        }
        // Целочисленное масштабирование: каждый пиксель ZX — квадрат
        // scale x scale, картинка строго без смаза.
        blit_zx_frame(src, &mut out, ow, oh, l);
        // HD-слой нативного рендера поверх масштабированного ZX-кадра.
        self.machine
            .hd
            .composite_onto(&mut out, ow, oh, l.x as usize, l.y as usize);
        window.pre_present_notify();
        let _ = out.present();
    }
}
