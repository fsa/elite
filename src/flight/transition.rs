//! Переходы между состояниями полёта: вылет, гиперпрыжок и стыковка.
//!
//! Оригинал показывает собственный ролик, а нативный вид рисует его сам.
//! Модуль хранит только состояние анимации и отсчёт, который ведёт Z80;
//! сама отрисовка живёт в [`super::render::transition`].

use super::memory::{self, Z80_JUMP_COUNTDOWN, Z80_JUMP_DOCKING};

/// Сколько секунд длится нативная анимация вылета со станции.
///
/// Оригинал проигрывает тоннель примерно 1.2 с (36 + 11 кадров), поэтому
/// к моменту, когда Z80 сбросит `JumpDocking`, станция успевает уйти, а
/// звёзды — разогнаться.
pub(super) const LAUNCH_TIME: f32 = 1.2;

/// Какой переход сейчас показывает нативный рендер.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransitionKind {
    /// Вылет со станции: станция уходит назад, звёзды разгоняются.
    Launch,
    /// Гиперпрыжок: сначала отсчёт, затем тоннель и полосы звёзд.
    Hyperspace,
    /// Стыковка: тот же тоннель оригинала, но без отсчёта.
    Docking,
}

/// Состояние нативной анимации перехода.
#[derive(Clone, Copy, Debug)]
pub(super) struct Transition {
    pub(super) kind: TransitionKind,
    /// Время с начала перехода.
    pub(super) elapsed: f32,
    /// Последнее значение `JumpCountdown` из памяти игры.
    pub(super) countdown: u8,
    /// Отсчёт закончился, рисуем сам прыжок (тоннель).
    pub(super) warp: bool,
    /// Время с начала тоннеля (для гиперпрыжка — после отсчёта).
    pub(super) warp_elapsed: f32,
}

impl Transition {
    pub(super) fn new(kind: TransitionKind, countdown: u8) -> Self {
        // Вылет начинается сразу со станции, стыковка — сразу с тоннеля.
        // Гиперпрыжок сначала показывает обычный полёт и отсчёт.
        let warp =
            kind != TransitionKind::Launch && (kind == TransitionKind::Docking || countdown == 0);
        Transition {
            kind,
            elapsed: 0.0,
            countdown,
            warp,
            warp_elapsed: 0.0,
        }
    }
}

/// Текущий переход и имя цели гиперпрыжка.
pub(super) struct TransitionState {
    pub(super) current: Option<Transition>,
    /// Имя планеты для надписи «Hyperspace -- <имя>» во время отсчёта.
    planet_name: String,
}

impl TransitionState {
    pub(super) fn new() -> Self {
        TransitionState {
            current: None,
            planet_name: String::new(),
        }
    }

    pub(super) fn reset(&mut self) {
        self.current = None;
        self.planet_name.clear();
    }

    /// Начать нативную анимацию перехода.
    ///
    /// `launch` передаётся из `F1` на станции: по памяти игры вылет и стыковку
    /// не отличить (оба взводят `JumpDocking = 1`), а вид должен знать, что
    /// показывать — уход станции или тоннель.
    pub(super) fn start(&mut self, launch: bool, mem: &[u8]) {
        if self.current.is_some() {
            return;
        }
        let countdown = mem.get(Z80_JUMP_COUNTDOWN).copied().unwrap_or(0);
        let intergalactic = mem.get(Z80_JUMP_DOCKING).copied() == Some(2);
        let kind = if launch {
            TransitionKind::Launch
        } else if countdown > 0 || intergalactic {
            TransitionKind::Hyperspace
        } else {
            TransitionKind::Docking
        };
        if kind == TransitionKind::Hyperspace && countdown > 0 {
            self.planet_name = memory::planet_name_from_z80(mem);
        }
        self.current = Some(Transition::new(kind, countdown));
    }

    /// Обновить отсчёт и имя цели по памяти игры.
    pub(super) fn sync(&mut self, mem: &[u8]) {
        // Отсчёт гиперпрыжка ведёт Z80. Читаем его каждый кадр: пока число
        // больше нуля, нативный рендер показывает обычный полёт и цифру, а
        // после нуля переключается на тоннель.
        if let Some(t) = &mut self.current {
            if t.kind == TransitionKind::Hyperspace {
                t.countdown = mem.get(Z80_JUMP_COUNTDOWN).copied().unwrap_or(0);
                if t.countdown == 0 {
                    t.warp = true;
                } else {
                    // Во время отсчёта игра держит в `PlanetName` имя цели
                    // прыжка — показываем его внизу (как на заставке).
                    self.planet_name = memory::planet_name_from_z80(mem);
                }
            }
        }
    }

    /// Продвинуть таймеры анимации.
    pub(super) fn tick(&mut self, dt: f32) {
        if let Some(t) = &mut self.current {
            t.elapsed += dt;
            if t.warp {
                t.warp_elapsed += dt;
            }
        }
    }

    /// Идёт ли сейчас нативный переход (вылет, прыжок или стыковка).
    pub(super) fn in_transition(&self) -> bool {
        self.current.is_some()
    }

    /// Какой переход сейчас идёт (для тестов и отладки).
    #[allow(dead_code)]
    pub(super) fn kind(&self) -> Option<TransitionKind> {
        self.current.map(|t| t.kind)
    }

    pub(super) fn planet_name(&self) -> &str {
        &self.planet_name
    }
}
