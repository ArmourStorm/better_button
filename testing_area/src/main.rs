use bevy::prelude::*;
use bevy::color::palettes::css::*;
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
        // .add_systems(
        //     Update,
        //     respond_to_button_state.after(BButtonUpdateSet)
        // )
        .add_systems(
            FixedUpdate,
            respond_to_button_events,
        )
        .run();
}

fn respond_to_button_events(
    mut query: Query<&mut BackgroundColor, With<BHoverState>>,
    mut event_reader: EventReader<BHoverEvent>,
) {
    for event in event_reader.read() {
        match event {
            BHoverEvent::JustEntered { entity } => {
                if let Ok(mut background_color) = query.get_mut(*entity) {
                    background_color.0 = YELLOW_GREEN.into();
                }
            }
            BHoverEvent::JustExited { entity } => {
                if let Ok(mut background_color) = query.get_mut(*entity) {
                    background_color.0 = WHITE.into();
                }
            }
        }
    }
}

fn respond_to_button_state(
    mut query: Query<(&BPressState, &mut BackgroundColor)>
) {
    for (state, mut background_color) in &mut query {
        if state.just_entered {
            background_color.0 = YELLOW_GREEN.into();
        }
        if state.just_exited {
            background_color.0 = WHITE.into();
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
                background_color: BackgroundColor(WHITE.into()),
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