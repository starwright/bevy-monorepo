use bevy::app::{PluginGroup, PluginGroupBuilder};

pub struct CameraPluginGroup;

impl PluginGroup for CameraPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
    }
}