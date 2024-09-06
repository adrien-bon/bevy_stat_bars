use bevy::prelude::*;
use bevy_stat_bars::*;

// Spawns a red and navy statbar with a white border in the middle of the window.
// The left and right cursor keys decrease and increase the value of the bar.

/// A minimal newtype struct that implements `StatbarObservable`
#[derive(Copy, Clone, Default, Component, Reflect)]
#[reflect(Component)]
pub struct ObservedValue(pub f32);

impl StatbarObservable for ObservedValue {
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
            Statbar::<ObservedValue> {
                color: Color::from(bevy::color::palettes::css::RED),
                empty_color: Color::from(bevy::color::palettes::css::NAVY),
                length: 400.,
                thickness: 40.,
                ..Default::default()
            },
            StatbarBorder::<ObservedValue>::all(Color::WHITE, 4.0),
            ObservedValue(0.35),
        ))
        .insert(SpatialBundle::default());
}

fn adjust_value(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut observed_values: Query<&mut ObservedValue>,
) {
    let delta = time.delta_seconds() * 0.25;
    observed_values.iter_mut().for_each(|mut value| {
        if input.pressed(KeyCode::ArrowLeft) {
            value.0 -= delta;
        }
        if input.pressed(KeyCode::ArrowRight) {
            value.0 += delta;
        }
        value.0 = value.0.clamp(0., 1.0);
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_statbar_component_observer::<ObservedValue>()
        .add_systems(Startup, (spawn_camera, spawn_statbar))
        .add_systems(Update, adjust_value)
        .run();
}
