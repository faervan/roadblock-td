use bevy::{prelude::*, window::PrimaryWindow};

use crate::grid::{Grid, Tile, TileType, world_to_grid_coords};

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, place_tower);
    }
}

#[derive(Component)]
#[require(Tile)]
struct Tower;

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
                if let Some(cur_entity) = grid.grid.get(&grid_pos) {
                    match cur_entity.1 {
                        TileType::Empty => {
                            // Currently only deletes the current tile, need to implement spawning new tower still
                            commands.entity(cur_entity.0).despawn();
                            grid.grid.remove(&grid_pos);
                        }
                        _ => todo!(),
                    }
                }
            }
        } else {
            warn!("Unable to get Cursor Position {:?}", world_pos.unwrap_err())
        }
    }
}
