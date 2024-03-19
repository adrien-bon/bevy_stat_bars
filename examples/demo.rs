use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_stat_bars::*;
use std::marker::PhantomData;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct WizardCharacter;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Stat<T>
where
    T: Component,
{
    pub value: f32,
    pub max: f32,
    #[reflect(ignore)]
    phantom: PhantomData<T>,
}

impl<T> Default for Stat<T>
where
    T: Component,
{
    fn default() -> Self {
        Self {
            value: 10.0,
            max: 10.0,
            phantom: PhantomData,
        }
    }
}

impl<T> Stat<T>
where
    T: Component,
{
    fn new_full(value: f32) -> Self {
        assert!(0. < value);
        Self {
            value,
            max: value,
            ..Default::default()
        }
    }
}

impl<T> std::ops::AddAssign<f32> for Stat<T>
where
    T: Component,
{
    fn add_assign(&mut self, rhs: f32) {
        self.value = (self.value + rhs).clamp(0.0, self.max);
    }
}

impl<T> std::ops::SubAssign<f32> for Stat<T>
where
    T: Component,
{
    fn sub_assign(&mut self, rhs: f32) {
        self.value = (self.value - rhs).clamp(0.0, self.max);
    }
}

impl<T> StatbarObservable for Stat<T>
where
    T: Component,
{
    fn get_statbar_value(&self) -> f32 {
        self.value / self.max
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct HealthValue;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct MagicValue;

type Health = Stat<HealthValue>;
type Magic = Stat<MagicValue>;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_demo(mut commands: Commands, asset_server: Res<AssetServer>) {
    let wizard_id = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(128. * Vec2::ONE),
                ..Default::default()
            },
            texture: asset_server.load("wizard.png"),
            ..Default::default()
        })
        .insert((
            WizardCharacter,
            Health::new_full(20.0),
            Magic::new_full(17.0),
            Statbar::<Health> {
                empty_color: Color::rgb(0., 0.1, 0.),
                length: 100.0,
                thickness: 16.0,
                displacement: 70. * Vec2::Y,
                ..Default::default()
            },
            StatbarBorder::<Health>::all(Color::DARK_GRAY, 2.0),
            StatbarColorSwitch::<Health>::new(0.33, Color::RED, Color::rgb(0., 0.8, 0.)),
            Statbar::<Magic> {
                empty_color: Color::rgb(0.1, 0.0, 0.1),
                length: 100.0,
                thickness: 16.0,
                displacement: 90. * Vec2::Y,
                ..Default::default()
            },
            StatbarBorder::<Magic>::all(Color::DARK_GRAY, 2.0),
            StatbarColorLerp::<Magic>::new(Color::rgb(0.5, 0.0, 0.5), Color::FUCHSIA),
        ))
        .id();

    commands
        .spawn((
            Statbar::<Health> {
                color: Color::WHITE,
                empty_color: Color::BLACK,
                length: 500.0,
                thickness: 50.0,
                ..Default::default()
            },
            StatbarObserveEntity(wizard_id),
        ))
        .insert(SpatialBundle {
            transform: Transform::from_translation(-200. * Vec3::Y),
            ..Default::default()
        });
}

fn move_character(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<WizardCharacter>>,
) {
    let speed = 60.;
    player_query.iter_mut().for_each(|mut player_transform| {
        let translation = &mut player_transform.translation;
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            translation.x -= time.delta_seconds() * speed;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            translation.x += time.delta_seconds() * speed;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            translation.y -= time.delta_seconds() * speed;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            translation.y += time.delta_seconds() * speed;
        }
    });
}

fn adjust_stats(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut hp_query: Query<&mut Health>,
    mut mp_query: Query<&mut Magic>,
) {
    let delta = 5.0 * time.delta_seconds();
    hp_query.iter_mut().for_each(|mut hp| {
        if keyboard_input.pressed(KeyCode::KeyA) {
            *hp -= delta;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            *hp += delta;
        }
    });
    mp_query.iter_mut().for_each(|mut mp| {
        if keyboard_input.pressed(KeyCode::KeyQ) {
            *mp -= delta;
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            *mp += delta;
        }
    });
}

fn spawn_instructions(mut commands: Commands, asset_server: Res<AssetServer>) {
    let title_style = TextStyle {
        font: asset_server.load("FiraMono-Regular.ttf"),
        font_size: 32.0,
        color: Color::YELLOW,
    };
    let text_style = TextStyle {
        font: asset_server.load("FiraMono-Regular.ttf"),
        font_size: 24.0,
        color: Color::ANTIQUE_WHITE,
    };

    commands.spawn(
        NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::NONE),
        ..Default::default()
    }).with_children(|builder| {
        builder.spawn(
            TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "bevy_stat_bars demo\n\n".to_string(), 
                            style: title_style
                        },
                        TextSection {
                            value:
                            "left, right, down, up keys => move wizard\nQ, W => -/+ magic stat\nA, S => -/+ health stat".to_string(),
                            style: text_style
                        }
                    ],
                    justify: JustifyText::Center,
                    linebreak_behavior: bevy::text::BreakLineOn::WordBoundary,
                },
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    ..Default::default()
                },
                ..Default::default()
        });
    });
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1000., 1000.),
                        resizable: true,
                        ..Default::default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(WorldInspectorPlugin::new())
        .register_type::<Health>()
        .register_type::<Magic>()
        .register_type::<WizardCharacter>()
        .add_statbar_component_observer::<Health>()
        .add_statbar_component_observer::<Magic>()
        .add_systems(Startup, (spawn_camera, spawn_demo, spawn_instructions))
        .add_systems(Update, (move_character, adjust_stats))
        .run();
}
