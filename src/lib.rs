use bevy::app::{App, Plugin};

use crate::event::*;

pub mod event;


pub struct BevyTestHelperPlugin;

impl Plugin for BevyTestHelperPlugin {
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