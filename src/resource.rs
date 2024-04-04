use std::fmt::Debug;

use bevy::app::{App, Plugin};
use bevy::prelude::{Mut, Resource};

use crate::resource::bool::Bool;

pub mod bool;

pub trait DirectResourceControl {
    fn init_resource<R: Resource + Default>(&mut self);

    fn insert_resource<R: Resource>(&mut self, r: R);

    fn remove_resource<R: Resource>(&mut self) -> Option<R>;

    fn resource<R: Resource>(&self) -> &R;

    fn resource_mut<R: Resource>(&mut self) -> Mut<R>;

    #[inline]
    fn assert_resource_eq<R: Resource + Debug + PartialEq>(&self, expect: R) {
        assert_eq!(self.resource::<R>(), &expect);
    }

    #[inline]
    fn assert_resource<R: Resource, T: PartialEq + Debug>(&self, expect: T, f: impl FnOnce(&R) -> T) {
        assert_eq!(f(self.resource::<R>()), expect);
    }
}

impl DirectResourceControl for App {
    #[inline]
    fn init_resource<R: Resource + Default>(&mut self) {
        self.world.init_resource::<R>();
    }

    #[inline]
    fn insert_resource<R: Resource>(&mut self, r: R) {
        self.world.insert_resource(r);
    }

    #[inline]
    fn remove_resource<R: Resource>(&mut self) -> Option<R> {
        self.world.remove_resource::<R>()
    }

    #[inline]
    fn resource<R: Resource>(&self) -> &R {
        self.world.resource::<R>()
    }

    #[inline]
    fn resource_mut<R: Resource>(&mut self) -> Mut<R> {
        self.world.resource_mut::<R>()
    }
}

pub struct BevyTestHelperResourcePlugin;

impl Plugin for BevyTestHelperResourcePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Bool>();
    }
}