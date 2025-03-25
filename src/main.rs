use bevy::{
    input::mouse::AccumulatedMouseMotion,
    log::{Level, LogPlugin},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use enemy::EnemyPlugin;
use grid::GridPlugin;
use path_finding::PathfindingPlugin;
use tower::TowerPlugin;

mod enemy;
mod grid;
mod path_finding;
mod tower;

fn main() {
    let mut app = App::new();

    // only show debug logs on a debug build
    #[cfg(debug_assertions)]
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: bevy::window::PresentMode::AutoNoVsync,
                    mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Current),
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
    app.register_type::<MapInfo>();

    app.add_plugins((PathfindingPlugin, GridPlugin, TowerPlugin, EnemyPlugin));
    app.add_systems(Startup, init);
    app.add_systems(Update, (pan_camera, exit_on_ctrl_q));
    app.run();
}

#[derive(Reflect, Resource, Debug)]
#[reflect(Resource)]
struct MapInfo {
    size: Vec2,
    /// Bottom left anchor of the map in bevy's coordinate system
    anchor: Vec2,
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2d);

    let map_size = Vec2::new(3000., 2000.);
    let map_anchor = Vec2::new(-map_size.x / 2., -map_size.y / 2.);

    commands.spawn((
        Sprite::from_color(Color::hsl(91., 1., 0.5), map_size),
        Transform::from_translation(Vec3::new(
            map_anchor.x + map_size.x / 2.,
            map_anchor.y + map_size.y / 2.,
            -1.,
        )),
    ));
    commands.insert_resource(MapInfo {
        size: map_size,
        anchor: map_anchor,
    });
}

fn pan_camera(
    mut camera: Single<&mut Transform, With<Camera>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    map_info: Res<MapInfo>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    if mouse_input.pressed(MouseButton::Middle) && motion.delta != Vec2::ZERO {
        camera.translation.x -= motion.delta.x;
        camera.translation.y += motion.delta.y;

        let window_size = window.size() / 2.;

        // Prevent panning to infinity
        if camera.translation.x < map_info.anchor.x + window_size.x {
            camera.translation.x = map_info.anchor.x + window_size.x;
        } else if camera.translation.x > map_info.anchor.x + map_info.size.x - window_size.x {
            camera.translation.x = map_info.anchor.x + map_info.size.x - window_size.x;
        }

        if camera.translation.y < map_info.anchor.y + window_size.y {
            camera.translation.y = map_info.anchor.y + window_size.y;
        } else if camera.translation.y > map_info.anchor.y + map_info.size.y - window_size.y {
            camera.translation.y = map_info.anchor.y + map_info.size.y - window_size.y;
        }
    }
}

fn exit_on_ctrl_q(mut app_exit: EventWriter<AppExit>, input: Res<ButtonInput<KeyCode>>) {
    if input.pressed(KeyCode::ControlLeft) && input.just_pressed(KeyCode::KeyQ) {
        app_exit.send(AppExit::Success);
    }
}
