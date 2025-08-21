use bevy::prelude::*;

use crate::systems::{spawn_debug_hud, update_current_player_camera_mode_text, update_fps_text};

mod components;
mod systems;

pub struct DebugHudPlugin;

impl Plugin for DebugHudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_debug_hud).add_systems(
            Update,
            (update_current_player_camera_mode_text, update_fps_text),
        );
    }
}
