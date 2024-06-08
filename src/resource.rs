use std::fmt::Debug;

use bevy::app::{App, Plugin};
use bevy::prelude::{Mut, Resource};

use crate::resource::bool::Bool;
use crate::resource::count::Count;

pub mod bool;
pub mod count;

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
        self.world_mut().init_resource::<R>();
    }

    #[inline]
    fn insert_resource<R: Resource>(&mut self, r: R) {
        self.world_mut().insert_resource(r);
    }

    #[inline]
    fn remove_resource<R: Resource>(&mut self) -> Option<R> {
        self.world_mut().remove_resource::<R>()
    }

    #[inline]
    fn resource<R: Resource>(&self) -> &R {
        self.world().resource::<R>()
    }

    #[inline]
    fn resource_mut<R: Resource>(&mut self) -> Mut<R> {
        self.world_mut().resource_mut::<R>()
    }
}

pub struct BevyTestHelperResourcePlugin;

impl Plugin for BevyTestHelperResourcePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Bool>()
            .init_resource::<Count>();
    }
}