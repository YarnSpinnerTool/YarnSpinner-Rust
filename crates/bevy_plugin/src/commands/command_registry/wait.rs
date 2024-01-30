//! In an ideal world, this would just be a new thread doing `sleep`.
//! Alas, Wasm forces us to do this

use crate::prelude::YarnSpinnerSystemSet;
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub(crate) fn wait_command_plugin(app: &mut App) {
    app.init_resource::<Wait>()
        .add_systems(Update, update_wait.in_set(YarnSpinnerSystemSet));
}

#[derive(Debug, Clone, Resource, Default)]
pub(crate) struct Wait(HashMap<Duration, WaitPeriod>);

#[derive(Debug, Clone)]
pub(crate) struct WaitPeriod {
    duration: Duration,
    done: Arc<AtomicBool>,
}

impl Wait {
    pub(crate) fn add(&mut self, duration: Duration) -> Arc<AtomicBool> {
        let done = Arc::new(AtomicBool::new(false));
        self.0.insert(
            duration,
            WaitPeriod {
                duration,
                done: done.clone(),
            },
        );
        done
    }
}

pub(crate) fn update_wait(time: Res<Time>, mut wait: ResMut<Wait>) {
    for period in wait.0.values_mut() {
        if period.duration <= time.delta() {
            period.duration = Duration::from_secs(0);
            period.done.store(true, Ordering::Relaxed);
        } else {
            period.duration -= time.delta();
        }
    }
    wait.0.remove(&Duration::from_secs(0));
}
