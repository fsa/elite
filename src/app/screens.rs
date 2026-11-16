//! Открытие и закрытие нативных экранов (полёт, карта, закупка).

use super::App;
use crate::equip;
use crate::flight;
use crate::galaxy;
use crate::market;
use crate::native;
use crate::status;
use crate::sysinfo;
use crate::world;

impl App<'_> {
    /// Открыть нативную карту галактики и перевести игру в режим
    /// `SHORT RANGE CHART`: Z80 продолжает работать, но считает, что на
    /// экране локальная карта.
    pub(super) fn open_hdmap(&mut self) {
        self.hdmap.open(&self.machine.mem, galaxy::KLADOV_GALAXY);
        if let Some(w) = &self.window {
            let size = w.inner_size();
            let (gw, gh) = self.game_size(size.width, size.height);
            self.hdmap.resize(gw, gh);
        }
        let prev = self.machine.mem[flight::Z80_WHAT_ON_SCREEN];
        self.hdmap_screen = Some(prev);
        self.machine.mem[flight::Z80_WHAT_ON_SCREEN] = flight::Z80_SCREEN_LOCAL_MAP;
        self.sync_hdmap_target();
        self.shop.close();
        self.close_status();
        self.close_sysinfo();
        self.close_market();
        self.pending_sysinfo = false;
        self.menu = false;
        self.pending_key = None;
        self.pending_flight = None;
        self.pending_launch = false;
    }

    /// Закрыть нативную карту. Прежний экран восстанавливается только если
    /// игра всё ещё на `SHORT RANGE CHART`: если она сама успела сменить
    /// экран, его не затираем.
    pub(super) fn close_hdmap(&mut self) {
        if !self.hdmap.active {
            return;
        }
        self.hdmap.close();
        if let Some(prev) = self.hdmap_screen.take() {
            if self.machine.mem[flight::Z80_WHAT_ON_SCREEN] == flight::Z80_SCREEN_LOCAL_MAP {
                self.machine.mem[flight::Z80_WHAT_ON_SCREEN] = prev;
            }
        }
    }

    /// Передать в эмулятор точку, на которой стоит курсор карты: родная карта
    /// читает её из `Pilot_CrdXTarget`/`Pilot_CrdYTarget`.
    pub(super) fn sync_hdmap_target(&mut self) {
        if !self.hdmap.active {
            return;
        }
        let (x, y) = self.hdmap.cursor();
        native::set_map_target(
            &mut self.machine,
            world::save_layout_kladov(),
            x as u8,
            y as u8,
        );
    }

    /// Открыть нативный экран закупки поверх текущего состояния игры.
    fn open_shop(&mut self) {
        self.shop.open(&self.machine.mem, equip::KLADOV_SHOP);
        if let Some(w) = &self.window {
            let size = w.inner_size();
            let (gw, gh) = self.game_size(size.width, size.height);
            self.shop.resize(gw, gh);
        }
        self.close_hdmap();
        self.close_status();
        self.close_sysinfo();
        self.close_market();
        self.pending_sysinfo = false;
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

    /// Открыть нативный экран статуса пилота (F9) — альтернативу экрану `L`.
    /// Пока он открыт, Z80 продолжает работать: в эмулятор уходит нажатие
    /// `L`, и оригинал переходит в режим показа капитана.
    pub(super) fn open_status(&mut self) {
        self.close_hdmap();
        self.shop.close();
        self.flight.close();
        self.close_sysinfo();
        self.close_market();
        self.pending_sysinfo = false;
        self.status_screen = Some(self.machine.mem[flight::Z80_WHAT_ON_SCREEN]);
        self.status.open(&self.machine.mem, status::KLADOV_STATUS);
        self.menu = false;
        self.pending_key = None;
        self.pending_shop = false;
        self.pending_flight = None;
        self.pending_launch = false;
    }

    /// Закрыть нативный экран статуса и вернуть прежний экран игры, если
    /// оригинал всё ещё показывает экран капитана (`L`).
    pub(super) fn close_status(&mut self) {
        if !self.status.active {
            return;
        }
        self.status.close();
        self.pending_key = None;
        if let Some(prev) = self.status_screen.take() {
            if self.machine.mem[flight::Z80_WHAT_ON_SCREEN] == flight::Z80_SCREEN_PILOT_INFO {
                self.machine.mem[flight::Z80_WHAT_ON_SCREEN] = prev;
            }
        }
    }

    /// Убрать нативную надстройку и оставить классический экран капитана:
    /// по `L` игра показывает свой родной экран, а не прежний вид.
    pub(super) fn close_status_to_classic(&mut self) {
        self.status.close();
        self.pending_key = None;
        self.status_screen = None;
        self.machine.mem[flight::Z80_WHAT_ON_SCREEN] = flight::Z80_SCREEN_PILOT_INFO;
    }

    /// Открыть нативный экран информации о планете (F7), отложенный до
    /// обработки клавиши `P` оригиналом.
    pub(super) fn open_pending_sysinfo(&mut self) {
        if !self.pending_sysinfo {
            return;
        }
        self.pending_sysinfo = false;
        if self.machine.intro_seen || self.menu {
            self.sysinfo_screen = None;
            return;
        }
        self.close_hdmap();
        self.shop.close();
        self.flight.close();
        self.close_status();
        self.close_market();
        self.pending_flight = None;
        self.pending_launch = false;
        self.sysinfo.open(&self.machine.mem, sysinfo::KLADOV_SYSINFO);
    }

    /// Закрыть нативный экран информации о планете и вернуть прежний экран
    /// игры, если оригинал всё ещё показывает экран `P`.
    pub(super) fn close_sysinfo(&mut self) {
        if !self.sysinfo.active && self.sysinfo_screen.is_none() {
            self.pending_sysinfo = false;
            return;
        }
        self.sysinfo.close();
        self.pending_sysinfo = false;
        self.pending_key = None;
        if let Some(prev) = self.sysinfo_screen.take() {
            if self.machine.mem[flight::Z80_WHAT_ON_SCREEN] == flight::Z80_SCREEN_INFO_PLANET {
                self.machine.mem[flight::Z80_WHAT_ON_SCREEN] = prev;
            }
        }
    }

    /// Включить нативный показ экрана цен на станции (альтернатива `K`).
    /// Эмулятор должен находиться в режиме рынка (`Z80_SCREEN_MARKET`).
    pub(super) fn open_market(&mut self) {
        self.close_hdmap();
        self.shop.close();
        self.flight.close();
        self.close_status();
        self.close_sysinfo();
        self.pending_sysinfo = false;
        self.market.enable(&self.machine.mem, market::KLADOV_MARKET);
        self.menu = false;
        self.pending_shop = false;
        self.pending_flight = None;
        self.pending_launch = false;
    }

    /// Выключить нативный показ экрана цен.
    pub(super) fn close_market(&mut self) {
        self.market.disable();
    }

    /// Открыть нативный экран полёта поверх текущего состояния игры.
    fn open_flight(&mut self, dir: flight::ViewDir) {
        let planet_color = world::current_planet_color(&self.machine.mem);
        self.flight.open(planet_color);
        self.flight.set_view_dir(dir);
        self.close_hdmap();
        self.shop.close();
        self.close_status();
        self.close_sysinfo();
        self.close_market();
        self.pending_sysinfo = false;
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
        // Заставка — это уже не полёт. После гибели корабля оригинал уходит
        // на `loc_739E`, где `WhatOnScreen` снова равен нулю, и без этой
        // проверки нативный вид остался бы поверх заставки.
        if self.machine.intro_seen {
            self.flight.close();
            self.pending_flight = None;
            self.pending_launch = false;
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
