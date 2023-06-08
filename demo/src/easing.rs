use bevy::prelude::*;
use bevy::utils::Instant;
use std::f32::consts::PI;
use std::fmt::Debug;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone, Resource)]
pub(crate) struct EasedChange<T: Debug + Clone> {
    pub(crate) from: T,
    pub(crate) to: T,
    pub(crate) duration: f32,
    pub(crate) done: Arc<AtomicBool>,
    pub(crate) start_time: Instant,
}
impl<T: Debug + Clone> EasedChange<T> {
    pub(crate) fn new(from: T, to: T, duration: f32) -> Self {
        Self {
            from,
            to,
            duration,
            done: Arc::new(AtomicBool::new(false)),
            start_time: Instant::now(),
        }
    }
    pub(crate) fn input(&self) -> f32 {
        (self.start_time.elapsed().as_secs_f32() / self.duration).min(1.0)
    }

    pub(crate) fn is_done(&self) -> bool {
        self.input() >= 0.99
    }

    pub(crate) fn set_done(&mut self) {
        self.done.store(true, Ordering::Relaxed);
    }

    /// Source: <https://github.com/facebook/react-native/blob/main/packages/react-native/Libraries/Animated/Easing.js#L165>
    pub(crate) fn elastic(&self) -> f32 {
        const C4: f32 = (2.0 * std::f32::consts::PI) / 3.0;

        (2.0_f32).powf(-10.0 * x) * ((x * 10.0 - 0.75) * C4).sin() + 1.0
    }

    pub(crate) fn smooth_start(&self) -> f32 {
        let x = self.input();
        x * x * x
    }

    pub(crate) fn smooth_end(&self) -> f32 {
        let x = self.input();
        let n = 1.0 - x;
        1.0 - n * n * n
    }
}
