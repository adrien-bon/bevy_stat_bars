use bevy::prelude::*;
use bevy_stat_bars::*;

// Spawns a red and navy statbar with a white border in the middle of the window.
// The left and right cursor keys decrease and increase the value of the bar.

#[derive(Resource, Reflect)]
struct ObservedResource(f32);

impl StatbarObservable for ObservedResource {
    fn get_statbar_value(&self) -> f32 {
        self.0
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_statbar(mut commands: Commands) {
    commands
        .spawn((
            Statbar::<ObservedResource> {
                color: Color::from(bevy::color::palettes::css::RED),
                empty_color: Color::from(bevy::color::palettes::css::NAVY),
                length: 500.,
                thickness: 50.,
                vertical: true,
                ..Default::default()
            },
            StatbarBorder::<ObservedResource>::all(Color::WHITE, 10.0),
        ))
        .insert(SpatialBundle::default());
}

fn adjust_value(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut my_resource: ResMut<ObservedResource>,
) {
    let delta = time.delta_seconds() * 0.25;
    if input.pressed(KeyCode::ArrowDown) {
        my_resource.0 -= delta;
    }
    if input.pressed(KeyCode::ArrowUp) {
        my_resource.0 += delta;
    }
    my_resource.0 = my_resource.0.clamp(0., 1.0);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ObservedResource(1.0))
        .add_statbar_resource_observer::<ObservedResource>()
        .add_systems(Startup, (spawn_camera, spawn_statbar))
        .add_systems(Update, adjust_value)
        .run();
}
