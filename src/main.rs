#![feature(stmt_expr_attributes)]

mod systems;
mod utils;

#[allow(unused_imports)]
use crate::systems::{flycam::FlyCamPlugin, player::PlayerPlugin};
use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                present_mode: PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }))
        // .add_plugins(FlyCamPlugin)
        .add_plugins(PlayerPlugin)
        .insert_resource(ClearColor(Color::srgb_u8(122, 133, 255)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(32.))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}
