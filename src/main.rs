use bevy::prelude::*;
use tile::Tile;

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
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_map(mut commands: Commands) {
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
                Sprite::from_color(Color::hsl(246., 1., 0.5), Vec2::splat(TILE_SIZE)),
                Transform {
                    translation: Vec3::new(x, y, 0.),
                    ..Default::default()
                },
            ));
        }
    }
}
