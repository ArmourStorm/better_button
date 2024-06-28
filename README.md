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


There are two main ways to 

## How It Works

### State Components

One of the intended ways you can respond to button interactions is by using the provided "state" components. 
These components each represent a state which only become active under certain conditions, 
and they also track whether the corresponding state just entered or exited activity.

> These components are updated based on the `bevy::prelude::Interaction` component included in the `bevy::prelude::ButtonBundle` to remain in parity with Bevy's own button design.

Here is a list of the state components provided, along with the conditions that trigger them:
 - `BPressState` - press (has parity with `Interaction::Pressed`)
 - `BHoverState` - mouse over, while not pressed (has parity with `Interaction::Hovered`)
 - `BMouseOverState` - mouse over, regardless of pressed

Uses `bevy::prelude::Interaction` to track the provided states.

## Advanced Use Cases
### Using Without The Plugin
### Using Without The Bundle
