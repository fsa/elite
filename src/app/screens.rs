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
        self.pending_status = false;
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
        self.pending_status = false;
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

    /// Включить нативную отрисовку экрана капитана (F9). Экран показывается,
    /// только пока Z80 находится в режиме `Z80_SCREEN_PILOT_INFO`; ввод при
    /// этом остаётся за эмулятором.
    pub(super) fn open_status(&mut self) {
        self.close_hdmap();
        self.shop.close();
        self.flight.close();
        self.close_sysinfo();
        self.close_market();
        self.pending_status = false;
        self.pending_sysinfo = false;
        self.status.open(&self.machine.mem, status::KLADOV_STATUS);
        self.menu = false;
        self.pending_key = None;
        self.pending_shop = false;
        self.pending_flight = None;
        self.pending_launch = false;
    }

    /// Закрыть нативные экраны, которые не показываются на заставке.
    ///
    /// Отложенный запрос страницы капитана сохраняем: `SPACE` на заставке
    /// выставляет его в текущем кадре, а открыть экран можно только после
    /// выхода эмулятора из заставки. Обычный `close_status()` сбросил бы
    /// запрос вместе с экраном.
    pub(super) fn close_intro_screens(&mut self) {
        self.status.close();
        self.sysinfo.close();
    }

    /// Выключить нативную отрисовку экрана капитана. Экранная переменная
    /// Z80 не трогается: оригинал сам решает, когда уйти с этого экрана.
    pub(super) fn close_status(&mut self) {
        self.status.close();
        self.pending_status = false;
        self.pending_key = None;
    }

    /// Открыть нативный экран капитана, отложенный до выхода оригинала в
    /// режим `Z80_SCREEN_PILOT_INFO`: по `F9` (клавиша `L`) или по `SPACE` на
    /// заставке, когда эмулятор сам приходит на экран капитана веткой старта.
    ///
    /// Пока заставка ещё идёт, запрос сохраняем — он должен дождаться выхода
    /// эмулятора в игру. Здесь намеренно читаем состояние Z80 напрямую,
    /// минуя [`crate::state::GameState`]: `sync_state()` работает в начале
    /// кадра, до прогона эмулятора, и ещё видит заставку, хотя `SPACE` уже
    /// увёл Z80 на ветку старта. Главное — дождаться, пока родная ветка
    /// старта выполнит инициализацию сохранения игры: до неё данные
    /// командира в памяти могут быть неверными, и показывать страницу
    /// капитана рано. В дальнейшем инициализация данных игры будет идти
    /// нативно здесь, а сейчас мы ждём эмулятор.
    pub(super) fn open_pending_status(&mut self) {
        if !self.pending_status {
            return;
        }
        if self.menu {
            self.pending_status = false;
            return;
        }
        if self.machine.intro_seen {
            return;
        }
        if self.machine.mem[flight::Z80_WHAT_ON_SCREEN] != flight::Z80_SCREEN_PILOT_INFO {
            return;
        }
        self.pending_status = false;
        self.close_hdmap();
        self.shop.close();
        self.flight.close();
        self.close_sysinfo();
        self.close_market();
        self.pending_sysinfo = false;
        self.pending_shop = false;
        self.pending_flight = None;
        self.pending_launch = false;
        self.status.open(&self.machine.mem, status::KLADOV_STATUS);
    }

    /// Открыть нативный экран информации о планете (F7), отложенный до
    /// обработки клавиши `P` оригиналом. Открываем только если оригинал
    /// действительно перешёл на экран `Z80_SCREEN_INFO_PLANET`.
    pub(super) fn open_pending_sysinfo(&mut self) {
        if !self.pending_sysinfo {
            return;
        }
        if self.machine.intro_seen || self.menu {
            self.pending_sysinfo = false;
            return;
        }
        if self.machine.mem[flight::Z80_WHAT_ON_SCREEN] != flight::Z80_SCREEN_INFO_PLANET {
            return;
        }
        self.pending_sysinfo = false;
        self.close_hdmap();
        self.shop.close();
        self.flight.close();
        self.close_status();
        self.close_market();
        self.pending_status = false;
        self.pending_flight = None;
        self.pending_launch = false;
        self.open_sysinfo();
    }

    /// Обновить в состоянии имя планеты, выбранной для показа. Оригинал
    /// заполняет `PlanetName` при обработке `P`, до очередной синхронизации
    /// состояния об этом нужно знать сразу.
    fn refresh_selected_planet(&mut self) {
        self.state.planet = world::selected_planet_name(&self.machine.mem);
    }

    /// Включить нативный экран информации о планете с текущим именем из состояния.
    pub(super) fn open_sysinfo(&mut self) {
        self.refresh_selected_planet();
        let name = self.state.planet.clone();
        self.sysinfo
            .open(&self.machine.mem, sysinfo::KLADOV_SYSINFO, &name);
    }

    /// Выключить нативную отрисовку экрана информации о планете. Экранная
    /// переменная Z80 не трогается.
    pub(super) fn close_sysinfo(&mut self) {
        self.sysinfo.close();
        self.pending_sysinfo = false;
        self.pending_key = None;
    }

    /// Показан ли сейчас нативный экран вместо экрана, который рисует
    /// эмулятор.
    pub(super) fn native_screen_active(&self) -> bool {
        self.hdmap.active
            || self.shop.active
            || self.status.active
            || self.sysinfo.active
            || self.market.native
            || self.flight.active
    }

    /// Следить за экранными режимами Z80 под нативными экранами без
    /// собственного интерактива (F7/F9). Когда оригинал уходит с экрана,
    /// нативная надстройка закрывается; пока режим тот же — остаётся.
    pub(super) fn follow_native_screens(&mut self) {
        let screen = self.machine.mem[flight::Z80_WHAT_ON_SCREEN];
        if self.status.active && screen != flight::Z80_SCREEN_PILOT_INFO {
            self.close_status();
        }
        if self.sysinfo.active && screen != flight::Z80_SCREEN_INFO_PLANET {
            self.close_sysinfo();
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
        self.pending_status = false;
        self.pending_sysinfo = false;
        let docked = flight::z80_is_docked(&self.machine.mem);
        self.market.enable(
            &self.machine.mem,
            market::KLADOV_MARKET,
            docked,
            &self.state.system,
        );
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
        self.pending_status = false;
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
        if !self.flight.active && self.flight.transition_kind().is_none() {
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
