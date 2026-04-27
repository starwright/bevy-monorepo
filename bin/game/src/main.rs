use bevy::{
    prelude::*,
    window::{Window, WindowPlugin},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy-monorepo".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .run();
}
