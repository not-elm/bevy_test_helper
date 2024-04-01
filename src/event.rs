use bevy::prelude::Event;

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

