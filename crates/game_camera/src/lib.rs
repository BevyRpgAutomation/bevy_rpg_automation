use std::{f32::consts::FRAC_PI_2, ops::Range};

use bevy::prelude::*;

use crate::{
    states::PlayerCameraMode,
    systems::{
        camera_follow_player, mouse_rotate_camera, spawn_camera,
        switch_between_first_and_third_person,
    },
};

pub mod states;
mod systems;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum PlayerCameraAction {
    SwitchBetweenFirstAndThirdPerson,
}

#[derive(Debug, Resource)]
struct CameraSettings {
    pub orbit_distance: f32,
    pub pitch_speed: f32,
    // Clamp pitch to this range
    pub pitch_range: Range<f32>,
    pub roll_speed: f32,
    pub yaw_speed: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        // Limiting pitch stops some unexpected rotation past 90° up or down.
        let pitch_limit = FRAC_PI_2 - 0.01;
        Self {
            orbit_distance: 20.0,
            pitch_speed: 0.003,
            pitch_range: -pitch_limit..pitch_limit,
            roll_speed: 1.0,
            yaw_speed: 0.004,
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerCameraMode>()
            .insert_resource(CameraSettings::default())
            .add_systems(Startup, spawn_camera)
            .add_systems(
                Update,
                (
                    camera_follow_player,
                    switch_between_first_and_third_person,
                    mouse_rotate_camera,
                ),
            );
    }
}
