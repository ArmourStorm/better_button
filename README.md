# Introduction

Expands on the `Interaction` component provided in Bevy by tracking more states, and whether those states have just been entered or exited.
These states are all bundled together in `BButtonBundle` as components, and can be used by querying for these components, or listening for the events they generate.

The library works by updating the additional button components based on the `Interaction` component placed along side them, which ensures parity with Bevy's own button behavior.

Simpy add the `BButtonPlugin` to your project and use `BButtonBundle` instead of Bevy's `ButtonBundle` to get started.

# Tutorial

## 1. Add The Plugin

```
fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins,
                BButtonPlugin
            )
        )
        .run();
}
```

This simply adds the neccesary systems to update the button states, and also registers the button events for you.

## 2. Spawn The BButtonBundle

The `BButtonBundle` simply bundles `bevy::prelude::ButtonBundle` along with the components used in this library, n.l. `BPressState`, `BHoverState` and `BMouseOverState`.

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

There are two main ways to 

# How It Works

## State Components

One of the intended ways you can respond to button interactions is by using the provided "state" components. 
These components each represent a state which only become active under certain conditions, 
and they also track whether the corresponding state just entered or exited activity.

> These components are updated based on the `bevy::prelude::Interaction` component included in the `bevy::prelude::ButtonBundle` to remain in parity with Bevy's own button design.

Here is a list of the state components provided, along with the conditions that trigger them:
 - `BPressState` - press (has parity with `Interaction::Pressed`)
 - `BHoverState` - mouse over, while not pressed (has parity with `Interaction::Hovered`)
 - `BMouseOverState` - mouse over, regardless of pressed

Uses `bevy::prelude::Interaction` to track the provided states.

# Advanced Use Cases
## Using Without The Plugin
## Using Without The Bundle
