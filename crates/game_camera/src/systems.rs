use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};
use game_player::components::Player;
use leafwing_input_manager::prelude::*;

use crate::{CameraSettings, PlayerCameraAction, states::PlayerCameraMode};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        InputMap::new([(
            PlayerCameraAction::SwitchBetweenFirstAndThirdPerson,
            KeyCode::KeyV,
        )]),
    ));
}

pub fn camera_follow_player(
    player_query: Query<&Transform, (With<Player>, Without<Camera3d>)>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    player_camera_mode: Res<State<PlayerCameraMode>>,
) {
    let Ok(player) = player_query.single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };

    camera_transform.translation = player.translation;

    match player_camera_mode.get() {
        PlayerCameraMode::FirstPerson => {
            camera_transform.translation.z -= 10.0;
        }
        PlayerCameraMode::ThirdPerson => {
            camera_transform.translation.z += 50.0;
        }
    }
}

pub fn mouse_rotate_camera(
    mut camera: Single<&mut Transform, (With<Camera>, Without<Player>)>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
    camera_settings: Res<CameraSettings>,
    player_transform: Single<&Transform, (With<Player>, Without<Camera>)>,
) {
    let delta = mouse_motion.delta;
    let mut delta_roll = 0.0;

    // Mouse motion is one of the few inputs that should not be multiplied by delta time,
    // as we are already receiving the full movement since the last frame was rendered. Multiplying
    // by delta time here would make the movement slower that it should be.
    let delta_pitch = delta.y * camera_settings.pitch_speed;
    let delta_yaw = delta.x * camera_settings.yaw_speed;

    // Conversely, we DO need to factor in delta time for mouse button inputs.
    delta_roll *= camera_settings.roll_speed * time.delta_secs();

    // Obtain the existing pitch, yaw, and roll values from the transform.
    let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);

    // Establish the new yaw and pitch, preventing the pitch value from exceeding our limits.
    let pitch = (pitch + delta_pitch).clamp(
        camera_settings.pitch_range.start,
        camera_settings.pitch_range.end,
    );
    let roll = roll + delta_roll;
    let yaw = yaw + delta_yaw;
    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

    let target = player_transform.translation;
    camera.translation = target - camera.forward() * camera_settings.orbit_distance;
}

pub fn switch_between_first_and_third_person(
    action_query: Query<&ActionState<PlayerCameraAction>, With<Camera>>,
    current_player_camera_mode: Res<State<PlayerCameraMode>>,
    mut next_player_camera_mode: ResMut<NextState<PlayerCameraMode>>,
) {
    let Ok(action) = action_query.single() else {
        return;
    };
    if !action.just_pressed(&PlayerCameraAction::SwitchBetweenFirstAndThirdPerson) {
        return;
    }

    match current_player_camera_mode.get() {
        PlayerCameraMode::FirstPerson => {
            next_player_camera_mode.set(PlayerCameraMode::ThirdPerson);
        }
        PlayerCameraMode::ThirdPerson => {
            next_player_camera_mode.set(PlayerCameraMode::FirstPerson);
        }
    }
}
