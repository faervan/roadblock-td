use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use enemy::EnemyPlugin;
use fastrand::Rng;
use grid::GridPlugin;
use map::MapPlugin;
use tower::TowerPlugin;
use ui::UIPlugin;

mod animation;
mod enemy;
mod grid;
mod map;
mod tower;
mod ui;

fn main() {
    let mut app = App::new();

    let mut default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            #[cfg(debug_assertions)]
            present_mode: bevy::window::PresentMode::AutoNoVsync,
            mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
            ..default()
        }),
        ..default()
    });

    // only show debug logs on a debug build, unless the user specifies to keep the log level sane
    #[cfg(debug_assertions)]
    if !std::env::args().any(|a| a == "--ks" || a == "--keep-sanity") {
        default_plugins = default_plugins.set(LogPlugin {
            level: Level::DEBUG,
            ..default()
        });
    }

    if std::env::args().any(|a| a == "--egui") {
        app.add_plugins(WorldInspectorPlugin::new());
    }

    app.register_type::<Health>();

    app.insert_resource(RngResource(Rng::new()));

    app.add_plugins((
        default_plugins,
        animation::AnimationPlugin,
        GridPlugin,
        MapPlugin,
        TowerPlugin,
        EnemyPlugin,
        UIPlugin,
    ));
    app.run();
}

#[derive(Resource, Deref, DerefMut)]
struct RngResource(Rng);

#[derive(Reflect, Default, PartialEq, Debug, Clone, Copy)]
enum Orientation {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Orientation {
    fn is_horizontal(&self) -> bool {
        match self {
            Orientation::Up | Orientation::Down => false,
            Orientation::Left | Orientation::Right => true,
        }
    }
}

#[derive(Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
struct Health(isize);
