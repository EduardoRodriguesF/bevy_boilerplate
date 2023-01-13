use bevy_boilerplate::prelude::*;
use bevy_boilerplate::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: SCREEN_WIDTH as f32,
                height: SCREEN_HEIGHT as f32,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_plugin(GamePlugin)
        .run();
}
