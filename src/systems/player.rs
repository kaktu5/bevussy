use crate::error_and_exit;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Settings>()
            .add_systems(Startup, setup)
            .add_systems(Update, movement);
    }
}

#[derive(Component)]
struct Player;

#[derive(Resource, Deserialize, Serialize)]
struct KeyBinds {
    forward: KeyCode,
    backward: KeyCode,
    left: KeyCode,
    right: KeyCode,
    jump: KeyCode,
}

impl Default for KeyBinds {
    fn default() -> Self {
        Self {
            forward: KeyCode::KeyW,
            backward: KeyCode::KeyS,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            jump: KeyCode::Space,
        }
    }
}

#[derive(Resource, Deserialize, Serialize)]
struct Settings {
    initial_position: Vec3,
    movement_speed: f32,
    movement_acceleration: f32,
    sensitivity: f32,
    key_binds: KeyBinds,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            initial_position: Vec3::new(0., 8., 0.),
            movement_speed: 32.,
            movement_acceleration: 8.,
            sensitivity: 0.00005,
            key_binds: KeyBinds::default(),
        }
    }
}

fn setup(mut commands: Commands, settings: Res<Settings>) {
    commands
        .spawn((
            RigidBody::KinematicPositionBased,
            Camera3d::default(),
            Player,
        ))
        .insert(Collider::ball(0.5))
        .insert(Transform::from_translation(settings.initial_position))
        .insert(KinematicCharacterController {
            autostep: Some(CharacterAutostep {
                include_dynamic_bodies: true,
                max_height: CharacterLength::Relative(0.25),
                min_width: CharacterLength::Relative(0.25),
            }),
            max_slope_climb_angle: 45_f32.to_radians(),
            min_slope_slide_angle: 30_f32.to_radians(),
            ..default()
        });
}

fn movement(
    keys: Res<ButtonInput<KeyCode>>,
    settings: Res<Settings>,
    time: Res<Time>,
    mut players: Query<&mut Transform, With<Player>>,
) {
    let key_binds = &settings.key_binds;
    let mut transform = players
        .iter_mut()
        .next()
        .unwrap_or_else(|| error_and_exit!("Failed to get `player`"));

    let mut input = Vec2::ZERO;
    keys.get_pressed().for_each(|key| match *key {
        key if key == key_binds.forward => input.x += 1.,
        key if key == key_binds.backward => input.x -= 1.,
        key if key == key_binds.right => input.y += 1.,
        key if key == key_binds.left => input.y -= 1.,
        _ => {}
    });

    let velocity = {
        let local_z = transform.local_z();
        let forward = -Vec2::new(local_z.x, local_z.z);
        let right = Vec2::new(local_z.z, -local_z.x);
        (forward * input.x + right * input.y).normalize_or_zero()
            * settings.movement_speed
            * time.delta_secs()
    };

    transform.translation += Vec3::new(velocity.x, 0., velocity.y);
}
