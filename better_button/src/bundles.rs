use bevy::prelude::*;
use crate::components::{BHoverState, BMouseOverState, BPressState};

#[derive(Bundle, Debug)]
pub struct BButtonBundle {
    pub button_bundle: ButtonBundle,
    pub press_state: BPressState,
    pub hover_state: BHoverState,
    pub mouse_over_state: BMouseOverState,
}

impl Default for BButtonBundle {
    fn default() -> Self {
        Self {
            button_bundle: Default::default(),
            press_state: Default::default(),
            hover_state: Default::default(),
            mouse_over_state: Default::default(),
        }
    }
}