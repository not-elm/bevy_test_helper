use bevy::app::App;
use bevy::prelude::Event;

pub trait TriggerExtension {
    fn trigger<'a, E: Event<Trigger<'a>: Default>>(&mut self, event: E);
}

impl TriggerExtension for App {
    fn trigger<'a, E: Event<Trigger<'a>: Default>>(&mut self, event: E) {
        self.world_mut().trigger(event);
    }
}
