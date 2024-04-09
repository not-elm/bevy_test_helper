use bevy::prelude::{Deref, DerefMut, Resource};

#[derive(Default, Debug, Copy, Clone, DerefMut, Deref, Resource, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Count(usize);

impl Count{
    pub fn increment(&mut self) -> usize{
        self.0 += 1;
        self.0
    }

    pub fn decrement(&mut self) -> usize{
        self.0 -= 1;
        self.0
    }

    pub fn set(&mut self, v: usize){
        self.0 = v;
    }
}