use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct BPressState {
    pub active: bool,
    pub just_entered: bool,
    pub just_exited: bool,
}

#[derive(Component, Debug, Default)]
pub struct BHoverState {
    pub active: bool,
    pub just_entered: bool,
    pub just_exited: bool,
}

#[derive(Component, Debug, Default)]
pub struct BMouseOverState {
    pub active: bool,
    pub just_entered: bool,
    pub just_exited: bool,
}