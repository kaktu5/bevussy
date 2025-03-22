mod plugins;
mod utils;

use crate::plugins::{level::LevelPlugin, player::PlayerPlugin};
use bevy::{prelude::*, window::PresentMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::Immediate,
                    ..default()
                }),
                ..default()
            }),
            RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        .add_plugins((LevelPlugin, PlayerPlugin))
        .add_plugins(
            #[cfg(debug_assertions)]
            (
                RapierDebugRenderPlugin::default(),
                WorldInspectorPlugin::new(),
            ),
            #[cfg(not(debug_assertions))]
            (),
        )
        .run();
}
