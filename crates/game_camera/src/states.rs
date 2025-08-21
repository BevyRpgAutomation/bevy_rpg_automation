use bevy::prelude::*;

#[derive(States, Clone, Default, Eq, PartialEq, Hash, Debug)]
pub enum PlayerCameraMode {
    #[default]
    FirstPerson,
    ThirdPerson,
}

impl std::fmt::Display for PlayerCameraMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FirstPerson => write!(f, "First Person"),
            Self::ThirdPerson => write!(f, "Third Person"),
        }
    }
}
