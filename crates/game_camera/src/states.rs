use bevy::prelude::*;

#[derive(States, Clone, Default, Eq, PartialEq, Hash, Debug)]
pub enum PlayerCameraMode {
    #[default]
    FirstPerson,
    ThirdPerson,
}
