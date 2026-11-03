//! Жизненный цикл окна winit и события ввода.

use std::process::exit;
use std::rc::Rc;
use std::time::Instant;

use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::keyboard::{ModifiersState, PhysicalKey};
use winit::window::{Fullscreen, Window, WindowId};

use super::{App, FRAME_TIME};
use crate::display;
use crate::ui::present::{SRC_H, SRC_W};

impl App<'_> {
    fn create_window(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attrs = Window::default_attributes()
            .with_title("Elite — F1: полёт, Esc: меню, F10: карта, F4: закупка, F11: полный экран, F12: ускорение x2, Ctrl+Q: выход")
            .with_inner_size(LogicalSize::new(4.0 * SRC_W as f64, 4.0 * SRC_H as f64));
        let window = match event_loop.create_window(attrs) {
            Ok(w) => Rc::new(w),
            Err(e) => {
                eprintln!("cannot open window: {e}");
                exit(1);
            }
        };
        if self.opts.fullscreen {
            window.set_fullscreen(Some(Fullscreen::Borderless(None)));
        }
        let context = match Context::new(window.clone()) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("cannot init softbuffer: {e}");
                exit(1);
            }
        };
        let surface = match Surface::new(&context, window.clone()) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("cannot create surface: {e}");
                exit(1);
            }
        };
        self.window = Some(window);
        self.context = Some(context);
        self.surface = Some(surface);
        self.next_frame = Instant::now();
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.create_window(event_loop);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Focused(false) => {
                // При потере фокуса не «залипаем» на нажатых клавишах.
                self.matrix = [[false; 5]; 8];
                self.kempston = [false; 5];
                self.ctrl = false;
                self.alt = false;
            }
            WindowEvent::ModifiersChanged(m) => {
                self.ctrl = m.state().contains(ModifiersState::CONTROL);
                self.alt = m.state().contains(ModifiersState::ALT);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    let pressed = event.state == ElementState::Pressed;
                    self.key_event(code, pressed, event.repeat);
                }
                if self.quit {
                    event_loop.exit();
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                // Позиция мыши нужна карте для ЛКМ; храним её и когда карта
                // закрыта, чтобы первый клик после открытия уже работал.
                self.hdmap.pointer_moved(position.x, position.y);
                // При открытой карте движение мыши только запоминается: курсор
                // карты двигают кнопки, а ЛКМ перепрыгивает на звезду.
                if self.hdmap.active {
                    return;
                }
                // При открытом экране закупки мышь наводит строку списка.
                if self.shop.active {
                    self.shop.point_at(position.x, position.y);
                    return;
                }
                // Координаты окна -> координаты экрана игры (целый масштаб,
                // та же математика, что в `present`).
                if let Some(w) = &self.window {
                    let size = w.inner_size();
                    let l = display::layout(size.width, size.height);
                    let px = position.x - l.x as f64;
                    let py = position.y - l.y as f64;
                    if px >= 0.0 && px < l.w as f64 && py >= 0.0 && py < l.h as f64 {
                        self.mouse_x = (px / l.scale as f64) as i32;
                        self.mouse_y = (py / l.scale as f64) as i32;
                    } else {
                        self.mouse_x = -1;
                        self.mouse_y = -1;
                    }
                } else {
                    self.mouse_x = position.x as i32;
                    self.mouse_y = position.y as i32;
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let pressed = state == ElementState::Pressed;
                // При открытой карте ЛКМ перепрыгивает курсором на звезду под
                // указателем и передаёт выбор в эмулятор.
                if self.hdmap.active {
                    if button == MouseButton::Left && pressed {
                        self.hdmap.click();
                        self.sync_hdmap_target();
                    }
                    return;
                }
                // При открытом экране закупки ЛКМ покупает строку/вариант.
                if self.shop.active {
                    if button == MouseButton::Left && pressed {
                        self.shop.click(&mut self.machine.mem);
                    }
                    return;
                }
                match button {
                    MouseButton::Left => self.mouse_left = pressed,
                    MouseButton::Right => self.mouse_right = pressed,
                    _ => {}
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let dy = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y,
                    MouseScrollDelta::PixelDelta(p) => p.y as f32,
                };
                if dy == 0.0 {
                    return;
                }
                // При открытой карте колесо меняет масштаб (зум к текущей
                // системе — карта всегда центрирована на ней).
                if self.hdmap.active {
                    self.hdmap.zoom_by(if dy > 0.0 { 1 } else { -1 });
                    self.sync_hdmap_target();
                    return;
                }
                if dy > 0.0 {
                    self.mouse_wheel_up = true;
                } else {
                    self.mouse_wheel_down = true;
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.quit {
            event_loop.exit();
            return;
        }
        let now = Instant::now();
        if now >= self.next_frame {
            self.frame();
            self.next_frame += FRAME_TIME;
            // Если не успеваем за реальным временем — не копим долг.
            if self.next_frame <= now {
                self.next_frame = now + FRAME_TIME;
            }
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_frame));
    }
}
