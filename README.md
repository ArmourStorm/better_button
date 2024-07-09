## Introduction

Expands on the `Interaction` component provided in Bevy by tracking more states, and whether those states have just been entered or exited. These states are all bundled together in `BButtonBundle` as components, and can be used by querying for these components, or listening for the events they generate.

The library works by updating the additional button components based on the `Interaction` component placed along side them, which ensures parity with Bevy's own button behavior.

Simpy add the `BButtonPlugin` to your project and use `BButtonBundle` instead of Bevy's `ButtonBundle` to get started.

## Tutorial

### 1. Setup

Create a new binary crate, add bevy as a dependency and copy the following code into your `main.rs`:

```
use bevy::prelude::*;
use bevy::color::palettes::css::*;

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
use bevy::color::palettes::css::*;
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

This simply adds the necessary systems to update the button states, and also registers the button events for you.

### 3. Spawn The BButtonBundle

The `BButtonBundle` contains the Bevy `ButtonBundle` along with the button components provided by the `better_button` crate.

Create a new system to spawns your first `BButtonBundle`:

```
fn spawn_button(mut commands: Commands) {
    commands.spawn(BButtonBundle::new(
        ButtonBundle {
            style: Style
            {
                width: Val::Px(125.0),
                height: Val::Px(125.0),
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                ..default()
            },
            background_color: BackgroundColor(WHITE.into()),
            ..default()
        }
    ));
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

Create a new system that changes the button's color to when pressed:

```
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
```

The system queries the `BPressState` component, which is a part of the `BButtonBundle` we used earlier.

Now add the system to your app, but make sure specify that it should run before or after the `BButtonUpdateSet`. This ensures that your system will run at least once between consecutive runs of the `BButtonUpdateSet`, since the order in which systems and system sets run in Bevy can change from frame-to-frame if left unordered:

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
            respond_to_button_state.after(BButtonUpdateSet) // <------- Add the system, and set it to run after the `BButtonUpdateSet`.
        )
        .run();
}
```

### 4. Respond To Button Presses By Reading Events

Since the state components of the button we created are updated in the `Update` schedule when using the `BButtonPlugin`, we cannot reliably read button presses from systems in the `FixedUpdate` schedule with queries on the button components. This is because the `just_entered` and `just_exited` properties of the `BPressState` component are only set to `true` for one frame in the `Update` schedule. And since in most cases the `Update` schedule runs multiple times for each `FixedUpdate`, the `just_entered` and `just_exited` properties could be set to `true` and then back to `false` between two `FixedUpdate` frames.

This is where using events come in hand, since Bevy ensures that all systems in both `Update` and `FixedUpdate` receive any events generated exactly once before the events disappear.

Create a new system that changes the button's color when hovered, this time using events:

```
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
```

Comment out the previous system and add the latest one to the `FixedUpdate` schedule in your Bevy app:

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
        // .add_systems(
        //     Update,
        //     respond_to_button_state.after(BButtonUpdateSet)
        // )
        .add_systems(
            FixedUpdate,
            respond_to_button_events, // <------- Add the `respond_to_button_events` system to `FixedUpdate`.
        )
        .run();
}
```

As you might have noticed, we do not need to specify whether our new system runs before or after `BButtonUpdateSet`. As mentioned earlier, Bevy ensures that all systems receive the events they read exactly once. So even if we did add our new system to the `Update` schedule without using `.after(BButtonUpdateSet)`, we can know for sure that it will not miss anything.

### 5. Conclusion

The decision whether to use the `better_button` crate with its events or by querying the components directly is up to you. There are pros and cons for both techniques.

In general, when working in the `Update` schedule, it tends to be easier to query the button components directly when working with non-gameplay related logic. Like updating your button's visuals. On the other hand, using events are necessary when you are working in `FixedUpdate`. For example, using a button press to make a character jump on mobile devices.

## What Next?

Please check out the [Wiki](https://github.com/ArmourStorm/better_button/wiki) for more information about this library.
