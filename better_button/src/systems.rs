use bevy::prelude::*;
use crate::components::{BHoverState, BMouseOverState, BPressState};
use crate::events::{BHoverEvent, BMouseOverEvent, BPressEvent};


pub fn update_press_state(mut query: Query<(&Interaction, &mut BPressState)>) {
    for (interaction, mut press_state) in &mut query {
        // Handle active or inactive case
        if *interaction == Interaction::Pressed {
            when_active(&mut press_state);
        } else {
            when_inactive(&mut press_state);
        }

        // Aux function
        fn when_active(press_state: &mut Mut<BPressState>) {
            if !press_state.active {
                press_state.active = true;
                press_state.just_entered = true;
            } else {
                if press_state.just_entered {
                    press_state.just_entered = false;
                }
            }
            if press_state.just_exited {
                press_state.just_exited = false;
            }
        }

        // Aux function
        fn when_inactive(press_state: &mut Mut<BPressState>) {
            if press_state.active {
                press_state.active = false;
                press_state.just_exited = true;
            } else {
                if press_state.just_exited {
                    press_state.just_exited = false;
                }
            }
            if press_state.just_entered {
                press_state.just_entered = false;
            }
        }
    }
}

pub fn update_hover_state(mut query: Query<(&Interaction, &mut BHoverState)>) {
    for (interaction, mut hover_state) in &mut query {
        // Handle active or inactive case
        if *interaction == Interaction::Hovered {
            when_active(&mut hover_state);
        } else {
            when_inactive(&mut hover_state);
        }

        // Aux function
        fn when_active(hover_state: &mut Mut<BHoverState>) {
            if !hover_state.active {
                hover_state.active = true;
                hover_state.just_entered = true;
            } else {
                if hover_state.just_entered {
                    hover_state.just_entered = false;
                }
            }
            if hover_state.just_exited {
                hover_state.just_exited = false;
            }
        }

        // Aux function
        fn when_inactive(hover_state: &mut Mut<BHoverState>) {
            if hover_state.active {
                hover_state.active = false;
                hover_state.just_exited = true;
            } else {
                if hover_state.just_exited {
                    hover_state.just_exited = false;
                }
            }
            if hover_state.just_entered {
                hover_state.just_entered = false;
            }
        }
    }
}

pub fn update_mouse_over_state(mut query: Query<(&Interaction, &mut BMouseOverState)>) {
    for (interaction, mut mouse_over_state) in &mut query {
        // Handle active or inactive case
        if *interaction == Interaction::None {
            when_inactive(&mut mouse_over_state);
        } else {
            when_active(&mut mouse_over_state);
        }

        // Aux function
        fn when_active(mouse_over_state: &mut Mut<BMouseOverState>) {
            if !mouse_over_state.active {
                mouse_over_state.active = true;
                mouse_over_state.just_entered = true;
            } else {
                if mouse_over_state.just_entered {
                    mouse_over_state.just_entered = false;
                }
            }
            if mouse_over_state.just_exited {
                mouse_over_state.just_exited = false;
            }
        }

        // Aux function
        fn when_inactive(mouse_over_state: &mut Mut<BMouseOverState>) {
            if mouse_over_state.active {
                mouse_over_state.active = false;
                mouse_over_state.just_exited = true;
            } else {
                if mouse_over_state.just_exited {
                    mouse_over_state.just_exited = false;
                }
            }
            if mouse_over_state.just_entered {
                mouse_over_state.just_entered = false;
            }
        }
    }
}


pub fn generate_press_events(
    query: Query<(Entity, &BPressState)>,
    mut event_writer: EventWriter<BPressEvent>,
) {
    for (entity, press_state) in &query {
        if press_state.just_entered {
            event_writer.send(BPressEvent::JustEntered { entity });
        }
        if press_state.just_exited {
            event_writer.send(BPressEvent::JustExited { entity });
        }
    }
}

pub fn generate_hover_events(
    query: Query<(Entity, &BHoverState)>,
    mut event_writer: EventWriter<BHoverEvent>,
) {
    for (entity, hover_state) in &query {
        if hover_state.just_entered {
            event_writer.send(BHoverEvent::JustEntered { entity });
        }
        if hover_state.just_exited {
            event_writer.send(BHoverEvent::JustExited { entity });
        }
    }
}

pub fn generate_mouse_over_events(
    query: Query<(Entity, &BMouseOverState)>,
    mut event_writer: EventWriter<BMouseOverEvent>,
) {
    for (entity, mouse_over_state) in &query {
        if mouse_over_state.just_entered {
            event_writer.send(BMouseOverEvent::JustEntered { entity });
        }
        if mouse_over_state.just_exited {
            event_writer.send(BMouseOverEvent::JustExited { entity });
        }
    }
}