//! A Bevy utility crate that provides an ergonomic way to manage timers on
//! entities, powered by ECS observers.
//!
//! # Example
//!
//! ```
//! use bevy_ecs::prelude::*;
//! use bevy_time::prelude::*;
//! use bevy_observed_timers::prelude::*;
//!
//! // This component is used to identify the timer that will trigger the
//! // regeneration of the entity's health.
//! #[derive(Component)]
//! struct Regenerate;
//!
//! #[derive(Component)]
//! struct Health(f32);
//!
//! # let mut world = World::new();
//! # world.init_resource::<Time>();
//! // When the timer finishes, the `OnTimerFinished` event will be triggered
//! // with the component the timer is associated with.
//! world.add_observer(|
//!     t: Trigger<OnTimerFinished, Regenerate>,
//!     mut health: Query<&mut Health>,
//!     mut commands: Commands
//! | {
//!     let mut health = health.get_mut(t.entity()).unwrap();
//!     health.0 = (health.0 + 20.).min(100.);
//!     if health.0 >= 100. {
//!         // We can also cancel the timer manually when we're done with it.
//!         // One-shot timers are automatically cleaned up when they finish,
//!         // but repeating ones need to be manually cancelled.
//!         commands.entity(t.entity()).cancel_timer::<Regenerate>();
//!     }
//! });
//!
//! # let mut commands = world.commands();
//! let player = commands.spawn(Health(0.)).id();
//!
//! // Start a timer tagged with the `Regenerate` component on the player entity.
//! commands.entity(player)
//!     .start_timer::<Regenerate>(Timer::from_seconds(1., TimerMode::Repeating));
//! # world.flush();
//! # world.run_system_cached(tick_entity_timers).unwrap();
//! # assert_eq!(world.get::<Health>(player).unwrap().0, 0.);
//! # world.resource_mut::<Time>().advance_by(std::time::Duration::from_secs(1));
//! # world.run_system_cached(tick_entity_timers).unwrap();
//! # assert_eq!(world.get::<Health>(player).unwrap().0, 20.);
//! # world.resource_mut::<Time>().advance_by(std::time::Duration::from_secs(1));
//! # world.run_system_cached(tick_entity_timers).unwrap();
//! # assert_eq!(world.get::<Health>(player).unwrap().0, 40.);
//! # world.resource_mut::<Time>().advance_by(std::time::Duration::from_secs(1));
//! # world.run_system_cached(tick_entity_timers).unwrap();
//! # assert_eq!(world.get::<Health>(player).unwrap().0, 60.);
//! # world.resource_mut::<Time>().advance_by(std::time::Duration::from_secs(1));
//! # world.run_system_cached(tick_entity_timers).unwrap();
//! # assert_eq!(world.get::<Health>(player).unwrap().0, 80.);
//! # world.resource_mut::<Time>().advance_by(std::time::Duration::from_secs(1));
//! # world.run_system_cached(tick_entity_timers).unwrap();
//! # assert_eq!(world.get::<Health>(player).unwrap().0, 100.);
//! ```

#![warn(missing_docs)]

use bevy_ecs::{component::ComponentId, entity::Entity, observer::TriggerTargets};

pub mod command;
pub mod core;
pub mod event;
#[cfg(feature = "bevy_app")]
pub mod plugin;

pub mod prelude {
    //! Re-exports the most commonly used types and traits.

    pub use crate::command::EntityCommandTimersExt as _;
    pub use crate::core::{tick_entity_timers, Timers};
    pub use crate::event::{OnTimerCancelled, OnTimerFinished};
    #[cfg(feature = "bevy_app")]
    pub use crate::plugin::ScheduleTimerTickPlugin;
}

struct TargetBoth(Entity, ComponentId);

impl TriggerTargets for TargetBoth {
    fn components(&self) -> &[ComponentId] {
        std::array::from_ref(&self.1)
    }

    fn entities(&self) -> &[Entity] {
        std::array::from_ref(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy_ecs::{
        component::Component,
        observer::Trigger,
        system::{ResMut, Resource},
        world::World,
    };
    use bevy_time::{Time, Timer};

    use crate::{
        command::EntityCommandTimersExt, core::tick_entity_timers, event::OnTimerFinished,
    };

    #[derive(Component)]
    struct Foo;

    #[derive(Resource, Default)]
    struct Finished(bool);

    #[test]
    fn once() {
        let mut world = World::new();
        world.init_resource::<Time>();
        world.init_resource::<Finished>();
        world.add_observer(
            |_: Trigger<OnTimerFinished, Foo>, mut finished: ResMut<Finished>| {
                finished.0 = true;
            },
        );

        let e1 = world.spawn_empty().id();

        world
            .commands()
            .entity(e1)
            .start_timer::<Foo>(Timer::from_seconds(5., bevy_time::TimerMode::Once));
        world.flush();
        assert!(!world.get_resource::<Finished>().unwrap().0);

        world.run_system_cached(tick_entity_timers).unwrap();
        assert!(!world.get_resource::<Finished>().unwrap().0);

        world
            .resource_mut::<Time>()
            .advance_by(Duration::from_secs(3));
        world.run_system_cached(tick_entity_timers).unwrap();
        assert!(!world.get_resource::<Finished>().unwrap().0);

        world
            .resource_mut::<Time>()
            .advance_by(Duration::from_secs(3));
        world.run_system_cached(tick_entity_timers).unwrap();
        assert!(world.get_resource::<Finished>().unwrap().0);
    }
}
