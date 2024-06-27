
When using `BButtonPlugin

# How To

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
 > See [Using Without The Plugin](#using-without-the-plugin)

## 2. Spawn A BButtonBundle

The `BButtonBundle` simply bundle `bevy::prelude::ButtonBundle` along with the components needed for this library.
> See [Using Without The Bundle](#using-without-the-bundle)



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



# Using Without The Plugin

# Using Without The Bundle
