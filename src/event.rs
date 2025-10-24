use bevy::app::{App, Plugin};
use bevy::ecs::message::{MessageCursor, MessageId, MessageIterator};
use bevy::prelude::{Event, Message, Messages, World};
use bevy_test_helper_macro_impl::delegate_app;

#[delegate_app]
pub trait DirectEvents {
    fn write<E: Message>(&mut self, event: E) -> MessageId<E>;

    fn write_default<E: Message + Default>(&mut self) -> MessageId<E>;

    fn read_messages<'a, E: Message>(
        &'a self,
        reader: &'a mut MessageCursor<E>,
    ) -> MessageIterator<'a, E>;

    fn read_last_message<'a, E: Message>(
        &'a self,
        reader: &'a mut MessageCursor<E>,
    ) -> Option<&'a E> {
        self.read_messages(reader).last()
    }

    fn assert_message_comes<'a, E: Message>(&'a self, reader: &'a mut MessageCursor<E>) {
        assert!(self.read_last_message(reader).is_some());
    }

    fn assert_message_not_comes<'a, E: Message>(&'a self, reader: &'a mut MessageCursor<E>) {
        assert!(self.read_last_message(reader).is_none());
    }
}

impl DirectEvents for World {
    fn write<E: Message>(&mut self, event: E) -> MessageId<E> {
        self.resource_mut::<Messages<E>>().write(event)
    }

    fn write_default<E: Message + Default>(&mut self) -> MessageId<E> {
        self.resource_mut::<Messages<E>>().write_default()
    }

    fn read_messages<'a, E: Message>(
        &'a self,
        reader: &'a mut MessageCursor<E>,
    ) -> MessageIterator<'a, E> {
        reader.read(self.resource::<Messages<E>>())
    }
}

pub struct BevyTestHelperEventsPlugin;

impl Plugin for BevyTestHelperEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<TestEvent1>()
            .add_message::<TestEvent2>()
            .add_message::<TestEvent3>()
            .add_message::<TestEvent4>()
            .add_message::<TestEvent5>()
            .add_message::<TestEvent6>()
            .add_message::<TestEvent7>()
            .add_message::<TestEvent8>()
            .add_message::<TestEvent9>()
            .add_message::<TestEvent10>()
            .add_message::<TestEvent11>()
            .add_message::<TestEvent12>();
    }
}

macro_rules! test_event {
    ($name: ident) => {
        #[derive(Default, Eq, PartialEq, Copy, Clone, Message, Hash, Debug, Event)]
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
