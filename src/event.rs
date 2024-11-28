//! [`Event`]s that are triggered by this crate.

use bevy_ecs::event::Event;

/// [`Event`] that is triggered when a [`Timer`] on an entity just finished.
/// The [`Trigger`] will contain the [`Component`] that identifies the timer.
///
/// # Example
///
/// ```
/// # use bevy_ecs::prelude::*;
/// # use bevy_observed_timers::prelude::*;
/// # #[derive(Component)]
/// # struct MyComponent;
/// # let mut world = World::new();
/// world.add_observer(|_: Trigger<OnTimerFinished, MyComponent>| {
///     // ...
/// });
/// ```
///
/// [`Timer`]: bevy_time::Timer
/// [`Trigger`]: bevy_ecs::observer::Trigger
/// [`Component`]: bevy_ecs::component::Component
#[derive(Event)]
pub struct OnTimerFinished;

/// [`Event`] that is triggered when a [`Timer`] is manually cancelled via
/// [`cancel_timer`](crate::command::EntityCommandTimersExt::cancel_timer).
///
/// # Example
///
/// ```
/// # use bevy_ecs::prelude::*;
/// # use bevy_observed_timers::prelude::*;
/// # #[derive(Component)]
/// # struct MyComponent;
/// # let mut world = World::new();
/// world.add_observer(|_: Trigger<OnTimerCancelled, MyComponent>| {
///    // ...
/// });
/// ```
///
/// [`Timer`]: bevy_time::Timer
#[derive(Event)]
pub struct OnTimerCancelled;
