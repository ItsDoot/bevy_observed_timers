//! [`EntityCommand`]s for managing [`Timer`]s on entities.

use core::marker::PhantomData;

use bevy_ecs::{
    component::Component,
    entity::Entity,
    system::{EntityCommand, EntityCommands},
    world::World,
};
use bevy_time::Timer;

use crate::{core::Timers, event::OnTimerCancelled, TargetBoth};

/// [`EntityCommands`] extension trait that provides methods for starting,
/// resetting, pausing, unpausing, and cancelling timers on entities.
pub trait EntityCommandTimersExt {
    /// Start a [`Timer`] on the target entity. The [`Component`] `T` is used as
    /// a tag to identify the timer.
    ///
    /// If a [`Timer`] with the same tag already exists, it will be replaced.
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_ecs::prelude::*;
    /// # use bevy_observed_timers::prelude::*;
    /// # use bevy_time::prelude::*;
    /// # #[derive(Component)]
    /// # struct Regenerate;
    /// # let mut world = World::new();
    /// # let mut commands = world.commands();
    /// # let e1 = commands.spawn_empty().id();
    /// commands.entity(e1)
    ///     .start_timer::<Regenerate>(Timer::from_seconds(5., TimerMode::Repeating));
    /// ```
    fn start_timer<T: Component>(&mut self, timer: Timer) -> &mut Self;

    /// Reset a [`Timer`] on the target entity. The [`Component`] `T` is used as
    /// a tag to identify the timer.
    ///
    /// If the timer does not exist, this command does nothing.
    /// Calls [`Timer::reset`] on the timer.
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_ecs::prelude::*;
    /// # use bevy_observed_timers::prelude::*;
    /// # use bevy_time::prelude::*;
    /// # #[derive(Component)]
    /// # struct Regenerate;
    /// # let mut world = World::new();
    /// # let mut commands = world.commands();
    /// # let e1 = commands.spawn_empty().id();
    /// commands.entity(e1).reset_timer::<Regenerate>();
    /// ```
    fn reset_timer<T: Component>(&mut self) -> &mut Self;

    /// Pause a [`Timer`] on the target entity. The [`Component`] `T` is used as
    /// a tag to identify the timer.
    ///
    /// If the timer does not exist, this command does nothing.
    /// Calls [`Timer::pause`] on the timer.
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_ecs::prelude::*;
    /// # use bevy_observed_timers::prelude::*;
    /// # use bevy_time::prelude::*;
    /// # #[derive(Component)]
    /// # struct Regenerate;
    /// # let mut world = World::new();
    /// # let mut commands = world.commands();
    /// # let e1 = commands.spawn_empty().id();
    /// commands.entity(e1).pause_timer::<Regenerate>();
    /// ```
    fn pause_timer<T: Component>(&mut self) -> &mut Self;

    /// Unpause a [`Timer`] on the target entity. The [`Component`] `T` is used
    /// as a tag to identify the timer.
    ///
    /// If the timer does not exist, this command does nothing.
    /// Calls [`Timer::unpause`] on the timer.
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_ecs::prelude::*;
    /// # use bevy_observed_timers::prelude::*;
    /// # use bevy_time::prelude::*;
    /// # #[derive(Component)]
    /// # struct Regenerate;
    /// # let mut world = World::new();
    /// # let mut commands = world.commands();
    /// # let e1 = commands.spawn_empty().id();
    /// commands.entity(e1).unpause_timer::<Regenerate>();
    /// ```
    fn unpause_timer<T: Component>(&mut self) -> &mut Self;

    /// Cancel a [`Timer`] on the target entity. The [`Component`] `T` is used
    /// as a tag to identify the timer.
    ///
    /// If the timer does not exist, this command does nothing.
    /// Removes the [`Timer`] from the entity and triggers [`OnTimerCancelled`].
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_ecs::prelude::*;
    /// # use bevy_observed_timers::prelude::*;
    /// # use bevy_time::prelude::*;
    /// # #[derive(Component)]
    /// # struct Regenerate;
    /// # let mut world = World::new();
    /// # let mut commands = world.commands();
    /// # let e1 = commands.spawn_empty().id();
    /// commands.entity(e1).cancel_timer::<Regenerate>();
    /// ```
    fn cancel_timer<T: Component>(&mut self) -> &mut Self;
}

impl EntityCommandTimersExt for EntityCommands<'_> {
    fn start_timer<T: Component>(&mut self, timer: Timer) -> &mut Self {
        self.queue(StartTimer::<T>::new(timer))
    }

    fn reset_timer<T: Component>(&mut self) -> &mut Self {
        self.queue(ResetTimer::<T>::default())
    }

    fn pause_timer<T: Component>(&mut self) -> &mut Self {
        self.queue(PauseTimer::<T>::default())
    }

    fn unpause_timer<T: Component>(&mut self) -> &mut Self {
        self.queue(UnpauseTimer::<T>::default())
    }

    fn cancel_timer<T: Component>(&mut self) -> &mut Self {
        self.queue(CancelTimer::<T>::default())
    }
}

/// An [`EntityCommand`] that starts a [`Timer`] on the target entity. The
/// [`Component`] `T` is used as a tag to identify the timer.
///
/// Use [`EntityCommands::start_timer`] to queue this command.
pub struct StartTimer<T: Component>(Timer, PhantomData<T>);

impl<T: Component> StartTimer<T> {
    /// Creates a new entity command.
    pub fn new(timer: Timer) -> Self {
        Self(timer, PhantomData)
    }
}

impl<T: Component> EntityCommand for StartTimer<T> {
    fn apply(self, entity: Entity, world: &mut World) {
        let component = world.register_component::<T>();

        let Ok(mut emut) = world.get_entity_mut(entity) else {
            return;
        };
        let mut timers = emut.entry::<Timers>().or_default();
        timers.insert(component, self.0);
    }
}

/// An [`EntityCommand`] that resets a [`Timer`] on the target entity. The
/// [`Component`] `T` is used as a tag to identify the timer.
///
/// Use [`EntityCommands::reset_timer`] to queue this command.
pub struct ResetTimer<T: Component>(PhantomData<T>);

impl<T: Component> Default for ResetTimer<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: Component> EntityCommand for ResetTimer<T> {
    fn apply(self, entity: Entity, world: &mut World) {
        let component = world.register_component::<T>();

        let Ok(mut emut) = world.get_entity_mut(entity) else {
            return;
        };
        let Some(mut timers) = emut.get_mut::<Timers>() else {
            return;
        };
        let Some(timer) = timers.get_mut(component) else {
            return;
        };
        timer.reset();
    }
}

/// An [`EntityCommand`] that pauses a [`Timer`] on the target entity. The
/// [`Component`] `T` is used as a tag to identify the timer.
///
/// Use [`EntityCommands::pause_timer`] to queue this command.
pub struct PauseTimer<T: Component>(PhantomData<T>);

impl<T: Component> Default for PauseTimer<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: Component> EntityCommand for PauseTimer<T> {
    fn apply(self, entity: Entity, world: &mut World) {
        let component = world.register_component::<T>();

        let Ok(mut emut) = world.get_entity_mut(entity) else {
            return;
        };
        let Some(mut timers) = emut.get_mut::<Timers>() else {
            return;
        };
        let Some(timer) = timers.get_mut(component) else {
            return;
        };
        timer.pause();
    }
}

/// An [`EntityCommand`] that unpauses a [`Timer`] on the target entity. The
/// [`Component`] `T` is used as a tag to identify the timer.
///
/// Use [`EntityCommands::unpause_timer`] to queue this command.
pub struct UnpauseTimer<T: Component>(PhantomData<T>);

impl<T: Component> Default for UnpauseTimer<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: Component> EntityCommand for UnpauseTimer<T> {
    fn apply(self, entity: Entity, world: &mut World) {
        let component = world.register_component::<T>();

        let Ok(mut emut) = world.get_entity_mut(entity) else {
            return;
        };
        let Some(mut timers) = emut.get_mut::<Timers>() else {
            return;
        };
        let Some(timer) = timers.get_mut(component) else {
            return;
        };
        timer.unpause();
    }
}

/// An [`EntityCommand`] that cancels a [`Timer`] on the target entity. The
/// [`Component`] `T` is used as a tag to identify the timer.
///
/// Use [`EntityCommands::cancel_timer`] to queue this command.
pub struct CancelTimer<T: Component>(PhantomData<T>);

impl<T: Component> Default for CancelTimer<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: Component> EntityCommand for CancelTimer<T> {
    fn apply(self, entity: Entity, world: &mut World) {
        let component = world.register_component::<T>();

        let Ok(mut emut) = world.get_entity_mut(entity) else {
            return;
        };
        let Some(mut timers) = emut.get_mut::<Timers>() else {
            return;
        };
        if timers.remove(component).is_some() {
            world.trigger_targets(OnTimerCancelled, TargetBoth(entity, component));
        }
    }
}
