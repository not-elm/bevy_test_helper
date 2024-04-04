use std::sync::{Arc, Mutex, MutexGuard};
use bevy::prelude::Resource;


pub fn create_shares<T: Default>() -> (Share<T>, Share<T>){
    let share = Share::<T>::default();
    (share.clone(), share)
}


#[derive(Debug, Default, Resource)]
pub struct Share<T>(Arc<Mutex<T>>);

impl<T> Clone for Share<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T> Share<T> {
    #[inline]
    pub fn lock(&self) -> MutexGuard<'_, T> {
        self.0.lock().unwrap()
    }

    #[inline]
    pub fn set(&self, t: T){
        *self.0.lock().unwrap() = t;
    }
}




