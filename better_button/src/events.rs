use bevy::prelude::*;

#[derive(Event, Debug)]
pub enum BPressEvent {
    JustEntered {
        entity: Entity
    },
    JustExited {
        entity: Entity
    },
}

#[derive(Event, Debug)]
pub enum BHoverEvent {
    JustEntered {
        entity: Entity
    },
    JustExited {
        entity: Entity
    },
}

#[derive(Event, Debug)]
pub enum BMouseOverEvent {
    JustEntered {
        entity: Entity
    },
    JustExited {
        entity: Entity
    },
}