use std::ops::Neg;

use avian3d::prelude::*;
use bevy::{color::palettes::css::RED, prelude::*};
use game_action::{ActionState, GameAction};

use crate::{PLAYER_JUMP_SPEED, PLAYER_WALK_SPEED, components::Player};

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // TODO: move to world module/plugin
    // ground
    commands.spawn((
        Collider::cuboid(100.0, 1.0, 100.0),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Static,
    ));

    commands.spawn((
        Player,
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
    action_state: Res<ActionState<GameAction>>,
    mut player_query: Query<&mut LinearVelocity, With<Player>>,
) {
    let Ok(mut player) = player_query.single_mut() else {
        return;
    };

    if action_state.just_pressed(&GameAction::Jump) {
        player.y = PLAYER_JUMP_SPEED;
    }
    if action_state.pressed(&GameAction::RunForward) {
        player.z = PLAYER_WALK_SPEED.neg();
    }
    if action_state.pressed(&GameAction::RunBackwards) {
        player.z = PLAYER_WALK_SPEED;
    }
    if action_state.pressed(&GameAction::RunLeft) {
        player.x = PLAYER_WALK_SPEED.neg();
    }
    if action_state.pressed(&GameAction::RunRight) {
        player.x = PLAYER_WALK_SPEED;
    }
}
