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
    player_query: Query<&Transform, (With<Player>, Without<Camera3d>)>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    player_camera_mode: Res<State<PlayerCameraMode>>,
) {
    let Ok(player) = player_query.single() else {
        return;
    };
    let Ok(mut camera) = camera_query.single_mut() else {
        return;
    };

    match player_camera_mode.get() {
        PlayerCameraMode::FirstPerson => {
            camera.translation = player.translation;
            camera.translation.z -= 10.0;
        }
        PlayerCameraMode::ThirdPerson => {
            camera.translation = player.translation;
            camera.translation.z += 50.0;
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
