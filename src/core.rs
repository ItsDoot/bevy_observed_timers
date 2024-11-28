//! The core functionality of the crate, providing the [`Timers`] component and
//! the [`tick_entity_timers`] system.

use bevy_ecs::{
    component::{Component, ComponentId},
    entity::Entity,
    system::{ParallelCommands, Query, Res},
};
use bevy_time::{Time, Timer, TimerMode};
use indexmap::IndexMap;

use crate::{event::OnTimerFinished, TargetBoth};

/// [`Component`] that stores [`Timer`]s for an entity, tagged by [`Component`]s.
///
/// Although this component can be accessed directly, it is recommended to use
/// the [`EntityCommandTimersExt`] trait on [`EntityCommands`] to interact with
/// it more easily.
///
/// [`EntityCommandTimersExt`]: crate::command::EntityCommandTimersExt
/// [`EntityCommands`]: bevy_ecs::system::EntityCommands
#[derive(Component, Default)]
pub struct Timers(IndexMap<ComponentId, Timer>);

impl Timers {
    /// Create a new Timers component.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a reference to the [`Timer`] with the given [`ComponentId`].
    pub fn get(&self, component: ComponentId) -> Option<&Timer> {
        self.0.get(&component)
    }

    /// Returns a mutable reference to the [`Timer`] with the given
    /// [`ComponentId`].
    pub fn get_mut(&mut self, component: ComponentId) -> Option<&mut Timer> {
        self.0.get_mut(&component)
    }

    /// Insert a new [`Timer`] identified by the given [`ComponentId`].
    pub fn insert(&mut self, component: ComponentId, timer: Timer) {
        self.0.insert(component, timer);
    }

    /// Remove the [`Timer`] identified by the given [`ComponentId`].
    pub fn remove(&mut self, component: ComponentId) -> Option<Timer> {
        self.0.swap_remove(&component)
    }

    /// Returns an iterator over the [`Timer`]s and their [`ComponentId`]s.
    pub fn iter(&self) -> impl Iterator<Item = (&ComponentId, &Timer)> {
        self.0.iter()
    }

    /// Returns a mutable iterator over the [`Timer`]s and their [`ComponentId`]s.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&ComponentId, &mut Timer)> {
        self.0.iter_mut()
    }
}

/// [`System`] that ticks [`Timers`] on entities, and triggers
/// [`OnTimerFinished`] when a timer just finished.
///
/// This system can be scheduled with the [`ScheduleTimerTickPlugin`] plugin,
/// or added to a schedule manually.
///
/// [`System`]: bevy_ecs::system::System
/// [`ScheduleTimerTickPlugin`]: crate::plugin::ScheduleTimerTickPlugin
pub fn tick_entity_timers(
    mut timers: Query<(Entity, &mut Timers)>,
    time: Res<Time>,
    par_commands: ParallelCommands,
) {
    timers.par_iter_mut().for_each(|(entity, mut timers)| {
        let mut finished_timers = Vec::new();
        par_commands.command_scope(|mut commands| {
            for (&component, timer) in timers.0.iter_mut() {
                if timer.tick(time.delta()).just_finished() {
                    commands.trigger_targets(OnTimerFinished, TargetBoth(entity, component));
                    if timer.mode() == TimerMode::Once {
                        finished_timers.push(component);
                    }
                }
            }
        });
        for component in finished_timers {
            timers.0.swap_remove(&component);
        }
    });
}
