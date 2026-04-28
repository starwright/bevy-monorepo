use bevy::app::{PluginGroup, PluginGroupBuilder};

pub struct ConfPluginGroup;

impl PluginGroup for ConfPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
    }
}