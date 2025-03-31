use std::ops::{Deref, DerefMut};

use bevy::{input::common_conditions::input_just_pressed, prelude::*, window::PrimaryWindow};

use crate::{
    Health, Orientation,
    app_state::{AppState, TowerPlacing, set_tower_placing_state},
    enemy::PathChangedEvent,
    grid::{COLUMNS, Grid, GridPos, ROWS, TILE_SIZE, grid_to_world_coords, world_to_grid_coords},
};

use super::{Tower, TowerType};

pub struct TowerPlacingPlugin;

impl Plugin for TowerPlacingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TowerPreview>()
            .insert_resource(SelectedTower(Tower::new(TowerType::Wall, Orientation::Up)))
            .add_systems(OnEnter(TowerPlacing), spawn_preview)
            .add_systems(OnExit(TowerPlacing), despawn_preview)
            .add_systems(
                Update,
                (
                    place_tower.run_if(input_just_pressed(MouseButton::Left)),
                    change_rotation.run_if(input_just_pressed(KeyCode::KeyR)),
                    update_preview,
                    exit_tower_place_state.run_if(input_just_pressed(KeyCode::KeyQ)),
                )
                    .run_if(in_state(TowerPlacing)),
            );
    }
}

fn exit_tower_place_state(
    current_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    set_tower_placing_state(&current_state, &mut next_state);
}

#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct SelectedTower(pub Tower);

impl Deref for SelectedTower {
    type Target = Tower;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for SelectedTower {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Reflect, Component)]
#[reflect(Component)]
struct TowerPreview;

pub fn place_tower(
    mut commands: Commands,
    mut event_writer: EventWriter<PathChangedEvent>,
    window: Single<&Window, With<PrimaryWindow>>,
    cam: Single<(&Camera, &GlobalTransform)>,
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut grid: ResMut<Grid>,
    tower: Res<SelectedTower>,
) {
    let mouse_pos = window.cursor_position();

    if let Some(mouse_pos) = mouse_pos {
        let (camera, cam_transform) = *cam;

        let world_pos = camera.viewport_to_world_2d(cam_transform, mouse_pos);
        if let Ok(world_pos) = world_pos {
            if let Some(grid_pos) = world_to_grid_coords(world_pos) {
                let grid_pos = apply_offset(grid_pos, tower.0.variant, tower.0.orientation);

                let tower_size = tower.size();

                // Check if tiles are free
                for i in 0..tower_size.0 {
                    for j in 0..tower_size.1 {
                        let pos = GridPos {
                            col: grid_pos.col + i,
                            row: grid_pos.row + j,
                        };
                        if !grid.is_free(&pos) {
                            return;
                        }

                        if pos.col > COLUMNS - 1 || pos.col < 0 || pos.row > ROWS - 1 || pos.row < 0
                        {
                            return;
                        }
                    }
                }

                let entity = commands
                    .spawn((
                        Health(tower.max_hp()),
                        tower.0.clone(),
                        Sprite {
                            color: Color::srgb(0.0, 0.5, 1.0),
                            custom_size: Some(Vec2 {
                                x: tower_size.0 as f32 * TILE_SIZE,
                                y: tower_size.1 as f32 * TILE_SIZE,
                            }),
                            anchor: bevy::sprite::Anchor::BottomLeft,
                            ..default()
                        },
                        Transform {
                            translation: (grid_to_world_coords(grid_pos) - (TILE_SIZE * 0.5))
                                .extend(1.0),
                            ..default()
                        },
                    ))
                    .id();

                event_writer.send(PathChangedEvent::now_blocked(
                    tower.fill_grid(&grid_pos, &mut grid, entity),
                ));

                if !input.pressed(KeyCode::ShiftLeft) {
                    set_tower_placing_state(&current_state, &mut next_state);
                }
            }
        } else {
            warn!("Unable to get Cursor Position {:?}", world_pos.unwrap_err())
        }
    }
}

fn change_rotation(mut selection: ResMut<SelectedTower>) {
    selection.orientation = match selection.orientation {
        Orientation::Up => Orientation::Right,
        Orientation::Right => Orientation::Down,
        Orientation::Down => Orientation::Left,
        Orientation::Left => Orientation::Up,
    };
}

fn spawn_preview(mut commands: Commands) {
    commands.spawn((
        TowerPreview,
        Sprite {
            color: Color::srgb(0.0, 0.5, 1.0),
            anchor: bevy::sprite::Anchor::BottomLeft,
            ..default()
        },
        Visibility::Hidden,
    ));
}

fn despawn_preview(mut commands: Commands, preview: Query<Entity, With<TowerPreview>>) {
    if let Ok(preview) = preview.get_single() {
        commands.entity(preview).despawn();
    }
}

fn update_preview(
    window: Single<&Window, With<PrimaryWindow>>,
    cam: Single<(&Camera, &GlobalTransform)>,
    grid: Res<Grid>,
    tower: Res<SelectedTower>,
    mut preview: Query<(&mut Sprite, &mut Transform, &mut Visibility), With<TowerPreview>>,
) {
    let (mut sprite, mut transform, mut visibility) = preview.single_mut();

    let mouse_pos = window.cursor_position();

    if let Some(mouse_pos) = mouse_pos {
        let (camera, cam_transform) = *cam;

        let world_pos = camera.viewport_to_world_2d(cam_transform, mouse_pos);
        if let Ok(world_pos) = world_pos {
            if let Some(grid_pos) = world_to_grid_coords(world_pos) {
                let grid_pos = apply_offset(grid_pos, tower.0.variant, tower.orientation);

                let tower_size = tower.size();

                sprite.color = Color::srgb(0.0, 0.5, 1.0);

                // Check if tiles are free
                for i in 0..tower_size.0 {
                    for j in 0..tower_size.1 {
                        let pos = GridPos {
                            col: grid_pos.col + i,
                            row: grid_pos.row + j,
                        };
                        if !grid.is_free(&pos) {
                            sprite.color = Color::srgb(1.0, 0.0, 0.0);
                        }

                        if pos.col > COLUMNS - 1 || pos.col < 0 || pos.row > ROWS - 1 || pos.row < 0
                        {
                            sprite.color = Color::srgb(1.0, 0.0, 0.0);
                        }
                    }
                }

                sprite.custom_size = Some(Vec2 {
                    x: tower_size.0 as f32 * TILE_SIZE,
                    y: tower_size.1 as f32 * TILE_SIZE,
                });

                transform.translation =
                    (grid_to_world_coords(grid_pos) - (TILE_SIZE * 0.5)).extend(2.0);

                *visibility = Visibility::Inherited;
            } else {
                *visibility = Visibility::Hidden;
            }
        } else {
            warn!("Unable to get Cursor Position {:?}", world_pos.unwrap_err())
        }
    }
}

fn apply_offset(grid_pos: GridPos, tower: TowerType, orientation: Orientation) -> GridPos {
    match orientation {
        Orientation::Up => GridPos {
            col: grid_pos.col - tower.offset().0,
            row: grid_pos.row - tower.offset().1,
        },
        Orientation::Down => GridPos {
            col: grid_pos.col - (tower.size().0 - 1 - tower.offset().0),
            row: grid_pos.row - (tower.size().1 - 1 - tower.offset().1),
        },
        Orientation::Left => GridPos {
            col: grid_pos.col - tower.offset().1,
            row: grid_pos.row - tower.offset().0,
        },
        Orientation::Right => GridPos {
            col: grid_pos.col - (tower.size().1 - 1 - tower.offset().1),
            row: grid_pos.row - (tower.size().0 - 1 - tower.offset().0),
        },
    }
}
