use bevy::prelude::*;

use crate::{
    states::PlayerCameraMode,
    systems::{camera_follow_player, spawn_camera, switch_between_first_and_third_person},
};

pub mod states;
mod systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerCameraMode>()
            .add_systems(Startup, spawn_camera)
            .add_systems(
                Update,
                (camera_follow_player, switch_between_first_and_third_person),
            );
    }
}
