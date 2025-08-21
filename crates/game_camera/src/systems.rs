use bevy::prelude::*;
use game_player::components::Player;
use leafwing_input_manager::prelude::*;

use crate::{PlayerCameraAction, states::PlayerCameraMode};

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
    window_query: Query<&Window>,
    player_query: Query<&Transform, (With<Player>, Without<Camera3d>)>,
    mut camera_query: Query<
        (&Camera, &mut Transform, &GlobalTransform),
        (With<Camera3d>, Without<Player>),
    >,
    player_camera_mode: Res<State<PlayerCameraMode>>,
) {
    let Ok(player) = player_query.single() else {
        return;
    };
    let Ok(camera) = camera_query.single_mut() else {
        return;
    };
    let (camera, mut camera_transform, camera_global_transform) = camera;

    let window = window_query.single().expect("Exactly one window exists");
    let Some(cursor_position_in_window) = window.cursor_position() else {
        return;
    };

    camera_transform.translation = player.translation;

    match camera.viewport_to_world(&camera_global_transform, cursor_position_in_window) {
        Ok(value) => {
            *camera_transform = camera_transform.looking_at(value.direction.into(), Vec3::Y);
            println!("value: {:?}", value);
        }
        Err(error) => {
            eprintln!("Failed to convert viewport to world: {}", error);
        }
    }

    match player_camera_mode.get() {
        PlayerCameraMode::FirstPerson => {
            camera_transform.translation.z -= 10.0;
        }
        PlayerCameraMode::ThirdPerson => {
            camera_transform.translation.z += 50.0;
        }
    }
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
