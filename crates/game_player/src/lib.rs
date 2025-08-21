use bevy::prelude::*;

use crate::systems::{player_movement, spawn_player};

pub mod components;
mod systems;

pub const PLAYER_WALK_SPEED: f32 = 15.0;
pub const PLAYER_JUMP_SPEED: f32 = 20.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}
