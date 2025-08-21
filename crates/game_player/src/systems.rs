use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{PlayerAction, components::Player};

pub fn spawn_player(mut commands: Commands) {
    let input_map = InputMap::new([(PlayerAction::Jump, KeyCode::Space)]);

    commands.spawn((Player, input_map));
}

pub fn player_movement(query: Query<&ActionState<PlayerAction>, With<Player>>) {
    let Ok(action_state) = query.single() else {
        return;
    };

    if action_state.just_pressed(&PlayerAction::Jump) {
        println!("jumping!");
    }
}
