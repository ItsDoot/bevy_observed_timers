//! Provides a Bevy [`Plugin`] that schedules the [`tick_entity_timers`] system.

use bevy_app::{App, FixedPreUpdate, Plugin, PreUpdate};
use bevy_ecs::schedule::{InternedScheduleLabel, ScheduleLabel};

use crate::core::tick_entity_timers;

/// [`Plugin`] that schedules the [`tick_entity_timers`] system in a given
/// schedule.
pub struct ScheduleTimerTickPlugin {
    /// The schedule in which the [`tick_entity_timers`] system is scheduled.
    pub tick_in: InternedScheduleLabel,
}

impl ScheduleTimerTickPlugin {
    /// Creates a new plugin that ticks entity timers in the given schedule.
    pub fn new(schedule: impl ScheduleLabel) -> Self {
        Self {
            tick_in: schedule.intern(),
        }
    }

    /// Creates a new plugin that ticks entity timers in the [`PreUpdate`]
    /// schedule.
    pub fn pre_update() -> Self {
        Self {
            tick_in: PreUpdate.intern(),
        }
    }

    /// Creates a new plugin that ticks entity timers in the [`FixedPreUpdate`]
    /// schedule.
    pub fn fixed_pre_update() -> Self {
        Self {
            tick_in: FixedPreUpdate.intern(),
        }
    }
}

impl Plugin for ScheduleTimerTickPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(self.tick_in, tick_entity_timers);
    }
}
