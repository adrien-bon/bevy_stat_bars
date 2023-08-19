use bevy::prelude::*;
use bevy_stat_bars::*;

// spawns a statbar in the middle of the window

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_standalone_statbar::<()>()
        .add_startup_system(|mut commands: Commands| {
            commands
                .spawn(Camera2dBundle::default())
                .commands()
                .spawn(SpatialBundle::default())
                .insert(Statbar::<()>::default());
        })
        .run();
}
