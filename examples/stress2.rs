use bevy::math::vec2;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::window::WindowMode;
use bevy_stat_bars::*;

#[derive(TypePath)]
struct StatbarMarker<const N: usize>;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_statbars(mut commands: Commands) {
    let length = 16.;
    let space = 2.;
    let thickness = 2.;
    let s = -0.5 * vec2(100. * (length + space), 200. * (space + space));
    let mut displacement = s;

    for _ in 0..100 {
        let mut entity_commands = commands.spawn(SpatialBundle::default());
        seq_macro::seq!(N in 0 .. 200 {
            entity_commands.insert(Statbar::<StatbarMarker<N>> {
                color: Color::WHITE,
                empty_color: Color::from(bevy::color::palettes::css::BLUE),
                length,
                thickness,
                displacement,
                ..Default::default()
            })
            .insert(StatbarColorLerp::<StatbarMarker<N>>::new(Color::from(bevy::color::palettes::css::RED), Color::WHITE)) ;
            displacement.y += thickness + space;
        });
        displacement.y = s.y;
        displacement.x += length + space;
    }
}

fn adjust_stats<const N: usize>(
    time: Res<Time>,
    mut statbar: Query<&mut Statbar<StatbarMarker<N>>>,
) {
    statbar.iter_mut().for_each(|mut bar| {
        bar.value = time.elapsed_seconds().sin().abs();
    });
}

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::srgb(0.0, 0.5, 0.0)))
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::Immediate,
                        mode: WindowMode::Fullscreen,
                        ..default()
                    }),
                    ..default()
                }),
            bevy::diagnostic::LogDiagnosticsPlugin::default(),
            bevy::diagnostic::FrameTimeDiagnosticsPlugin,
        ))
        .add_systems(Startup, (spawn_camera, spawn_statbars));

    seq_macro::seq!(N in 0  .. 200 {
        app.add_standalone_statbar::<StatbarMarker<N>>()
            .add_systems(Update, adjust_stats::<N>);
    });

    app.run();
}
