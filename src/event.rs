use bevy::app::{App, Plugin};
use bevy::ecs::event::{EventId, EventIterator, ManualEventReader};
use bevy::prelude::{Event, Events, World};

use bevy_test_helper_macro_impl::delegate_app;

#[delegate_app]
pub trait DirectEvents {
    fn send<E: Event>(&mut self, event: E) -> EventId<E>;

    fn send_default<E: Event + Default>(&mut self) -> EventId<E>;

    fn read_events<'a, E: Event>(&'a self, reader: &'a mut ManualEventReader<E>) -> EventIterator<'a, E>;

    fn read_last_event<'a, E: Event>(&'a self, reader: &'a mut ManualEventReader<E>) -> Option<&E> {
        self.read_events(reader).last()
    }

    fn assert_event_comes<'a, E: Event>(&'a self, reader: &'a mut ManualEventReader<E>) {
        assert!(self.read_last_event(reader).is_some());
    }

    fn assert_event_not_comes<'a, E: Event>(&'a self, reader: &'a mut ManualEventReader<E>) {
        assert!(self.read_last_event(reader).is_none());
    }
}

impl DirectEvents for World {
    fn send<E: Event>(&mut self, event: E) -> EventId<E> {
        self.resource_mut::<Events<E>>().send(event)
    }

    fn send_default<E: Event + Default>(&mut self) -> EventId<E> {
        self.resource_mut::<Events<E>>().send_default()
    }

    fn read_events<'a, E: Event>(&'a self, reader: &'a mut ManualEventReader<E>) -> EventIterator<'a, E> {
        reader.read(self.resource::<Events<E>>())
    }
}

pub struct BevyTestHelperEventsPlugin;

impl Plugin for BevyTestHelperEventsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<TestEvent1>()
            .add_event::<TestEvent2>()
            .add_event::<TestEvent3>()
            .add_event::<TestEvent4>()
            .add_event::<TestEvent5>()
            .add_event::<TestEvent6>()
            .add_event::<TestEvent7>()
            .add_event::<TestEvent8>()
            .add_event::<TestEvent9>()
            .add_event::<TestEvent10>()
            .add_event::<TestEvent11>()
            .add_event::<TestEvent12>();
    }
}


macro_rules! test_event {
    ($name: ident) => {
        #[derive(Default, Eq, PartialEq, Copy, Clone, Event, Hash, Debug)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name;
    };
}

test_event!(TestEvent1);
test_event!(TestEvent2);
test_event!(TestEvent3);
test_event!(TestEvent4);
test_event!(TestEvent5);
test_event!(TestEvent6);
test_event!(TestEvent7);
test_event!(TestEvent8);
test_event!(TestEvent9);
test_event!(TestEvent10);
test_event!(TestEvent11);
test_event!(TestEvent12);

