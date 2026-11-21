//! Жизненный цикл окна winit и события ввода.

use std::process::exit;
use std::rc::Rc;
use std::time::{Duration, Instant};

use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::keyboard::{ModifiersState, PhysicalKey};
use winit::window::{Fullscreen, Window, WindowId};

use super::{App, FRAME_TIME};
use crate::cockpit;
use crate::display;
use crate::state::Screen;
use crate::ui::present::{SRC_H, SRC_W, debug_button_at, save_editor_button_at};

/// Через сколько времени без движения мыши прячем курсор в полном экране.
const CURSOR_IDLE: Duration = Duration::from_secs(3);

impl App<'_> {
    /// Любое событие мыши возвращает курсор и отсчитывает тайм-аут заново.
    pub(super) fn note_mouse_activity(&mut self) {
        self.last_mouse_activity = Instant::now();
        if !self.cursor_visible {
            if let Some(w) = &self.window {
                w.set_cursor_visible(true);
            }
            self.cursor_visible = true;
        }
    }

    /// Спрятать курсор в полном экране после паузы; в обычном окне он всегда
    /// остаётся видимым.
    fn update_cursor_visibility(&mut self) {
        let Some(w) = &self.window else {
            return;
        };
        let hide = cursor_hidden_by_idle(w.fullscreen().is_some(), self.last_mouse_activity.elapsed());
        if hide == !self.cursor_visible {
            return;
        }
        w.set_cursor_visible(!hide);
        self.cursor_visible = !hide;
    }

    /// Клик по овалу радара в нативном кокпите переключает вид сканера.
    ///
    /// Координаты приходят в пикселях окна. Нативный кокпит рисуется в HD-слое
    /// 1:1 в игровой области (слева от отладочной полосы), поэтому перевод в
    /// координаты HD-слоя — это просто вычитание начала игровой области; здесь
    /// она начинается в (0,0). Клик по радару поглощается, чтобы он не
    /// срабатывал одновременно как выстрел.
    pub(super) fn toggle_radar_at(&mut self, x: i32, y: i32, win_w: u32, win_h: u32) -> bool {
        if self.menu
            || self.docked.is_active()
            || !matches!(self.state.screen, Screen::Flight | Screen::Animation(_))
        {
            return false;
        }
        let (game_w, game_h) = self.game_size(win_w, win_h);
        if x < 0 || y < 0 || x >= game_w as i32 || y >= game_h as i32 {
            return false;
        }
        if !cockpit::oval_radar_hit(&cockpit::panel_geometry(game_w, game_h), x, y) {
            return false;
        }
        self.radar_view = match self.radar_view {
            cockpit::RadarView::Classic => cockpit::RadarView::Kladov,
            cockpit::RadarView::Kladov => cockpit::RadarView::Classic,
        };
        true
    }

    fn create_window(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let debug_w = if self.opts.debug {
            display::DEBUG_W as f64
        } else {
            0.0
        };
        let attrs = Window::default_attributes()
            .with_title("Elite: Kladov Revisited")
            .with_inner_size(LogicalSize::new(
                4.0 * SRC_W as f64 + debug_w,
                4.0 * SRC_H as f64,
            ));
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

/// Чистая часть решения о скрытии курсора — вынесена из метода окна, чтобы
/// тайм-аут можно было проверить без настоящего `Window`.
fn cursor_hidden_by_idle(fullscreen: bool, idle: Duration) -> bool {
    fullscreen && idle >= CURSOR_IDLE
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
                self.note_mouse_activity();
                self.cursor_x = position.x;
                self.cursor_y = position.y;
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
                    let l = self.zx_layout(size.width, size.height);
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
                self.note_mouse_activity();
                let pressed = state == ElementState::Pressed;
                // Кнопки отладочной полосы переключают вид под миниатюрой и
                // не попадают в игру.
                if self.opts.debug && button == MouseButton::Left && pressed {
                    let out_w = self
                        .window
                        .as_ref()
                        .map(|w| w.inner_size().width as usize)
                        .unwrap_or(0);
                    let out_h = self
                        .window
                        .as_ref()
                        .map(|w| w.inner_size().height as usize)
                        .unwrap_or(0);
                    if let Some(view) =
                        debug_button_at(out_w, self.cursor_x, self.cursor_y)
                    {
                        self.debug_view = view;
                        return;
                    }
                    if !self.machine.intro_seen && save_editor_button_at(out_w, out_h, self.cursor_x, self.cursor_y) {
                        self.save_editor_open = true;
                        self.save_editor_sel = 0;
                        self.save_editor_editing = false;
                        self.save_editor_input.clear();
                        self.save_editor_name_editing = false;
                        self.save_editor_name_input.clear();
                        self.save_editor_hex_cursor = 0;
                        self.save_editor_hex_editing = false;
                        self.save_editor_hex_input.clear();
                        return;
                    }
                }
                // При открытой карте ЛКМ перепрыгивает курсором на звезду под
                // указателем, ПКМ делает то же и открывает информацию о системе.
                if self.hdmap.active {
                    if pressed {
                        match button {
                            MouseButton::Left => {
                                self.hdmap.click();
                                self.sync_hdmap_target();
                                if self.hdmap.is_searching() {
                                    self.hdmap.cancel_search();
                                }
                            }
                            MouseButton::Right => {
                                self.hdmap.click();
                                self.sync_hdmap_target();
                                if self.hdmap.is_searching() {
                                    self.hdmap.cancel_search();
                                }
                                if !self.menu && !self.machine.intro_seen {
                                    self.open_planet_info();
                                    self.sysinfo_from_map = true;
                                }
                            }
                            _ => {}
                        }
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
                // Экран информации о системе, открытый по ПКМ с карты, по ПКМ
                // возвращает на карту в прежнем состоянии.
                if self.sysinfo.active
                    && self.sysinfo_from_map
                    && button == MouseButton::Right
                    && pressed
                {
                    self.return_from_sysinfo_to_map();
                    return;
                }
                // В полёте ЛКМ по овалу радара переключает вид локатора.
                if button == MouseButton::Left && pressed {
                    if let Some(w) = &self.window {
                        let size = w.inner_size();
                        if self.toggle_radar_at(
                            self.cursor_x as i32,
                            self.cursor_y as i32,
                            size.width,
                            size.height,
                        ) {
                            return;
                        }
                    }
                }
                match button {
                    MouseButton::Left => self.mouse_left = pressed,
                    MouseButton::Right => self.mouse_right = pressed,
                    _ => {}
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.note_mouse_activity();
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
        self.update_cursor_visibility();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_hides_only_after_idle_in_fullscreen() {
        assert!(!cursor_hidden_by_idle(false, CURSOR_IDLE * 2));
        assert!(!cursor_hidden_by_idle(true, CURSOR_IDLE - Duration::from_millis(1)));
        assert!(cursor_hidden_by_idle(true, CURSOR_IDLE));
        assert!(cursor_hidden_by_idle(true, CURSOR_IDLE * 2));
    }
}
