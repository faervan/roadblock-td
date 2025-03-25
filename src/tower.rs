use bevy::{prelude::*, window::PrimaryWindow};

use crate::grid::{
    Grid, GridPos, TILE_SIZE, Tile, TileType, grid_to_world_coords, world_to_grid_coords,
};

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, place_tower);
        app.register_type::<Tower>();
    }
}

#[derive(Reflect, Component)]
#[reflect(Component)]
#[require(Tile(tower_tile))]
struct Tower;

fn tower_tile() -> Tile {
    Tile {
        pos: GridPos::default(),
        tile_type: TileType::Tower,
    }
}

fn place_tower(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    cam: Single<(&Camera, &GlobalTransform)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut grid: ResMut<Grid>,
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
                if grid.grid.get(&grid_pos).is_none() {
                    let entity = commands
                        .spawn((
                            Tower,
                            Sprite::from_color(Color::srgb(0.0, 0.5, 1.0), Vec2::splat(TILE_SIZE)),
                            Transform {
                                translation: grid_to_world_coords(grid_pos).extend(0.0),
                                ..default()
                            },
                        ))
                        .id();
                    grid.grid.insert(grid_pos, (entity, TileType::Tower));
                }
            }
        } else {
            warn!("Unable to get Cursor Position {:?}", world_pos.unwrap_err())
        }
    }
}
