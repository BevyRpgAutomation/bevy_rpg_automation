use std::ops::Neg;

use avian3d::prelude::*;
use bevy::{color::palettes::css::RED, prelude::*};
use leafwing_input_manager::prelude::*;

use crate::{PLAYER_JUMP_SPEED, PLAYER_WALK_SPEED, PlayerAction, components::Player};

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let input_map = InputMap::new([
        (PlayerAction::Jump, KeyCode::Space),
        (PlayerAction::RunForward, KeyCode::KeyW),
        (PlayerAction::RunLeft, KeyCode::KeyA),
        (PlayerAction::RunRight, KeyCode::KeyD),
        (PlayerAction::RunBackwards, KeyCode::KeyS),
    ]);

    // TODO: move to world module/plugin
    // ground
    commands.spawn((
        Collider::cuboid(100.0, 1.0, 100.0),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Static,
    ));

    commands.spawn((
        Player,
        input_map,
        Collider::capsule(2.0, 2.0),
        RigidBody::Dynamic,
        LinearVelocity::ZERO,
        Mesh3d(meshes.add(Capsule3d::new(2.0, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: RED.into(),
            ..Default::default()
        })),
        Transform::from_xyz(10.0, 10.0, 10.0),
        LockedAxes::ROTATION_LOCKED,
        Friction::new(2.0),
    ));
}

pub fn player_movement(
    action_query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut player_query: Query<&mut LinearVelocity, With<Player>>,
) {
    let Ok(action_state) = action_query.single() else {
        return;
    };

    let Ok(mut player) = player_query.single_mut() else {
        return;
    };

    if action_state.just_pressed(&PlayerAction::Jump) {
        player.y = PLAYER_JUMP_SPEED;
    }
    if action_state.pressed(&PlayerAction::RunForward) {
        player.z = PLAYER_WALK_SPEED.neg();
    }
    if action_state.pressed(&PlayerAction::RunBackwards) {
        player.z = PLAYER_WALK_SPEED;
    }
    if action_state.pressed(&PlayerAction::RunLeft) {
        player.x = PLAYER_WALK_SPEED.neg();
    }
    if action_state.pressed(&PlayerAction::RunRight) {
        player.x = PLAYER_WALK_SPEED;
    }
}
