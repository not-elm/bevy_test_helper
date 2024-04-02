use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::event::*;

pub mod event;


pub struct BevyTestHelperPlugin;

impl PluginGroup for BevyTestHelperPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BevyTestHelperEventsPlugin)
            .build()
    }
}
