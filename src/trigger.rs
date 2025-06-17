use bevy::app::App;
use bevy::prelude::{Entity, Event};

pub trait TriggerExtension{
    fn trigger<E: Event>(&mut self, event: E);
    
    fn trigger_with_target<E: Event>(&mut self, event: E, target: Entity);
}

impl TriggerExtension for App {
    fn trigger<E: Event>(&mut self, event: E) {
        self.world_mut().trigger(event);
    }
    
    fn trigger_with_target<E: Event>(&mut self, event: E, target: Entity) {
        self.world_mut().trigger_targets(event, target);
    }
}

