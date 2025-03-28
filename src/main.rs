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

    // only show debug logs on a debug build
    #[cfg(debug_assertions)]
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: bevy::window::PresentMode::AutoNoVsync,
                    mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            })
            .set(LogPlugin {
                level: Level::DEBUG,
                ..default()
            }),
    );

    #[cfg(not(debug_assertions))]
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Current),
            ..default()
        }),
        ..default()
    }));

    if std::env::args().any(|a| a == "--egui") {
        app.add_plugins(WorldInspectorPlugin::new());
    }

    app.insert_resource(RngResource(Rng::new()));

    app.add_plugins((
        animation::AnimationPlugin,
        GridPlugin,
        MapPlugin,
        TowerPlugin,
        EnemyPlugin,
        UIPlugin,
    ));
    app.run();
}

#[derive(Resource)]
struct RngResource(Rng);

#[derive(Reflect, Default, PartialEq, Debug)]
enum Orientation {
    #[default]
    Up,
    Down,
    Left,
    Right,
}
