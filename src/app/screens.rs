//! Открытие и закрытие нативных экранов (полёт, карта, закупка).

use winit::keyboard::KeyCode;

use super::{App, StationNativePending};
use crate::equip;
use crate::flight;
use crate::galaxy;
use crate::hdmap;
use crate::keys::matrix_pos;
use crate::market;
use crate::native;
use crate::status;
use crate::sysinfo;
use crate::world;

impl App<'_> {
    /// Закрыть все нативные экраны и отложенные переходы. Используется при
    /// передаче управления эмулятору и при входе в нативный режим станции.
    fn close_all_native(&mut self) {
        self.close_hdmap();
        self.shop.close();
        self.close_status();
        self.close_sysinfo();
        self.close_market();
        self.flight.close();
        self.docked.close();
        self.pause_stub = false;
        self.pending_status = false;
        self.pending_sysinfo = false;
        self.pending_market = false;
        self.pending_shop = false;
        self.pending_flight = None;
        self.pending_launch = false;
        self.pending_key = None;
    }

    /// Запросить нативный режим станции, сначала отдав эмулятору `L`.
    ///
    /// До заморозки Z80 должен обработать клавишу: оригинал выходит из режима
    /// покупки/продажи и уходит на экран капитана. Состояние покупки заранее не
    /// проверяем — `L` посылается при любом переходе в нативную станцию.
    fn request_native_station(&mut self, reason: StationNativePending) {
        self.pending_station_native = Some(reason);
        self.pending_key = matrix_pos(KeyCode::KeyL);
    }

    /// Сбросить отложенный переход, если он больше не актуален.
    pub(super) fn cancel_pending_station(&mut self) {
        if self.pending_station_native.take().is_some() {
            self.pending_key = None;
        }
    }

    /// Запомнить, что открытый нативный экран на станции забирает управление у
    /// эмулятора. В полёте этот вызов ничего не меняет.
    fn native_screen_opened(&mut self) {
        if self.machine.intro_seen
            || self.flight.active
            || !flight::z80_is_docked(&self.machine.mem)
            || self.station_native
        {
            return;
        }
        // Несколько экранов подряд могут открыться до того, как эмулятор
        // обработает `L`. Открытый экран должен пережить вход в нативный режим,
        // поэтому обычный вход повышаем до входа через экран. Клавишу тоже
        // восстанавливаем: открытие экрана могло сбросить `pending_key`.
        if let Some(reason) = self.pending_station_native {
            if reason == StationNativePending::Docked {
                self.pending_station_native = Some(StationNativePending::Screen);
            }
            if self.pending_key.is_none() {
                self.pending_key = matrix_pos(KeyCode::KeyL);
            }
            return;
        }
        self.request_native_station(StationNativePending::Screen);
    }

    /// Войти в нативный режим станции: эмулятор остаётся на экране капитана и
    /// больше не исполняется, пока управление не вернут клавишей `Backquote`.
    pub(super) fn enter_native_station(&mut self) {
        if self.machine.intro_seen
            || !flight::z80_is_docked(&self.machine.mem)
            || flight::z80_in_transition(&self.machine.mem)
        {
            return;
        }
        self.close_all_native();
        self.station_native = true;
        self.station_emulator = false;
        self.menu = false;
        if !self.status.active {
            self.status.open(&self.machine.mem, status::KLADOV_STATUS);
        }
    }

    /// Завершить отложенный вход после кадра, в котором `L` ушёл в Z80.
    /// Открытый нативный экран при этом сохраняется.
    pub(super) fn finish_pending_station_native(&mut self) {
        let Some(reason) = self.pending_station_native.take() else {
            return;
        };
        self.pending_key = None;
        if self.machine.intro_seen
            || !flight::z80_is_docked(&self.machine.mem)
            || flight::z80_in_transition(&self.machine.mem)
        {
            return;
        }
        match reason {
            StationNativePending::Docked => self.enter_native_station(),
            StationNativePending::Screen => {
                self.station_native = true;
                self.station_emulator = false;
            }
        }
    }

    /// Включить нативный режим станции, если игра оказалась пристыкованной и
    /// управление ещё не передано эмулятору явно. Заморозка откладывается на
    /// один кадр: эмулятор должен получить `L`.
    pub(super) fn maybe_enter_native_station(&mut self) {
        if !flight::z80_is_docked(&self.machine.mem) {
            self.station_native = false;
            self.station_emulator = false;
            self.cancel_pending_station();
            return;
        }
        if self.machine.intro_seen || self.flight.active {
            self.cancel_pending_station();
            return;
        }
        if flight::z80_in_transition(&self.machine.mem) {
            self.cancel_pending_station();
            return;
        }
        if self.station_native {
            self.cancel_pending_station();
            return;
        }
        // Отложенный вход из эмулятора живёт ровно до конца кадра, в котором
        // Z80 получит `L`: `station_emulator` ещё включён, но отменять запрос
        // нельзя.
        if self.station_emulator || self.pending_station_native.is_some() {
            return;
        }
        self.request_native_station(StationNativePending::Docked);
    }

    /// Нативный экран передаёт управление эмулятору: скрываем все нативные
    /// экраны и показываем кадр Z80 без дополнительных клавиш.
    pub(super) fn handoff_to_emulator(&mut self) {
        self.close_all_native();
        self.cancel_pending_station();
        self.station_native = false;
        self.station_emulator = true;
        self.menu = false;
        // `Backquote` отсутствует в матрице ZX, поэтому в игру не уходит ни
        // одна клавиша: эмулятор просто снимается с паузы.
    }

    /// Открыть нативную карту в галактическом масштабе (клавиша `F6` или `I`).
    pub(super) fn open_hdmap(&mut self) {
        self.open_hdmap_mode(hdmap::MapMode::Galaxy);
    }

    /// Открыть нативную карту в заданном масштабе или переключить масштаб,
    /// если карта уже открыта. Игра переводится в режим `SHORT RANGE CHART`,
    /// чтобы родные сцены карты оставались в ожидаемом состоянии.
    pub(super) fn open_hdmap_mode(&mut self, mode: hdmap::MapMode) {
        if self.hdmap.active {
            self.hdmap.set_mode(mode);
            self.sync_hdmap_target();
            return;
        }
        // Пресет `Local` зависит от размера окна, поэтому сначала сообщаем
        // карте геометрию, и только потом читаем из неё масштаб.
        if let Some(w) = &self.window {
            let size = w.inner_size();
            let (gw, gh) = self.game_size(size.width, size.height);
            self.hdmap.resize(gw, gh);
        }
        self.hdmap
            .open_mode(&self.machine.mem, galaxy::KLADOV_GALAXY, mode);
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
        self.native_screen_opened();
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
    pub(super) fn open_shop(&mut self) {
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
        self.native_screen_opened();
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
        self.native_screen_opened();
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
        if !self.status.active {
            self.close_hdmap();
            self.shop.close();
            self.flight.close();
            self.close_sysinfo();
            self.close_market();
            self.pending_sysinfo = false;
            self.pending_market = false;
            self.pending_shop = false;
            self.pending_flight = None;
            self.pending_launch = false;
            self.status.open(&self.machine.mem, status::KLADOV_STATUS);
        }
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
        if self.sysinfo.active {
            self.refresh_selected_planet();
        } else {
            self.close_hdmap();
            self.shop.close();
            self.flight.close();
            self.close_status();
            self.close_market();
            self.pending_status = false;
            self.pending_market = false;
            self.pending_flight = None;
            self.pending_launch = false;
            self.open_sysinfo();
        }
    }

    /// Обновить в состоянии имя планеты, выбранной для показа. Оригинал
    /// заполняет `PlanetName` при обработке `P`, до очередной синхронизации
    /// состояния об этом нужно знать сразу.
    fn refresh_selected_planet(&mut self) {
        self.state.planet = world::selected_planet_name(&self.machine.mem);
    }

    /// Включить нативный экран информации о планете с текущим именем из состояния.
    pub(super) fn open_sysinfo(&mut self) {
        self.close_hdmap();
        self.shop.close();
        self.flight.close();
        self.close_status();
        self.close_market();
        self.pending_status = false;
        self.pending_market = false;
        self.pending_flight = None;
        self.pending_launch = false;
        self.pending_key = None;
        self.refresh_selected_planet();
        let name = self.state.planet.clone();
        self.sysinfo
            .open(&self.machine.mem, sysinfo::KLADOV_SYSINFO, &name);
        self.native_screen_opened();
    }

    /// Выключить нативную отрисовку экрана информации о планете. Экранная
    /// переменная Z80 не трогается.
    pub(super) fn close_sysinfo(&mut self) {
        self.sysinfo.close();
        self.pending_sysinfo = false;
        self.pending_key = None;
    }

    /// Запомнить, что в текущем переходе Z80 вёл отсчёт гиперпрыжка.
    ///
    /// После обнуления `JumpCountdown` память уже не позволяет отличить прыжок
    /// от стыковки, поэтому признак снимается только по окончании перехода.
    pub(super) fn track_jump_countdown(&mut self) {
        let mem = &self.machine.mem;
        if mem.get(flight::Z80_JUMP_COUNTDOWN).copied().unwrap_or(0) > 0 {
            self.saw_jump_countdown = true;
        } else if !flight::z80_in_transition(mem) {
            self.saw_jump_countdown = false;
        }
    }

    /// В момент начала тоннеля гиперпрыжка открыть нативный вид F1.
    ///
    /// Отсчёт может идти под любым нативным экраном (или под эмуляторным
    /// кадром): экран не закрывается до нуля, а сам прыжок уже показывается
    /// нативной анимацией, как по `F1`.
    pub(super) fn switch_to_flight_on_jump(&mut self) {
        if !self.saw_jump_countdown || self.flight.active {
            return;
        }
        let in_transition = flight::z80_in_transition(&self.machine.mem);
        let countdown = self
            .machine
            .mem
            .get(flight::Z80_JUMP_COUNTDOWN)
            .copied()
            .unwrap_or(0);
        if !in_transition || countdown != 0 {
            return;
        }
        self.show_flight(flight::ViewDir::Forward);
        self.flight
            .start_transition_kind(flight::TransitionKind::Hyperspace, countdown);
        self.saw_jump_countdown = false;
    }

    /// Следить за экранными режимами Z80 под нативными экранами без
    /// собственного интерактива (F7/F8/F9). Когда оригинал уходит с экрана,
    /// нативная надстройка закрывается; пока режим тот же — остаётся.
    pub(super) fn follow_native_screens(&mut self) {
        // Пока эмулятор обрабатывает `L`, его экранная переменная ещё может
        // смениться; закрывать только что открытый нативный экран нельзя.
        if self.pending_station_native.is_some() {
            return;
        }
        // Во время отсчёта гиперпрыжка нативный экран остаётся на месте:
        // переход к тоннелю сам откроет вид F1 (см. `switch_to_flight_on_jump`).
        if flight::z80_in_transition(&self.machine.mem)
            && self
                .machine
                .mem
                .get(flight::Z80_JUMP_COUNTDOWN)
                .copied()
                .unwrap_or(0)
                > 0
        {
            return;
        }
        // В нативном режиме станции эмулятор стоит на месте, поэтому его
        // экранная переменная не является признаком закрытия нативного вида.
        if self.station_native {
            return;
        }
        let screen = self.machine.mem[flight::Z80_WHAT_ON_SCREEN];
        if self.status.active
            && !self.pending_status
            && screen != flight::Z80_SCREEN_PILOT_INFO
        {
            self.close_status();
        }
        if self.sysinfo.active
            && !self.pending_sysinfo
            && screen != flight::Z80_SCREEN_INFO_PLANET
        {
            self.close_sysinfo();
        }
        if self.market.native
            && !self.pending_market
            && screen != flight::Z80_SCREEN_MARKET
        {
            self.close_market();
        }
    }

    /// Включить нативный показ экрана цен (альтернатива `K`).
    /// Эмулятор должен находиться в режиме рынка (`Z80_SCREEN_MARKET`).
    pub(super) fn open_market(&mut self) {
        self.close_hdmap();
        self.shop.close();
        self.flight.close();
        self.close_status();
        self.close_sysinfo();
        self.pending_status = false;
        self.pending_sysinfo = false;
        self.pending_market = false;
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
        self.native_screen_opened();
    }

    /// Выключить нативный показ экрана цен.
    pub(super) fn close_market(&mut self) {
        self.market.disable();
        self.pending_market = false;
        self.pending_key = None;
    }

    /// Открыть нативный экран цен, отложенный до выхода оригинала в
    /// режим `Z80_SCREEN_MARKET`: по `F8` (клавиша `K`), когда эмулятор
    /// сам переходит на экран рынка.
    pub(super) fn open_pending_market(&mut self) {
        if !self.pending_market {
            return;
        }
        if self.machine.intro_seen || self.menu {
            self.pending_market = false;
            return;
        }
        if self.machine.mem[flight::Z80_WHAT_ON_SCREEN] != flight::Z80_SCREEN_MARKET {
            return;
        }
        self.pending_market = false;
        self.open_market();
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
