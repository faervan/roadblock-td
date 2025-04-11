use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};

pub use grid_pos::GridPos;

use crate::app_state::AppState;

mod grid_pos;

pub const ROWS: isize = 40;
pub const COLUMNS: isize = 70;
pub const TILE_SIZE: f32 = 25.;

const LINE_WIDTH: f32 = 1.5;

const GRID_COLOR: Color = Color::hsl(0.0, 0.0, 1.0);

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Grid>();
        app.init_resource::<Grid>();
        app.add_systems(OnEnter(AppState::Game), spawn_grid);
        app.add_systems(OnExit(AppState::Game), exit);
    }
}

#[derive(Reflect, Resource, Default, Debug)]
#[reflect(Resource)]
pub struct Grid {
    /// contains all tiles occupied by a tower
    pub towers: HashMap<GridPos, Entity>,
    /// stores the origin tile of every tower entity
    pub tower_origins: HashMap<Entity, GridPos>,
    pub enemy_spawners: HashMap<GridPos, Entity>,
    pub enemy_goals: HashMap<GridPos, Entity>,
    pub unbuildable: HashSet<GridPos>,
}

impl Grid {
    pub fn is_free(&self, position: &GridPos) -> bool {
        !self.towers.contains_key(position)
            && !self.enemy_spawners.contains_key(position)
            && !self.enemy_goals.contains_key(position)
            && !self.unbuildable.contains(position)
    }
}

pub fn spawn_grid(mut commands: Commands) {
    commands.insert_resource(Grid::default());

    let position = |total: f32, current| (-(total * 0.5 * TILE_SIZE) + current * TILE_SIZE) - TILE_SIZE * 0.5;

    let total_size_x = ROWS as f32 * TILE_SIZE;
    let total_size_y = COLUMNS as f32 * TILE_SIZE;

    for column in 0..=COLUMNS {
        let x = position(COLUMNS as f32, column as f32);
        commands.spawn((
            Name::new(format!("Grid column border: {column}")),
            Sprite::from_color(
                GRID_COLOR,
                Vec2 {
                    x: LINE_WIDTH,
                    y: total_size_x,
                },
            ),
            Transform {
                translation: Vec3 {
                    x,
                    y: -TILE_SIZE * 0.5,
                    z: 0.0,
                },
                ..default()
            },
        ));
    }

    for row in 0..=ROWS {
        let y = position(ROWS as f32, row as f32);
        commands.spawn((
            Name::new(format!("Grid row border: {row}")),
            Sprite::from_color(
                GRID_COLOR,
                Vec2 {
                    x: total_size_y,
                    y: LINE_WIDTH,
                },
            ),
            Transform {
                translation: Vec3 {
                    x: -TILE_SIZE * 0.5,
                    y,
                    z: 0.0,
                },
                ..default()
            },
        ));
    }
}

fn exit(mut commands: Commands) {
    commands.remove_resource::<Grid>();
}

pub fn world_to_grid_coords(pos: Vec2) -> Option<GridPos> {
    let max_pos_x = TILE_SIZE * COLUMNS as f32 * 0.5;
    let max_pos_y = TILE_SIZE * ROWS as f32 * 0.5;

    if (pos.x + TILE_SIZE * 0.5).abs() > max_pos_x || (pos.y + TILE_SIZE * 0.5).abs() > max_pos_y {
        return None;
    }

    Some(GridPos {
        row: ((pos.y + max_pos_y) / TILE_SIZE).round() as isize,
        col: ((pos.x + max_pos_x) / TILE_SIZE).round() as isize,
    })
}

pub fn grid_to_world_coords(pos: GridPos) -> Vec2 {
    Vec2 {
        x: -(COLUMNS as f32 * 0.5 * TILE_SIZE) + pos.col as f32 * TILE_SIZE,
        y: -(ROWS as f32 * 0.5 * TILE_SIZE) + pos.row as f32 * TILE_SIZE,
    }
}
