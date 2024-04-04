use bevy::prelude::{Deref, DerefMut, Resource, World};
use bevy_test_helper_macro_impl::delegate_app;

#[derive(Default, Debug, Resource, Copy, Clone, Deref, DerefMut)]
pub struct Bool(bool);


#[delegate_app]
pub trait BoolExtension{
    fn is_bool_false(&self) -> bool;

    fn is_bool_true(&self) -> bool;

    fn set_bool(&mut self, v: bool);
}


impl BoolExtension for World{
    #[inline]
    fn is_bool_false(&self) -> bool {
        !self.is_bool_true()
    }

    #[inline]
    fn is_bool_true(&self) -> bool {
        self.resource::<Bool>().0
    }

    #[inline]
    fn set_bool(&mut self, v: bool) {
        self.resource_mut::<Bool>().0 = v;
    }
}
