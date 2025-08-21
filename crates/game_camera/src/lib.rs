use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    states::PlayerCameraMode,
    systems::{camera_follow_player, spawn_camera, switch_between_first_and_third_person},
};

pub mod states;
mod systems;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum PlayerCameraAction {
    SwitchBetweenFirstAndThirdPerson,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerCameraMode>()
            .add_plugins(InputManagerPlugin::<PlayerCameraAction>::default())
            .add_systems(Startup, spawn_camera)
            .add_systems(
                Update,
                (camera_follow_player, switch_between_first_and_third_person),
            );
    }
}
