pub mod essentials;
pub mod physics;
pub mod prelude;
pub mod utils;

use essentials::*;
use physics::*;
use prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EssentialPlugins)
            .add_plugin(PhysicsPlugin)
            .add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz((SCREEN_WIDTH / 2) as f32, (SCREEN_HEIGHT / 2) as f32, 1.),
        ..default()
    });
}
