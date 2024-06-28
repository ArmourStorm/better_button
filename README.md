## Introduction

Expands on the `Interaction` component provided in Bevy by tracking more states, and whether those states have just been entered or exited.
These states are all bundled together in `BButtonBundle` as components, and can be used by querying for these components, or listening for the events they generate.

The library works by updating the additional button components based on the `Interaction` component placed along side them, which ensures parity with Bevy's own button behavior.

Simpy add the `BButtonPlugin` to your project and use `BButtonBundle` instead of Bevy's `ButtonBundle` to get started.

## Tutorial

### 1. Setup

Create a new binary crate, add bevy as a dependency and copy the following code into your `main.rs`:

```
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins,
            )
        )
        .add_systems(
            Startup,
            (
                spawn_camera,
            ),
        )
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(
        Camera3dBundle {
            ..default()
        }
    );
}
```

### 2. Add A BButtonPlugin

Import the `better_button` prelude and add the `BButtonPlugin` to your app:

```
use bevy::prelude::*;
use better_button::prelude::*; // <------- Import the `better_button` prelude.

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins,
                BButtonPlugin // <------- Add the `BButtonPlugin` to the app.
            )
        )
        .run();
}
```

This simply adds the neccesary systems to update the button states, and also registers the button events for you.

### 3. Spawn The BButtonBundle

The `BButtonBundle` contains the Bevy `ButtonBundle` along with the button components provided by the `better_button` crate.

Create a new system to spawns your first `BButtonBundle`:

```
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
```

Add it to your Bevy app:

```
fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins,
            )
        )
        .add_systems(
            Startup,
            (
                spawn_camera,
                spawn_button // <------- Add the `spawn_button` system to the `Startup` schedule.
            ),
        )
        .run();
}
```

### 4. Respond To Button Presses In `Update`

Create a new system that changes the button's colour to when pressed:

```
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
```

The system queries the `BPressState` component, which is a part of the `BButtonBundle` we used earlier.

Now we can add the system to the `Update` schedule in your app:

```
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
            Update, // <------- Make sure it's in the `Update` schedule. The reason will be explained later.
            respond_to_button_state // <------- Add the system.
        )
        .run();
}
```

## Where do I go from here?

Please check out the wiki for the project over at https://github.com/ArmourStorm/better_button/wiki for more information!

Also, consider checking out my Youtube channel for other Bevy tutorials: https://www.youtube.com/channel/UCNxrqjSmo5Pke_B1Lc3p6LA.
