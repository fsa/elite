//! Открытие и закрытие нативных экранов (полёт, карта, закупка).

use super::App;
use crate::equip;
use crate::flight;
use crate::world;

impl App<'_> {
    /// Открыть нативный экран закупки поверх текущего состояния игры.
    fn open_shop(&mut self) {
        self.shop.open(&self.machine.mem, equip::KLADOV_SHOP);
        if let Some(w) = &self.window {
            let size = w.inner_size();
            self.shop.resize(size.width, size.height);
        }
        self.hdmap.close();
        self.menu = false;
        self.pending_key = None;
        self.pending_flight = None;
        self.pending_launch = false;
    }

    /// Открыть закупку, отложенную до конца кадра (см. `pending_shop`).
    pub(super) fn open_pending_shop(&mut self) {
        if !self.pending_shop {
            return;
        }
        self.pending_shop = false;
        if !self.machine.intro_seen
            && !self.flight.active
            && !self.hdmap.active
            && flight::z80_is_docked(&self.machine.mem)
        {
            self.open_shop();
        }
    }

    /// Открыть нативный экран полёта поверх текущего состояния игры.
    fn open_flight(&mut self, dir: flight::ViewDir) {
        let planet_color = world::current_planet_color(&self.machine.mem);
        self.flight.open(planet_color);
        self.flight.set_view_dir(dir);
        self.hdmap.close();
        self.shop.close();
        self.menu = false;
        self.pending_key = None;
        self.pending_shop = false;
        self.pending_flight = None;
        self.pending_launch = false;
    }

    /// Показать нативный вид: уже открытый экран только меняет направление,
    /// чтобы не сбрасывать звёздное поле и начатый переход.
    pub(super) fn show_flight(&mut self, dir: flight::ViewDir) {
        if self.flight.active {
            self.flight.set_view_dir(dir);
            self.pending_flight = None;
            self.pending_launch = false;
        } else {
            self.open_flight(dir);
        }
    }

    /// Следить за экраном Z80 под нативным видом и закрывать его, когда игра
    /// уходит с вида из кабины.
    ///
    /// Переходы (вылет/прыжок/стыковка) больше не отдают экран Z80: нативный
    /// вид остаётся открытым и рисует собственную анимацию. Закрывается он
    /// только когда оригинал уходит на другой экран (станция, карта,
    /// инвентарь).
    pub(super) fn follow_flight_screen(&mut self) {
        if !self.flight.active {
            return;
        }
        self.flight.follow_z80_screen(&self.machine.mem);
    }

    /// Открыть нативный вид, отложенный до полёта или перехода (см.
    /// `pending_flight`). Если оригинал уже начал вылет/прыжок/стыковку,
    /// рендер включается сразу и проигрывает нативную анимацию.
    pub(super) fn open_pending_flight(&mut self) {
        let Some(dir) = self.pending_flight else {
            return;
        };
        let in_transition = flight::z80_in_transition(&self.machine.mem);
        if flight::z80_in_cockpit(&self.machine.mem) || in_transition {
            if !self.flight.active {
                let launch = self.pending_launch;
                self.open_flight(dir);
                if in_transition {
                    self.flight.start_transition(launch, &self.machine.mem);
                }
            } else {
                self.flight.set_view_dir(dir);
                self.pending_flight = None;
                self.pending_launch = false;
            }
            return;
        }
        // Переход закончился, но полёт так и не начался (например, стыковка
        // привела на станцию): запрос больше не нужен. Запрос на вылет
        // (`F1` на станции) ждёт, пока игра действительно начнёт переход.
        if flight::z80_is_docked(&self.machine.mem) && !self.pending_launch {
            self.pending_flight = None;
        }
    }
}
