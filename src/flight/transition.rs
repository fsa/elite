//! Переходы между состояниями полёта: вылет, гиперпрыжок и стыковка.
//!
//! Оригинал показывает собственный ролик, а нативный вид рисует его сам.
//! Модуль хранит только состояние анимации и отсчёт, который ведёт Z80;
//! сама отрисовка живёт в [`super::render::transition`].
//!
//! Гиперпрыжок и переходы станции — разные анимации с разными точками
//! начала и конца. Они лежат в отдельных полях и не используют общее
//! состояние: правки ролика вылета не должны менять поведение прыжка.

use super::memory::{Z80_FLG_JUMP_DOCKING, Z80_JUMP_COUNTDOWN, Z80_JUMP_DOCKING};

/// Какой переход сейчас показывает нативный рендер.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransitionKind {
    /// Вылет со станции: белый тоннель, в конце которого видна планета.
    Launch,
    /// Гиперпрыжок: сначала отсчёт, затем тоннель.
    Hyperspace,
    /// Стыковка: тот же белый тоннель, но без планеты в конце.
    Docking,
}

/// Переход станции: вылет или стыковка.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum StationKind {
    Launch,
    Docking,
}

/// Состояние ролика станции.
#[derive(Clone, Copy, Debug)]
pub(super) struct StationTransition {
    pub(super) kind: StationKind,
    /// Время с начала тоннеля.
    pub(super) elapsed: f32,
}

/// Состояние гиперпрыжка. Отсчёт и тоннель не зависят от роликов станции.
#[derive(Clone, Copy, Debug)]
pub(super) struct HyperspaceTransition {
    /// Последнее значение `JumpCountdown` из живой памяти.
    pub(super) countdown: u8,
    /// Отсчёт закончился, рисуем сам прыжок (тоннель).
    pub(super) warp: bool,
    /// Время с начала перехода.
    pub(super) elapsed: f32,
    /// Время с начала тоннеля (после отсчёта).
    pub(super) warp_elapsed: f32,
    /// Скорость корабля в момент начала тоннеля.
    ///
    /// Нужна для наложенной анимации звёзд: они стартуют с этой скорости и
    /// плавно возвращаются к ней к концу прыжка.
    pub(super) warp_initial_speed: Option<f32>,
}

/// Две независимые анимации: ролик станции и гиперпрыжок.
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct TransitionState {
    pub(super) station: Option<StationTransition>,
    pub(super) hyperspace: Option<HyperspaceTransition>,
}

impl TransitionState {
    pub(super) fn new() -> Self {
        Self::default()
    }

    /// Сбросить обе анимации.
    pub(super) fn reset(&mut self) {
        *self = Self::default();
    }

    pub(super) fn reset_station(&mut self) {
        self.station = None;
    }

    pub(super) fn reset_hyperspace(&mut self) {
        self.hyperspace = None;
    }

    /// Начать ролик станции. Одновременных анимаций не бывает, поэтому
    /// прыжок, если он ещё висел, сбрасывается.
    pub(super) fn start_station(&mut self, kind: StationKind) {
        self.hyperspace = None;
        self.station = Some(StationTransition {
            kind,
            elapsed: 0.0,
        });
    }

    /// Начать гиперпрыжок. Ролик станции, который мог остаться от прошлого
    /// перехода, сбрасывается: он не должен блокировать тоннель.
    pub(super) fn start_hyperspace(&mut self, countdown: u8) {
        self.station = None;
        self.hyperspace = Some(HyperspaceTransition {
            countdown,
            // Отсчёт гиперпрыжка сначала показывает обычный полёт. Если
            // анимация включается уже после отсчёта, живые флаги перехода
            // подтверждают, что нужно сразу показывать тоннель.
            warp: countdown == 0,
            elapsed: 0.0,
            warp_elapsed: 0.0,
            warp_initial_speed: None,
        });
    }

    /// Начать переход, вид которого известен заранее.
    pub(super) fn start_kind(&mut self, kind: TransitionKind, countdown: u8) {
        match kind {
            TransitionKind::Launch => self.start_station(StationKind::Launch),
            TransitionKind::Docking => self.start_station(StationKind::Docking),
            TransitionKind::Hyperspace => self.start_hyperspace(countdown),
        }
    }

    /// Начать переход по памяти Z80. `launch` отличает вылет от стыковки.
    pub(super) fn start(&mut self, launch: bool, mem: &[u8]) {
        let countdown = mem.get(Z80_JUMP_COUNTDOWN).copied().unwrap_or(0);
        let intergalactic = mem.get(Z80_JUMP_DOCKING).copied() == Some(2);
        // Признак прыжка по живой памяти важнее запроса вылета: `launch` может
        // остаться после предыдущей станции, а `JumpDocking == 2` и отсчёт
        // однозначно означают новый гиперпереход.
        if countdown > 0 || intergalactic {
            self.start_hyperspace(countdown);
        } else if launch {
            self.start_station(StationKind::Launch);
        } else {
            self.start_station(StationKind::Docking);
        }
    }

    /// Обновить отсчёт гиперпрыжка из живой памяти. Ролик станции не трогаем.
    pub(super) fn sync_hyperspace(&mut self, mem: &[u8]) {
        if let Some(t) = &mut self.hyperspace {
            t.countdown = mem.get(Z80_JUMP_COUNTDOWN).copied().unwrap_or(0);
            // Конец отсчёта в живой памяти приходит раньше, чем игра гасит
            // `JumpDocking` и `FlgJumpDocking`. Пока флаги взведены, тоннель
            // уже идёт, но переход ещё активен. Если обнулить его здесь, то
            // следующий `follow_z80_screen` не зафиксирует передний вид, а
            // синхронизация с `WhatOnScreen` вернёт боковую проекцию — звёзды
            // на выходе улетят горизонтально.
            let active = mem.get(Z80_JUMP_DOCKING).copied().unwrap_or(0) != 0
                || mem.get(Z80_FLG_JUMP_DOCKING).copied().unwrap_or(0) != 0;
            // Снимок сцены может ещё не содержать новый проход (в начале
            // прыжка `PrintCompass` не вызывается), поэтому ненулевой
            // отсчёт из живой памяти должен откатывать ложный тоннель.
            t.warp = t.countdown == 0 && active;
            if !t.warp {
                t.warp_initial_speed = None;
            }
        }
    }

    /// Продвинуть таймеры активной анимации.
    pub(super) fn tick(&mut self, dt: f32) {
        if let Some(t) = &mut self.station {
            t.elapsed += dt;
        }
        if let Some(t) = &mut self.hyperspace {
            t.elapsed += dt;
            if t.warp {
                t.warp_elapsed += dt;
            }
        }
    }

    pub(super) fn station_kind(&self) -> Option<StationKind> {
        self.station.map(|t| t.kind)
    }

    pub(super) fn in_transition(&self) -> bool {
        self.station.is_some() || self.hyperspace.is_some()
    }

    /// Идёт ли уже сама анимация, а не ожидание отсчёта гиперпрыжка.
    pub(super) fn in_animation(&self) -> bool {
        self.station.is_some() || self.hyperspace.is_some_and(|t| t.warp)
    }

    /// Какой переход сейчас идёт (для тестов и отладки).
    pub(super) fn kind(&self) -> Option<TransitionKind> {
        if let Some(t) = self.station {
            return Some(match t.kind {
                StationKind::Launch => TransitionKind::Launch,
                StationKind::Docking => TransitionKind::Docking,
            });
        }
        self.hyperspace.map(|_| TransitionKind::Hyperspace)
    }
}
