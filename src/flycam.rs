use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

pub struct FlyCamPlugin;

impl Plugin for FlyCamPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Settings>()
            .add_systems(Startup, (setup_camera, initial_grab_cursor))
            .add_systems(Update, (move_camera, look_camera, grab_cursor));
    }
}

#[derive(Component)]
pub struct FlyCam;

#[derive(Resource)]
pub struct KeyBinds {
    pub forward: KeyCode,
    pub backward: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub ascend: KeyCode,
    pub descend: KeyCode,
    pub toggle_grab_cursor: KeyCode,
}

impl Default for KeyBinds {
    fn default() -> Self {
        Self {
            forward: KeyCode::KeyW,
            backward: KeyCode::KeyS,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            ascend: KeyCode::ShiftLeft,
            descend: KeyCode::ControlLeft,
            toggle_grab_cursor: KeyCode::Escape,
        }
    }
}

#[derive(Resource)]
pub struct Settings {
    pub sensitivity: f32,
    pub speed: f32,
    pub initial_position: Vec3,
    pub key_binds: KeyBinds,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00005,
            speed: 12.,
            initial_position: Vec3::new(0., 8., 0.),
            key_binds: KeyBinds::default(),
        }
    }
}

fn setup_camera(mut commands: Commands, settings: Res<Settings>) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(settings.initial_position),
        FlyCam,
    ));
}

fn move_camera(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    settings: Res<Settings>,
    mut query: Query<(&FlyCam, &mut Transform)>,
) {
    let window = primary_window.get_single().unwrap_or_else(|err| {
        error!("Failed to get primary window: ${:?}", err);
        panic!();
    });
    let key_binds = &settings.key_binds;
    for (_, mut transform) in query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);
        for key in keys.get_pressed() {
            match window.cursor_options.grab_mode {
                CursorGrabMode::None => (),
                _ => {
                    let key = *key;
                    if key == key_binds.forward {
                        velocity += forward;
                    } else if key == key_binds.backward {
                        velocity -= forward;
                    } else if key == key_binds.left {
                        velocity -= right;
                    } else if key == key_binds.right {
                        velocity += right;
                    } else if key == key_binds.ascend {
                        velocity += Vec3::Y;
                    } else if key == key_binds.descend {
                        velocity -= Vec3::Y;
                    }
                }
            }
        }
        velocity = velocity.normalize_or_zero();
        transform.translation += velocity * settings.speed * time.delta_secs()
    }
}

fn look_camera(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    settings: Res<Settings>,
    mut state: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    let window = primary_window.get_single().unwrap_or_else(|err| {
        error!("Failed to get primary window: ${:?}", err);
        panic!();
    });
    for mut transform in query.iter_mut() {
        for ev in state.read() {
            let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
            match window.cursor_options.grab_mode {
                CursorGrabMode::None => (),
                _ => {
                    let window_scale = window.height().min(window.width());
                    pitch -= (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                    yaw -= (settings.sensitivity * ev.delta.x * window_scale).to_radians();
                }
            }
            pitch = pitch.clamp(-1.54, 1.54);
            transform.rotation =
                Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        }
    }
}

fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor_options.grab_mode {
        CursorGrabMode::None => {
            window.cursor_options = CursorOptions {
                grab_mode: CursorGrabMode::Confined,
                visible: false,
                ..window.cursor_options
            }
        }
        _ => {
            window.cursor_options = CursorOptions {
                grab_mode: CursorGrabMode::None,
                visible: true,
                ..window.cursor_options
            }
        }
    }
}

fn grab_cursor(
    keys: Res<ButtonInput<KeyCode>>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    settings: Res<Settings>,
) {
    let mut window = primary_window.get_single_mut().unwrap_or_else(|err| {
        error!("Failed to get primary window: ${:?}", err);
        panic!();
    });
    if keys.just_pressed(settings.key_binds.toggle_grab_cursor) {
        toggle_grab_cursor(&mut window);
    }
}

fn initial_grab_cursor(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = primary_window.get_single_mut().unwrap_or_else(|err| {
        error!("Failed to get primary window: ${:?}", err);
        panic!();
    });
    toggle_grab_cursor(&mut window);
}
