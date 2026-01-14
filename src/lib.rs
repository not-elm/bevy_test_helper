use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::event::*;
use crate::resource::BevyTestHelperResourcePlugin;

pub mod error;
pub mod event;
pub mod resource;
pub mod share;
pub mod system;
mod trigger;

pub mod prelude {
    pub use crate::trigger::*;
}

pub struct BevyTestHelperPlugin;

impl PluginGroup for BevyTestHelperPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BevyTestHelperEventsPlugin)
            .add(BevyTestHelperResourcePlugin)
            .build()
    }
}
