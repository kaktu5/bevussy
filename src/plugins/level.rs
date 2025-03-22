use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb_u8(122, 133, 255)))
            .add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::new(0., 1., 0.), Vec2::new(32., 32.)))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Collider::cuboid(32., 0., 32.),
        RigidBody::Fixed,
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(4., 4., 4.))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0., 2., -8.),
        Collider::cuboid(2., 2., 2.),
        RigidBody::Fixed,
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(100., 200., 100.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
