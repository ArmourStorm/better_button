use bevy::prelude::*;
use better_button::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins,
                BButtonPlugin
            )
        )
        .add_systems(
            Startup,
            (
                spawn_camera,
                spawn_button
            ),
        )
        .add_systems(
            Update,
            respond_to_button_state.after(BButtonUpdateSet)
        )
        .run();
}

fn respond_to_button_state(
    mut query: Query<(&BPressState, &mut BackgroundColor)>
) {
    for (state, mut background_color) in &mut query {
        if state.just_entered {
            background_color.0 = Color::GREEN;
        }
        if state.just_exited {
            background_color.0 = Color::WHITE;
        }
    }
}

fn spawn_button(mut commands: Commands) {
    commands.spawn(
        BButtonBundle {
            button_bundle: ButtonBundle {
                style: Style {
                    width: Val::Px(125.0),
                    height: Val::Px(125.0),
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    ..default()
                },
                ..default()
            },
            ..default()
        }
    );
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(
        Camera3dBundle {
            ..default()
        }
    );
}

//
// fn main() {
//     App::new()
//         .add_plugins(
//             (
//                 DefaultPlugins,
//                 BButtonPlugin,
//             )
//         )
//         .add_systems(
//             Startup,
//             (
//                 spawn_camera,
//                 spawn_b_button,
//             ),
//         )
//         .add_systems(
//             Update,
//             respond_to_button_events,
//         )
//         .run();
// }

// fn spawn_b_button(mut commands: Commands) {
//     commands.spawn(
//         BButtonBundle {
//             button_bundle: ButtonBundle {
//                 style: Style {
//                     width: Val::Px(125.0),
//                     height: Val::Px(125.0),
//                     align_self: AlignSelf::Center,
//                     justify_self: JustifySelf::Center,
//                     ..default()
//                 },
//                 ..default()
//             },
//             ..default()
//         }
//     );
// }

// fn respond_to_button_state(
//     mut query: Query<(&BMouseOverState, &mut BackgroundColor)>
// ) {
//     for (under_state, mut background_color) in &mut query {
//         if under_state.just_entered {
//             background_color.0 = Color::GREEN;
//         }
//         if under_state.just_exited {
//             background_color.0 = Color::WHITE;
//         }
//     }
// }

// fn respond_to_button_events(
//     mut query: Query<&mut BackgroundColor, With<BPressState>>,
//     mut event_reader: EventReader<BPressEvent>,
// ) {
//     for event in event_reader.read() {
//         match event {
//             BPressEvent::JustEntered { entity } => {
//                 if let Ok(mut background_color) = query.get_mut(*entity) {
//                     background_color.0 = Color::CYAN;
//                 }
//             }
//             BPressEvent::JustExited { entity } => {
//                 if let Ok(mut background_color) = query.get_mut(*entity) {
//                     background_color.0 = Color::WHITE;
//                 }
//             }
//         }
//     }
// }