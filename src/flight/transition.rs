//! Переходы между состояниями полёта: вылет, гиперпрыжок и стыковка.
//!
//! Оригинал показывает собственный ролик, а нативный вид рисует его сам.
//! Модуль хранит только состояние анимации и отсчёт, который ведёт Z80;
//! сама отрисовка живёт в [`super::render::transition`].

use super::memory::{Z80_JUMP_COUNTDOWN, Z80_JUMP_DOCKING};

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
    /// Скорость корабля в момент начала тоннеля гиперпрыжка.
    ///
    /// Нужна для наложенной анимации звёзд: они стартуют с этой скорости и
    /// плавно возвращаются к ней к концу прыжка.
    pub(super) warp_initial_speed: Option<f32>,
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
            warp_initial_speed: None,
        }
    }
}

/// Текущий переход.
pub(super) struct TransitionState {
    pub(super) current: Option<Transition>,
}

impl TransitionState {
    pub(super) fn new() -> Self {
        TransitionState { current: None }
    }

    pub(super) fn reset(&mut self) {
        self.current = None;
    }

    /// Начать нативную анимацию перехода.
    ///
    /// `launch` передаётся из `F1` на станции: по памяти игры вылет и стыковку
    /// не отличить (оба взводят `JumpDocking = 1`), а вид должен знать, что
    /// показывать — уход станции или тоннель.
    pub(super) fn start(&mut self, launch: bool, mem: &[u8]) {
        let countdown = mem.get(Z80_JUMP_COUNTDOWN).copied().unwrap_or(0);
        let intergalactic = mem.get(Z80_JUMP_DOCKING).copied() == Some(2);
        let kind = if launch {
            TransitionKind::Launch
        } else if countdown > 0 || intergalactic {
            TransitionKind::Hyperspace
        } else {
            TransitionKind::Docking
        };
        self.start_kind(kind, countdown);
    }

    /// Начать анимацию с явно известным видом. Нужно, когда приложение
    /// наблюдало отсчёт гиперпрыжка: после его окончания память уже не
    /// отличает прыжок от стыковки.
    pub(super) fn start_kind(&mut self, kind: TransitionKind, countdown: u8) {
        if self.current.is_some() {
            return;
        }
        self.current = Some(Transition::new(kind, countdown));
    }

    /// Обновить отсчёт по памяти игры.
    pub(super) fn sync(&mut self, mem: &[u8]) {
        // Отсчёт гиперпрыжка ведёт Z80. Читаем его каждый кадр: пока число
        // больше нуля, нативный рендер показывает обычный полёт и цифру, а
        // после нуля переключается на тоннель.
        if let Some(t) = &mut self.current {
            if t.kind == TransitionKind::Hyperspace {
                t.countdown = mem.get(Z80_JUMP_COUNTDOWN).copied().unwrap_or(0);
                // Снимок сцены может ещё не содержать новый проход (в начале
                // прыжка `PrintCompass` не вызывается), поэтому ненулевой
                // отсчёт из живой памяти должен откатывать ложный тоннель.
                t.warp = t.countdown == 0;
                if !t.warp {
                    t.warp_initial_speed = None;
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

    /// Идёт ли уже сама анимация, а не ожидание отсчёта гиперпрыжка.
    ///
    /// Гиперпрыжок до обнуления счётчика показывает обычный полёт с цифрой,
    /// и нативный вид в это время может переключаться. Вылет и стыковка
    /// анимацию не откладывают.
    pub(super) fn in_animation(&self) -> bool {
        self.current
            .is_some_and(|t| t.kind != TransitionKind::Hyperspace || t.warp)
    }

    /// Какой переход сейчас идёт (для тестов и отладки).
    #[allow(dead_code)]
    pub(super) fn kind(&self) -> Option<TransitionKind> {
        self.current.map(|t| t.kind)
    }
}
