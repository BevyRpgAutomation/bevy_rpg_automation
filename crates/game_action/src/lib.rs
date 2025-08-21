use bevy::prelude::*;
pub use leafwing_input_manager::prelude::ActionState;
use leafwing_input_manager::prelude::*;

pub struct GameActionPlugin;

impl Plugin for GameActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<GameAction>::default())
            .init_resource::<ActionState<GameAction>>()
            .insert_resource(GameAction::default_input_map());
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum GameAction {
    // Movement
    RunForward,
    RunLeft,
    RunRight,
    RunBackwards,
    Jump,

    // Camera
    SwitchBetweenFirstAndThirdPerson,
}

impl GameAction {
    fn default_input_map() -> InputMap<GameAction> {
        InputMap::new([
            (GameAction::Jump, KeyCode::Space),
            (GameAction::RunForward, KeyCode::KeyW),
            (GameAction::RunLeft, KeyCode::KeyA),
            (GameAction::RunRight, KeyCode::KeyD),
            (GameAction::RunBackwards, KeyCode::KeyS),
            (GameAction::SwitchBetweenFirstAndThirdPerson, KeyCode::KeyV),
        ])
    }
}
