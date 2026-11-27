//! Открытие и закрытие нативных экранов (полёт, карта, закупка).

use winit::keyboard::KeyCode;

use super::{App, MarketMission, MarketMissionStep, MissionPrompt, StationNativePending};
use crate::equip;
use crate::flight;
use crate::galaxy;
use crate::hdmap;
use crate::keys::matrix_pos;
use crate::market;
use crate::native;
use crate::status;
use crate::world;

/// Диапазон PC цикла ожидания `Y/N` в `sub_E1F7` (`loc_E1FD`..`ret`).
const MISSION_PROMPT_PC: std::ops::RangeInclusive<u16> = 0xE1FD..=0xE205;
/// `PressKeyPrnSymbol`: original ждёт не-`0xFF` в этом байте.
const PRESS_KEY_SYMBOL: usize = 0xFC0D;
/// PC команды `ld a,(PressKeyPrnSymbol)` в цикле `sub_E1F7`.
const MISSION_PROMPT_READ_PC: u16 = 0xE1FD;
/// Сколько кадров ждать запроса миссии/выхода в инвентарь.
const MISSION_WAIT_FRAMES: u16 = 30;
/// Перехват чтения `PressKeyPrnSymbol` в цикле `sub_E1F7`.
///
/// Обычный опрос матрицы успевает перезаписать символ до того, как Z80
/// прочитает его, поэтому во время миссионного сеанса original читает ответ
/// из нативного состояния. `hook_yield` нужен, потому что это не `CALL`:
/// возврат из `sub_E1F7` к своей команде делаем сами.
fn mission_prompt_key(cpu: &mut crate::z80::Z80, m: &mut crate::machine::Spectrum) -> u64 {
    cpu.a = m.mission_answer;
    cpu.pc = MISSION_PROMPT_READ_PC + 3;
    m.hook_yield = true;
    4
}

impl App<'_> {
    /// Закрыть все нативные экраны и отложенные переходы. Используется при
    /// передаче управления эмулятору и при входе в нативный режим станции.
    pub(super) fn close_all_native(&mut self) {
        self.close_hdmap();
        self.shop.close();
        self.close_inventory();
        self.close_status();
        self.close_sysinfo();
        self.close_market();
        self.flight.close();
        self.docked.close();
        self.pending_status = false;
        self.pending_sysinfo = false;
        self.pending_market = false;
        self.pending_shop = false;
        self.pending_flight = None;
        self.pending_launch = false;
        self.pending_key = None;
        self.market_mission = None;
        self.mission_prompt = None;
        self.machine.mission_answer = 0xFF;
        self.machine.remove_hook(MISSION_PROMPT_READ_PC);
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

    /// Войти в нативный режим станции: нативные экраны показываются поверх,
    /// но Z80 продолжает работать до естественного перехода на станционный
    /// экран. Клавиши игры в Z80 не передаются, кроме отложенных (например F1).
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
        self.flight_emulator = false;
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
        // Пока идёт загрузка, Z80 должен беспрепятственно дойти до хука.
        // Заморозка станции вернётся после снятия заглушки.
        if self.loading {
            self.cancel_pending_station();
            return;
        }
        if !flight::z80_is_docked(&self.machine.mem) {
            self.station_native = false;
            self.station_emulator = false;
            self.cancel_pending_station();
            return;
        }
        // `IndDanger` сбрасывается раньше, чем original уходит с вида из
        // кабины на экран станции (особенно после загрузки из полёта). Если
        // заморозить Z80 в этот момент, страница капитана не откроется.
        if flight::z80_in_cockpit(&self.machine.mem) {
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

    /// Убрать заглушку загрузки, когда хук вывел игру из заставки.
    ///
    /// `load_requested` снимается самим хуком, но до входа в `GameMain`
    /// эмулятор ещё может показывать интро либо готовиться к станции из
    /// полёта. Поэтому ждём сброса `intro_seen` и `load_wait_game`; нативный
    /// режим станции включится после этого, уже без заглушки.
    pub(super) fn sync_loading_overlay(&mut self) {
        if !self.loading || self.machine.load_requested || self.machine.load_wait_game {
            return;
        }
        if !self.machine.intro_seen {
            self.loading = false;
        }
    }

    /// Снять наложения, которые удерживают Z80 или перехватывают ввод.
    fn clear_handoff_overlays(&mut self) {
        self.exit_pause();
        self.menu = false;
        self.save_screen = None;
        self.input_char = None;
        self.loading = false;
        self.save_editor_open = false;
        self.save_editor_editing = false;
        self.save_editor_input.clear();
        self.save_editor_name_editing = false;
        self.save_editor_name_input.clear();
        self.save_editor_hex_editing = false;
        self.save_editor_hex_input.clear();
    }

    /// Кнопка ` — единый выход в эмулятор из всех режимов. На заставке
    /// нажатие переключает нативный титр и родное интро; повторное нажатие
    /// возвращает титр, не перезапуская Z80.
    pub(super) fn toggle_emulator_handoff(&mut self) {
        if self.machine.intro_seen && self.splash_emulator {
            self.clear_handoff_overlays();
            self.splash_emulator = false;
            self.station_emulator = false;
            self.flight_emulator = false;
            self.close_all_native();
            return;
        }
        self.handoff_to_emulator();
    }

    /// Нативный экран передаёт управление эмулятору: скрываем все нативные
    /// экраны и показываем кадр Z80 без дополнительных клавиш.
    pub(super) fn handoff_to_emulator(&mut self) {
        // Пауза, меню и редактор тоже удерживают Z80: снимаем их до того, как
        // отдать управление, иначе эмулятор останется замороженным.
        self.clear_handoff_overlays();
        self.close_all_native();
        self.cancel_pending_station();
        self.station_native = false;
        // Если ` нажата посреди вылета/прыжка/стыковки, родной ролик не должен
        // открыться снова на следующем кадре. Флаги Z80 читаем до ветвления:
        // переход мог начаться ещё до смены `IndDanger` на станции.
        let in_transition = flight::z80_in_transition(&self.machine.mem);
        if self.machine.intro_seen {
            // Титр и родное интро — два вида одного состояния заставки.
            self.splash_emulator = true;
            self.station_emulator = false;
            self.flight_emulator = false;
        } else if flight::z80_is_docked(&self.machine.mem) {
            // На станции сохраняем прежний смысл флагов: `station_emulator`
            // включает обработку оригинальных клавиш из режима эмулятора.
            self.station_emulator = true;
            self.flight_emulator = false;
        } else {
            // В космосе достаточно `flight_emulator`: он подавляет автооткрытие
            // нативного кокпита и передаёт игре клавиши управления.
            self.station_emulator = false;
            self.flight_emulator = true;
        }
        if !self.machine.intro_seen {
            self.emulator_transition_lock = in_transition;
        }
        // `Backquote` отсутствует в матрице ZX, поэтому в игру не уходит ни
        // одна клавиша: эмулятор просто продолжает с того места, где был
        // остановлен.
    }

    /// Открыть нативную карту в галактическом масштабе (клавиша `F6` или `I`).
    pub(super) fn open_hdmap(&mut self) {
        self.open_hdmap_mode(hdmap::MapMode::Galaxy);
    }

    /// Открыть нативную карту в заданном масштабе или переключить масштаб,
    /// если карта уже открыта. На станции игра переводится в режим
    /// `SHORT RANGE CHART`, чтобы родные сцены карты оставались в ожидаемом
    /// состоянии. В полёте эмулятор остаётся в режиме кабины и продолжает
    /// симуляцию, но кнопки управления кораблём под картой не передаются.
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
        if flight::z80_is_docked(&self.machine.mem) || self.station_native || self.station_emulator {
            let prev = self.machine.mem[flight::Z80_WHAT_ON_SCREEN];
            self.hdmap_screen = Some(prev);
            self.machine.mem[flight::Z80_WHAT_ON_SCREEN] = flight::Z80_SCREEN_LOCAL_MAP;
        } else {
            self.hdmap_screen = None;
        }
        self.sync_hdmap_target();
        self.shop.close();
        self.close_inventory();
        self.close_status();
        self.close_sysinfo();
        self.close_market();
        self.pending_status = false;
        self.pending_sysinfo = false;
        self.pending_market = false;
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
        self.close_inventory();
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

    /// Включить нативную отрисовку экрана капитана (F9/L). На станции и в
    /// нативном полёте экран показывается сразу по живым данным, без перевода
    /// эмулятора на экран `L`.
    pub(super) fn open_status(&mut self) {
        self.close_hdmap();
        self.shop.close();
        self.close_inventory();
        self.flight.close();
        self.close_sysinfo();
        self.close_market();
        self.pending_status = false;
        self.pending_sysinfo = false;
        self.pending_market = false;
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
        self.inventory.close();
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

    /// Открыть нативный экран инвентаря (Enter). На станции и в нативном
    /// полёте экран показывается сразу по живым данным: Z80 не переводится на
    /// экран `0x0C`, поэтому симуляция и заморозка станции не нарушаются. Из
    /// режима эмулятора экран открывается только когда оригинал уже дошёл до
    /// своего инвентаря (см. `App::emulator_screen_key`).
    pub(super) fn open_inventory(&mut self) {
        self.close_hdmap();
        self.shop.close();
        self.flight.close();
        self.close_status();
        self.close_sysinfo();
        self.close_market();
        if !self.machine.intro_seen {
            self.inventory
                .open(&self.machine.mem, status::KLADOV_STATUS.pilot);
        }
        self.menu = false;
        self.pending_key = None;
        self.pending_shop = false;
        self.pending_flight = None;
        self.pending_launch = false;
        self.native_screen_opened();
    }

    /// Закрыть нативный экран инвентаря при переходе на другую страницу.
    /// Экранную переменную Z80 не трогаем: нативный экран — только надстройка
    /// над текущим состоянием оригинала.
    pub(super) fn close_inventory(&mut self) {
        self.inventory.close();
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
            self.close_inventory();
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
            self.sysinfo
                .refresh(&self.machine.mem, galaxy::KLADOV_GALAXY);
            self.state.planet = self.sysinfo.selected.name.clone();
        } else {
            self.close_hdmap();
            self.shop.close();
            self.close_inventory();
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



    /// Открыть нативный экран информации о планете (F7/P). В полёте экран
    /// показывается нативно, а эмулятор остаётся в режиме кабины и продолжает
    /// симуляцию без передачи кнопок управления кораблём.
    pub(super) fn open_planet_info(&mut self) {
        self.sysinfo_from_map = false;
        let docked = flight::z80_is_docked(&self.machine.mem);
        let in_flight = !docked && !self.machine.intro_seen;
        let on_planet =
            self.machine.mem[flight::Z80_WHAT_ON_SCREEN] == flight::Z80_SCREEN_INFO_PLANET;
        self.close_hdmap();
        self.shop.close();
        self.close_inventory();
        self.close_status();
        self.flight.close();
        self.close_market();
        self.pending_shop = false;
        self.pending_flight = None;
        self.pending_launch = false;
        self.pending_status = false;
        self.pending_sysinfo = false;
        self.pending_key = None;
        self.open_sysinfo();
        // На станции данные планеты уже в памяти. В эмуляторном режиме вне
        // станции оригинал должен сам перейти на экран `P`; в нативном полёте
        // экран показывается поверх симуляции без переключения экрана Z80.
        if !in_flight && !docked && !on_planet {
            self.pending_sysinfo = true;
            self.pending_key = matrix_pos(KeyCode::KeyP);
        }
    }

    /// Включить нативный экран информации о планете.
    pub(super) fn open_sysinfo(&mut self) {
        self.sysinfo_from_map = false;
        self.close_hdmap();
        self.shop.close();
        self.close_inventory();
        self.flight.close();
        self.close_status();
        self.close_market();
        self.pending_status = false;
        self.pending_market = false;
        self.pending_flight = None;
        self.pending_launch = false;
        self.pending_key = None;
        self.sysinfo.open(&self.machine.mem, galaxy::KLADOV_GALAXY);
        self.state.planet = self.sysinfo.selected.name.clone();
        self.native_screen_opened();
    }

    /// Вернуться с экрана информации о планете на карту в прежнем состоянии.
    /// Используется, если экран был открыт по ПКМ на карте.
    pub(super) fn return_from_sysinfo_to_map(&mut self) {
        if !self.sysinfo.active || !self.sysinfo_from_map || self.hdmap.planet_count() == 0 {
            return;
        }
        self.sysinfo_from_map = false;
        self.close_sysinfo();
        self.shop.close();
        self.close_status();
        self.close_market();
        self.pending_status = false;
        self.pending_market = false;
        self.menu = false;
        self.pending_key = None;
        self.pending_flight = None;
        self.pending_launch = false;
        if let Some(w) = &self.window {
            let size = w.inner_size();
            let (gw, gh) = self.game_size(size.width, size.height);
            self.hdmap.resize(gw, gh);
        }
        let prev = self.machine.mem[flight::Z80_WHAT_ON_SCREEN];
        self.hdmap_screen = Some(prev);
        self.machine.mem[flight::Z80_WHAT_ON_SCREEN] = flight::Z80_SCREEN_LOCAL_MAP;
        self.hdmap.active = true;
        self.sync_hdmap_target();
        self.native_screen_opened();
    }

    /// Выключить нативную отрисовку экрана информации о планете. Экранная
    /// переменная Z80 не трогается.
    pub(super) fn close_sysinfo(&mut self) {
        self.sysinfo.close();
        self.pending_sysinfo = false;
        self.pending_key = None;
        self.sysinfo_from_map = false;
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

    /// Снять блокировку родного ролика, когда Z80 закончил переход.
    pub(super) fn sync_emulator_transition_lock(&mut self) {
        if !flight::z80_in_transition(&self.machine.mem) {
            self.emulator_transition_lock = false;
        }
    }

    /// Открыть нативный вид F1 с началом любой анимации перехода.
    ///
    /// Отсчёт гиперпрыжка может идти под любым экраном: до нуля экран остаётся
    /// на месте (на нём показывается только число), а в момент тоннеля вид
    /// принудительно переключается на передний и проигрывает нативную анимацию.
    /// Стыковка и вылет не имеют отсчёта, поэтому их анимация включается сразу
    /// по флагу `JumpDocking`, в том числе поверх экрана эмулятора.
    pub(super) fn switch_to_flight_on_transition(&mut self) {
        if self.menu || self.flight.in_animation_mode() || self.emulator_transition_lock {
            return;
        }
        let mem = &self.machine.mem;
        if !flight::z80_in_transition(mem) {
            return;
        }
        let countdown = mem.get(flight::Z80_JUMP_COUNTDOWN).copied().unwrap_or(0);
        let intergalactic = mem.get(flight::Z80_JUMP_DOCKING).copied() == Some(2);
        let hyperspace = self.saw_jump_countdown || countdown > 0 || intergalactic;
        // Пока идёт отсчёт, informational-экран не трогаем: он показывает число.
        if hyperspace && countdown > 0 {
            return;
        }
        // `IndDanger` в момент перехода уже переписан обеими сторонами:
        // при вылете оригинал ставит `4`, при стыковке обнуляет его в конце
        // ролика, не обязательно вместе со сбросом флагов. Вылет отличаем по
        // запросу `F1` и по состоянию кадра до эмуляции, а не по текущему
        // значению ячейки.
        let launch = self.pending_launch || self.state.is_docked();
        let kind = if hyperspace {
            flight::TransitionKind::Hyperspace
        } else if launch {
            flight::TransitionKind::Launch
        } else {
            flight::TransitionKind::Docking
        };
        self.show_flight(flight::ViewDir::Forward);
        self.flight.start_transition_kind(kind, countdown);
        self.saw_jump_countdown = false;
        self.pending_launch = false;
    }

    /// Следить за экранными режимами Z80 под нативными экранами без
    /// собственного интерактива (Enter, F7/F8/F9). Когда оригинал уходит с
    /// экрана, нативная надстройка закрывается; пока режим тот же — остаётся.
    pub(super) fn follow_native_screens(&mut self) {
        // Пока эмулятор обрабатывает `L`, его экранная переменная ещё может
        // смениться; закрывать только что открытый нативный экран нельзя.
        if self.pending_station_native.is_some() {
            return;
        }
        // Во время отсчёта гиперпрыжка нативный экран остаётся на месте:
        // переход к тоннелю сам откроет вид F1 (см.
        // `switch_to_flight_on_transition`).
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
        // В нативном полёте информационные экраны показываются поверх симуляции,
        // а эмулятор остаётся в режиме кабины для сохранения управления кораблём.
        if !self.machine.intro_seen
            && !flight::z80_is_docked(&self.machine.mem)
            && (flight::z80_in_cockpit(&self.machine.mem) || flight::z80_in_transition(&self.machine.mem))
        {
            return;
        }
        let screen = self.machine.mem[flight::Z80_WHAT_ON_SCREEN];
        if self.inventory.is_active() && screen != flight::Z80_SCREEN_INVENTORY {
            self.close_inventory();
        }
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
        self.close_inventory();
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

    /// Начать короткий сеанс миссионных проверок из нативного рынка.
    ///
    /// Нативный слой не решает, выдаст ли миссию original: он только видит,
    /// что первая миссия ещё не взята и трюм не занят пассажирами. Саму зону
    /// боев и ранг проверяет `GoMission1` на Z80 после клавиши `2`/`3`.
    pub(super) fn start_market_mission(&mut self, buy: bool) -> bool {
        if self.market_mission.is_some()
            || self.menu
            || self.machine.intro_seen
            || !self.station_native
            || !flight::z80_is_docked(&self.machine.mem)
        {
            return false;
        }
        let mem = self.machine.game_state();
        let mission1 = mem.get(0xD081).copied().unwrap_or(0);
        let ext = mem.get(0xD0B0).copied().unwrap_or(0);
        // `GoMission1` → `TestPilotInZoneCombat` → `TestPilotInZoneWar`
        // сразу возвращают CY=0 вне первой галактики и до начала войны.
        let galaxy = mem.get(0xD06A).copied().unwrap_or(0);
        let war = mem.get(0xD08F).copied().unwrap_or(0);
        if mission1 != 0 || ext & 0x20 != 0 || galaxy != 0 || war < 2 {
            return false;
        }
        self.machine.mission_answer = 0xFF;
        self.machine.install_hook(MISSION_PROMPT_READ_PC, mission_prompt_key, 4);
        self.market_mission = Some(MarketMission {
            buy,
            step: MarketMissionStep::Press,
            accept: true,
            frames: 0,
        });
        self.pending_key = matrix_pos(if buy {
            KeyCode::Digit2
        } else {
            KeyCode::Digit3
        });
        true
    }

    /// Один кадр сеанса. Возвращает `true`, если Z80 исполнялся.
    pub(super) fn step_market_mission(&mut self, z80_frames: u64) -> bool {
        let Some(mut session) = self.market_mission else {
            return false;
        };
        if z80_frames == 0 && session.step != MarketMissionStep::Prompt {
            return false;
        }
        match session.step {
            MarketMissionStep::Press => {
                // Один короткий импульс: original не должен обработать клавишу
                // несколько раз и сам уйти в обычную торговлю.
                self.forward_pending_key();
                self.machine.run_frames(1);
                self.machine.release_all_keys();
                session.step = MarketMissionStep::WaitPrompt;
                session.frames = 0;
            }
            MarketMissionStep::WaitPrompt => {
                self.machine.release_all_keys();
                self.machine.run_frames(1);
                session.frames += 1;
                if self.at_mission_prompt() {
                    let marines = self.machine.mem[0xD062] >= 0x84;
                    self.mission_prompt = Some(MissionPrompt {
                        accept: true,
                        marines,
                    });
                    session.step = MarketMissionStep::Prompt;
                } else if session.frames >= MISSION_WAIT_FRAMES {
                    // Миссии нет: original успел открыть обычное меню. `L`
                    // закрывает его, а сделку применяет нативный экран.
                    self.pending_key = matrix_pos(KeyCode::KeyL);
                    self.forward_pending_key();
                    self.machine.run_frames(2);
                    self.machine.release_all_keys();
                    self.apply_market_trade(session.buy);
                    self.end_market_mission();
                    return true;
                }
            }
            MarketMissionStep::Prompt => {
                // Диалог ждёт ответа: Z80 стоит в цикле `sub_E1F7`.
                return false;
            }
            MarketMissionStep::Answer => {
                self.machine.release_all_keys();
                // Ответ уже лежит в `Spectrum::mission_answer`: хук подставит его вместо
                // чтения `PressKeyPrnSymbol` в цикле `sub_E1F7`.
                self.machine.mission_answer = if session.accept { 0x59 } else { 0x4E };
                self.machine.run_frames(1);
                session.step = MarketMissionStep::WaitInventory;
                session.frames = 0;
            }
            MarketMissionStep::WaitInventory => {
                self.machine.release_all_keys();
                self.machine.run_frames(1);
                session.frames += 1;
                if self.machine.mem[flight::Z80_WHAT_ON_SCREEN]
                    == flight::Z80_SCREEN_INVENTORY
                    || session.frames >= MISSION_WAIT_FRAMES
                {
                    self.end_market_mission();
                    self.open_inventory();
                    return true;
                }
            }
        }
        self.market_mission = Some(session);
        true
    }

    /// Ответ нативного диалога: передать `Y`/`N` original на следующем кадре.
    pub(super) fn answer_mission_prompt(&mut self, accept: bool) {
        if let Some(prompt) = self.mission_prompt.as_mut() {
            prompt.accept = accept;
        }
        if let Some(session) = self.market_mission.as_mut() {
            session.accept = accept;
            session.step = MarketMissionStep::Answer;
        }
    }

    /// Завершить сеанс и снять перехват чтения символа ответа.
    fn end_market_mission(&mut self) {
        self.machine.remove_hook(MISSION_PROMPT_READ_PC);
        self.machine.mission_answer = 0xFF;
        self.mission_prompt = None;
        self.market_mission = None;
    }

    /// Переключить подсветку в диалоге стрелками.
    pub(super) fn toggle_mission_prompt(&mut self) {
        if let Some(prompt) = self.mission_prompt.as_mut() {
            prompt.accept = !prompt.accept;
        }
    }

    /// Original стоит в цикле ожидания символа `sub_E1F7`.
    fn at_mission_prompt(&self) -> bool {
        MISSION_PROMPT_PC.contains(&self.machine.cpu.pc)
            && self.machine.mem.get(PRESS_KEY_SYMBOL).copied() == Some(0xFF)
    }

    /// Применить сделку нативным экраном, когда миссионный сеанс её не выполнил.
    fn apply_market_trade(&mut self, buy: bool) {
        let docked = flight::z80_is_docked(&self.machine.mem);
        let system = self.state.system.clone();
        self.market
            .refresh(&self.machine.mem, market::KLADOV_MARKET, docked, &system);
        if buy {
            self.market.buy(&mut self.machine.mem, docked);
        } else {
            self.market.sell(&mut self.machine.mem, docked);
        }
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

    /// Снять нативные экраны, которые могут лежать поверх вида из кабины.
    ///
    /// Карта, закупка и информационные экраны имеют приоритет в
    /// [`crate::state::GameState`], поэтому одного открытого `flight` мало:
    /// пока надстройка активна, кадр по-прежнему рисует её, а не полёт.
    fn close_flight_overlays(&mut self) {
        self.close_hdmap();
        self.shop.close();
        self.close_inventory();
        self.close_status();
        self.close_sysinfo();
        self.close_market();
        self.pending_status = false;
        self.pending_sysinfo = false;
        self.pending_shop = false;
    }

    /// Открыть нативный экран полёта поверх текущего состояния игры.
    fn open_flight(&mut self, dir: flight::ViewDir) {
        self.flight_emulator = false;
        let planet_color = world::current_planet_color(&self.machine.mem);
        self.flight.open(planet_color);
        self.flight.set_view_dir(dir);
        self.close_flight_overlays();
        self.menu = false;
        self.pending_key = None;
        self.pending_flight = None;
        self.pending_launch = false;
    }

    /// Показать нативный вид: уже открытый экран только меняет направление,
    /// чтобы не сбрасывать звёздное поле и начатый переход.
    pub(super) fn show_flight(&mut self, dir: flight::ViewDir) {
        self.flight_emulator = false;
        if self.flight.active {
            self.close_flight_overlays();
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
            self.flight_emulator = false;
            return;
        }
        let launch = self.pending_launch;
        self.flight.follow_z80_screen(&self.machine.mem, launch);
        if launch && self.flight.transition_kind() == Some(flight::TransitionKind::Launch) {
            self.pending_launch = false;
        }
    }

    /// Открыть нативный вид, отложенный до полёта или перехода (см.
    /// `pending_flight`). Если оригинал уже начал вылет/прыжок/стыковку,
    /// рендер включается сразу и проигрывает нативную анимацию.
    pub(super) fn open_pending_flight(&mut self) {
        let Some(dir) = self.pending_flight else {
            return;
        };
        let in_transition = flight::z80_in_transition(&self.machine.mem);
        let in_cockpit = flight::z80_in_cockpit(&self.machine.mem);
        let docked = flight::z80_is_docked(&self.machine.mem);

        // После стыковки Z80 ещё несколько кадров показывает кокпит при
        // IndDanger==0, прежде чем перейти на экран станции. В этом окне
        // клавиша '1' означает вид вперёд, а не вылет; ждём, пока Z80 дойдёт
        // до станционного экрана.
        if self.pending_launch && docked && in_cockpit && !in_transition {
            return;
        }

        if in_cockpit || in_transition {
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

        // На станции запуск откладывается до готовности Z80: отправляем '1'
        // только когда он уже на станционном экране, но переход ещё не начался.
        if self.pending_launch && docked && !in_transition {
            self.pending_key = matrix_pos(KeyCode::Digit1);
            return;
        }

        // Переход закончился, но полёт так и не начался (например, стыковка
        // привела на станцию): запрос больше не нужен.
        if docked && !self.pending_launch {
            self.pending_flight = None;
        }
    }

    /// В полёте нативный вид — основной интерфейс. Если не открыт ни один
    /// информационный экран, показываем нативный вид кабины и синхронизируем
    /// направление с экраном Z80.
    pub(super) fn ensure_native_flight_in_space(&mut self) {
        // Под `LOADING` Z80 ещё может стоять в кабине, хотя сохранение уже
        // уводит его на станцию. Открытый здесь нативный вид остался бы и
        // после хука: он мешает входу в нативную станцию (не уходит `L`) и
        // заставляет `F1` переключать вид вместо вылета.
        if self.loading
            || self.machine.intro_seen
            || self.station_native
            || self.pending_station_native.is_some()
        {
            return;
        }
        // Запрос вылета должен дождаться `switch_to_flight_on_transition()`:
        // иначе `show_flight()` съест `pending_launch` до старта анимации и
        // переход будет принят за стыковку.
        if self.pending_launch || self.pending_flight.is_some() {
            return;
        }
        if flight::z80_is_docked(&self.machine.mem) {
            return;
        }
        // Во время анимации можно временно скрыть нативный вид клавишей `1`,
        // чтобы увидеть ролик эмулятора.
        if flight::z80_in_transition(&self.machine.mem)
            && !self.flight.active
            && self.flight.transition_kind().is_some()
        {
            return;
        }
        let in_cockpit = flight::z80_in_cockpit(&self.machine.mem);
        let in_transition = flight::z80_in_transition(&self.machine.mem);
        let info_screen = self.hdmap.active
            || self.status.active
            || self.sysinfo.active
            || self.market.native
            || self.shop.active;
        if !in_cockpit && !in_transition {
            return;
        }
        if !info_screen && !self.flight.active && !self.flight_emulator {
            let dir = match self.machine.mem[flight::Z80_WHAT_ON_SCREEN] {
                1 => flight::ViewDir::Back,
                2 => flight::ViewDir::Left,
                3 => flight::ViewDir::Right,
                _ => flight::ViewDir::Forward,
            };
            self.show_flight(dir);
        }
        // Во время отсчёта Z80 ещё держит вид из кабины, а направление выбирает
        // игрок: синхронизация с `WhatOnScreen` сбросила бы F2-F4 вперёд.
        if self.flight.active
            && !self.flight.in_animation_mode()
            && !flight::z80_in_transition(&self.machine.mem)
        {
            let dir = match self.machine.mem[flight::Z80_WHAT_ON_SCREEN] {
                1 => flight::ViewDir::Back,
                2 => flight::ViewDir::Left,
                3 => flight::ViewDir::Right,
                _ => flight::ViewDir::Forward,
            };
            self.flight.set_view_dir(dir);
        }
    }
}
