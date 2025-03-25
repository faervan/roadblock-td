use bevy::{
    input::mouse::AccumulatedMouseMotion, prelude::*, window::PrimaryWindow, winit::WinitWindows,
};
use bordered_rectangle::BorderedRectangle;
use path_finding::enemy_get_path;
use tile::Tile;

mod bordered_rectangle;
mod path_finding;
mod tile;

const TILE_SIZE: f32 = 10.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, (init, spawn_map))
        .add_systems(Update, (enemy_get_path, pan_camera, exit_on_ctrl_q))
        .run();
}

#[derive(Resource, Debug)]
struct MapInfo {
    size: Vec2,
    /// Bottom left anchor of the map in bevy's coordinate system
    anchor: Vec2,
}

#[derive(Component)]
struct Enemy {
    current: Tile,
    goal: Tile,
}

#[derive(Component)]
struct EnemyPath(Vec<Tile>);

fn init(
    mut commands: Commands,
) {
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

fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let rows = 80;
    let columns = 140;

    let position = |total: f32, current| -(total * 0.5 * TILE_SIZE) + current * TILE_SIZE;

    for row in 0..rows {
        let y = position(rows as f32, row as f32);
        for col in 0..columns {
            let x = position(columns as f32, col as f32);
            let tile = Tile::new(row, col);
            commands.spawn((
                tile,
                Mesh2d(meshes.add(BorderedRectangle::new(Vec2::splat(TILE_SIZE), 1.))),
                MeshMaterial2d(materials.add(Color::hsl(246., 1., 0.5))),
                Transform {
                    translation: Vec3::new(x, y, 0.),
                    ..Default::default()
                },
            ));
        }
    }
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
