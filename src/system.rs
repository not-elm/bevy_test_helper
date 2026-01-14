use bevy::app::App;
use bevy::ecs::system::RunSystemOnce;
use bevy::prelude::IntoSystem;

pub trait SystemExt {
    fn run_system_once<T, Out, Marker>(&mut self, system: T) -> Out
    where
        T: IntoSystem<(), Out, Marker>;
}

impl SystemExt for App {
    fn run_system_once<T, Out, Marker>(&mut self, system: T) -> Out
    where
        T: IntoSystem<(), Out, Marker>,
    {
        self.world_mut().run_system_once(system).unwrap()
    }
}
