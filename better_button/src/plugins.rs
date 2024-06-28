use bevy::prelude::*;
use crate::prelude::*;
use crate::systems::*;

pub struct BButtonPlugin;

impl Plugin for BButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BPressEvent>()
            .add_event::<BHoverEvent>()
            .add_event::<BMouseOverEvent>()
            .configure_sets(Update, BButtonUpdateSet)
            .add_systems(
                Update,
                (
                    (
                        update_hover_state,
                        update_press_state,
                        update_mouse_over_state,
                    ),
                    (
                        generate_hover_events,
                        generate_press_events,
                        generate_mouse_over_events,
                    )
                ).chain().in_set(BButtonUpdateSet),
            );
    }
}