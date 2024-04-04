use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::event::*;
use crate::resource::BevyTestHelperResourcePlugin;

pub mod event;
pub mod share;
pub mod resource;


pub struct BevyTestHelperPlugin;

impl PluginGroup for BevyTestHelperPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BevyTestHelperEventsPlugin)
            .add(BevyTestHelperResourcePlugin)
            .build()
    }
}
