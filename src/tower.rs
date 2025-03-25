use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};

use crate::grid::{
    Grid, GridPos, TILE_SIZE, Tile, TileType, grid_to_world_coords, world_to_grid_coords,
};

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedTower { tower: Tower::Wall });
        app.add_systems(Update, place_tower);
        app.register_type::<Tower>();
    }
}

#[derive(Reflect, Component)]
#[reflect(Component)]
#[require(Tile(tower_tile))]
enum Tower {
    Wall,
    SpikedWall,
    Canon,
}

impl Tower {
    fn max_hp(&self) -> u32 {
        match self {
            Self::Wall => 100,
            Self::SpikedWall => 100,
            Self::Canon => 80,
        }
    }

    fn size(&self) -> (isize, isize) {
        match self {
            Self:: Wall => (4, 2),
            Self::SpikedWall => (4, 2),
            Self::Canon => (3, 3),
        }
    }

    fn range(&self) -> f32 {
        match self {
            Self::Canon => TILE_SIZE * 10.0,
            _ => 0.0,
        }
    }

    fn strength(&self) -> u32 {
        match self {
            Self::Canon => 15,
            _ => 0,
        }
    }

    fn fire_cooldown(&self) -> Duration {
        match self {
            Self::Canon => Duration::from_secs(1),
            _ => Duration::ZERO,
        }
    }

    fn contact_damage(&self) -> u32 {
        match self {
            Self::SpikedWall => 5,
            _ => 0,
        }
    }

    fn contact_damage_cooldown(&self) -> Duration {
        match self {
            Self::SpikedWall => Duration::from_secs(1),
            _ => Duration::ZERO,
        }
    }
}

fn tower_tile() -> Tile {
    Tile {
        pos: GridPos::default(),
        tile_type: TileType::Tower,
    }
}

#[derive(Reflect, Resource)]
#[reflect(Resource)]
struct SelectedTower {
    tower: Tower
}

fn place_tower(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    cam: Single<(&Camera, &GlobalTransform)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut grid: ResMut<Grid>,
    tower: Res<SelectedTower>
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    let mouse_pos = window.cursor_position();

    if let Some(mouse_pos) = mouse_pos {
        let (camera, cam_transform) = *cam;

        let world_pos = camera.viewport_to_world_2d(cam_transform, mouse_pos);
        if let Ok(world_pos) = world_pos {
            if let Some(grid_pos) = world_to_grid_coords(world_pos) {
                for i in 0..tower.tower.size().0 {
                    for j in 0..tower.tower.size().1 {
                        let pos = GridPos { col: grid_pos.col + i, row: grid_pos.row + j};
                        if !grid.is_free(&pos) {
                            return;
                        }
                    }
                }
                let entity = commands
                    .spawn((
                        Tower::Wall,
                        Tile {
                            pos: grid_pos,
                            tile_type: TileType::Tower,
                        },
                        Sprite {
                            color: Color::srgb(0.0, 0.5, 1.0),
                            custom_size: Some(Vec2 {x: tower.tower.size().0 as f32 * TILE_SIZE, y: tower.tower.size().1 as f32 * TILE_SIZE}),
                            anchor: bevy::sprite::Anchor::BottomLeft,
                            ..default()
                        },
                        //Sprite::from_color(Color::srgb(0.0, 0.5, 1.0), Vec2 {x: tower.tower.size().0 as f32 * TILE_SIZE, y: tower.tower.size().1 as f32 * TILE_SIZE}),
                        Transform {
                            translation: (grid_to_world_coords(grid_pos) - (TILE_SIZE * 0.5)).extend(1.0),
                            ..default()
                        },
                    ))
                    .id();
                
                    for i in 0..tower.tower.size().0 {
                        for j in 0..tower.tower.size().1 {
                            let pos = GridPos { col: grid_pos.col + i, row: grid_pos.row + j};
                            grid.tower.insert(pos, entity);
                        }
                    }
            }
        } else {
            warn!("Unable to get Cursor Position {:?}", world_pos.unwrap_err())
        }
    }
}
