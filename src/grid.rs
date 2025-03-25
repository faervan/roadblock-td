use std::fmt::{Debug, Display};

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::{ecs::system::Resource, utils::HashSet};

use crate::bordered_rectangle::BorderedRectangle;

const ROWS: isize = 80;
const COLUMNS: isize = 140;
const TILE_SIZE: f32 = 10.;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Grid {
            grid: HashMap::new(),
        });
        app.add_systems(Startup, spawn_map);
    }
}

#[derive(Resource)]
pub struct Grid {
    pub grid: HashMap<GridPos, (Entity, TileType)>,
}

#[derive(Component, Clone, Copy, Default)]
pub struct Tile {
    pub pos: GridPos,
    pub tile_type: TileType,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Default)]
pub enum TileType {
    #[default]
    Empty,
    Tower,
    Enemy,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct GridPos {
    row: isize,
    col: isize,
}

impl Tile {
    pub fn new(row: isize, col: isize, tile_type: TileType) -> Self {
        Tile {
            pos: GridPos { row, col },
            tile_type,
        }
    }
}

impl GridPos {
    pub fn new(row: isize, col: isize) -> Self {
        GridPos { row, col }
    }

    pub fn distance_to(&self, other: &GridPos) -> usize {
        other.row.abs_diff(self.row) + other.col.abs_diff(self.col)
    }

    pub fn neighbors(&self, tiles: &HashSet<&GridPos>) -> Vec<GridPos> {
        let mut neighbors = vec![];

        let mut push_maybe = |row, col| {
            let tile = GridPos::new(row, col);
            tiles.contains(&tile).then(|| neighbors.push(tile));
        };

        push_maybe(self.row + 1, self.col);
        push_maybe(self.row - 1, self.col);
        push_maybe(self.row, self.col + 1);
        push_maybe(self.row, self.col - 1);

        neighbors
    }
}

impl Display for GridPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tile ({}, {})", self.row, self.col)
    }
}

impl Debug for GridPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut grid: ResMut<Grid>,
) {
    let position = |total: f32, current| -(total * 0.5 * TILE_SIZE) + current * TILE_SIZE;

    for row in 0..ROWS {
        let y = position(ROWS as f32, row as f32);
        for col in 0..COLUMNS {
            let x = position(COLUMNS as f32, col as f32);
            let tile = Tile::new(row, col, TileType::Empty);
            let entity = commands
                .spawn((
                    tile,
                    Mesh2d(meshes.add(BorderedRectangle::new(Vec2::splat(TILE_SIZE), 1.))),
                    MeshMaterial2d(materials.add(Color::hsl(246., 1., 0.5))),
                    Transform {
                        translation: Vec3::new(x, y, 0.),
                        ..Default::default()
                    },
                ))
                .id();

            grid.grid
                .insert(GridPos::new(row, col), (entity, TileType::Empty));
        }
    }
}

pub fn world_to_grid_coords(pos: Vec2) -> Option<GridPos> {
    let max_pos_x = TILE_SIZE * COLUMNS as f32 * 0.5;
    let max_pos_y = TILE_SIZE * ROWS as f32 * 0.5;

    if pos.x.abs() > max_pos_x || pos.y.abs() > max_pos_y {
        return None;
    }

    Some(GridPos {
        row: ((pos.y + max_pos_y) / TILE_SIZE).round() as isize,
        col: ((pos.x + max_pos_x) / TILE_SIZE).round() as isize,
    })
}
