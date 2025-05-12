use bevy::{prelude::*, utils::HashMap};

use crate::{
    Orientation,
    animation::AnimationConfig,
    app_state::GameState,
    enemy::Enemy,
    grid::{Grid, GridPos, TILE_SIZE, grid_to_world_coords},
    health::Health,
    tower::{Tower, place_tower, projectile_damage},
};

use super::attack::{Attacking, AttackingGoal};

pub struct EnemyMovementPlugin;

impl Plugin for EnemyMovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EnemyPath>()
            .add_event::<PathChangedEvent>()
            .add_systems(
                Update,
                (
                    check_for_broken_paths
                        .run_if(on_event::<PathChangedEvent>)
                        .after(place_tower),
                    enemy_get_path.after(check_for_broken_paths),
                    move_enemies.before(projectile_damage),
                )
                    .run_if(in_state(GameState::Running)),
            );
    }
}

#[derive(Reflect, Component, Debug)]
#[reflect(Component)]
pub struct EnemyPath {
    pub steps: Vec<GridPos>,
    next: Option<Vec3>,
}

impl EnemyPath {
    pub fn new(steps: Vec<GridPos>) -> Self {
        Self { steps, next: None }
    }
}

#[derive(Event)]
pub struct PathChangedEvent {
    changed: Vec<GridPos>,
    /// If true, the positions from self.changed are now empty. Else they are now blocked.
    now_free: bool,
}

impl PathChangedEvent {
    pub fn now_free(freed: Vec<GridPos>) -> Self {
        Self {
            changed: freed,
            now_free: true,
        }
    }
    pub fn now_blocked(blocked: Vec<GridPos>) -> Self {
        Self {
            changed: blocked,
            now_free: false,
        }
    }
}

/// `tiles` maps a `tower_entity` and a `travel_cost` to every `GridPos`
fn try_get_target(
    tiles: &HashMap<GridPos, (Entity, usize)>,
    enemy: &Enemy,
    goals: &HashMap<GridPos, Entity>,
    death_count: &HashMap<GridPos, f32>,
) -> Option<(HashMap<GridPos, GridPos>, GridPos)> {
    let distance = enemy.current.distance_to_closest(goals);
    let default_travel_cost = (enemy.velocity() * 2. / TILE_SIZE) as usize;

    // This is the A* algorithm, see https://www.youtube.com/watch?v=-L-WgKMFuhE

    // open contains f_cost, g_cost, parent and tower_entity of every tile
    let mut open: HashMap<GridPos, (usize, usize, GridPos, Option<Entity>)> =
        HashMap::from([(enemy.current, (distance, 0, enemy.current, None))]);
    let mut closed: HashMap<GridPos, GridPos> = HashMap::new();

    while let Some((tile, (_, g_cost, parent, tower_entity))) = open
        .iter()
        .min_by_key(|x| x.1.0)
        .map(|(tile, data)| (*tile, *data))
    {
        open.remove(&tile);
        closed.insert(tile, parent);

        if goals.contains_key(&tile) {
            return Some((closed, tile));
        }

        for (neighbor, nb_tower_entity, travel_cost) in
            tile.neighbors(tiles, default_travel_cost, death_count)
        {
            if closed.contains_key(&neighbor) {
                continue;
            }
            let new_nb_g_cost = g_cost
                + if tower_entity.is_some_and(|id| Some(&id) == nb_tower_entity) {
                    default_travel_cost
                } else {
                    travel_cost
                };
            if open
                .get(&neighbor)
                .is_none_or(|(_, nb_g_cost, _, _)| new_nb_g_cost < *nb_g_cost)
            {
                open.insert(
                    neighbor,
                    (
                        new_nb_g_cost + neighbor.distance_to_closest(goals),
                        new_nb_g_cost,
                        tile,
                        nb_tower_entity.copied(),
                    ),
                );
            }
        }
    }
    None
}

fn enemy_get_path(
    mut commands: Commands,
    enemies: Query<
        (&Enemy, Entity),
        (
            Without<EnemyPath>,
            Without<Attacking>,
            Without<AttackingGoal>,
        ),
    >,
    towers: Query<&Health, With<Tower>>,
    grid: Res<Grid>,
) {
    let get_path = |closed: HashMap<GridPos, GridPos>, enemy: &Enemy, goal: GridPos| {
        let mut path = vec![];
        let mut current = goal;
        while current != enemy.current {
            path.push(current);
            current = closed[&current];
        }
        path
    };
    for (enemy, entity) in &enemies {
        if let Some((closed, goal)) = try_get_target(
            &grid
                .towers
                .iter()
                .filter_map(|(pos, id)| {
                    towers
                        .get(*id)
                        .map(|hp| (*pos, (*id, enemy.travel_cost(**hp))))
                        .ok()
                })
                .collect(),
            enemy,
            &grid.enemy_goals,
            &grid.death_count,
        ) {
            let path = get_path(closed, enemy, goal);
            if !path.is_empty() {
                commands.entity(entity).insert(EnemyPath::new(path));
            }
        } else {
            unreachable!("No path was found! This shouldn't be possible.");
        }
    }
}

fn check_for_broken_paths(
    mut events: EventReader<PathChangedEvent>,
    mut commands: Commands,
    enemies: Query<(&EnemyPath, Entity), (With<Enemy>, Without<AttackingGoal>)>,
) {
    let mut freed_tiles: Vec<&GridPos> = vec![];
    let mut blocked_tiles: Vec<&GridPos> = vec![];
    for event in events.read() {
        match event.now_free {
            true => freed_tiles.extend(&event.changed),
            false => blocked_tiles.extend(&event.changed),
        }
    }
    // If a new path is available, every Enemy should check if it's more optimal for them
    if !freed_tiles.is_empty() {
        for (_, entity) in &enemies {
            commands
                .entity(entity)
                .remove::<EnemyPath>()
                .remove::<Attacking>();
        }
    }
    if !blocked_tiles.is_empty() {
        'outer: for (path, entity) in &enemies {
            if path
                .steps
                .last()
                .is_some_and(|tile| blocked_tiles.contains(&tile))
            {
                continue;
            }
            for tile in &blocked_tiles {
                if path.steps.contains(tile) {
                    commands.entity(entity).remove::<EnemyPath>();
                    continue 'outer;
                }
            }
        }
    }
}

pub fn move_enemies(
    mut query: Query<(
        &mut EnemyPath,
        &mut Enemy,
        &mut AnimationConfig,
        &mut Sprite,
        &mut Transform,
        Entity,
    )>,
    time: Res<Time>,
    grid: Res<Grid>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (mut path, mut enemy, mut animation, mut sprite, mut pos, entity) in &mut query {
        let next = match path.next {
            Some(target_pos) => target_pos,
            None => {
                let tile = path.steps.pop().unwrap();
                let orientation =
                    match (tile.row > enemy.current.row, tile.col > enemy.current.col) {
                        (true, false) => Orientation::Up,
                        (false, true) => Orientation::Right,
                        _ => match tile.row < enemy.current.row {
                            true => Orientation::Down,
                            false => Orientation::Left,
                        },
                    };

                if let Some(tower_entity) = grid.towers.get(&tile) {
                    if orientation != enemy.orientation {
                        enemy.orientation = orientation;
                    }
                    let weapon_id = commands
                        .spawn((
                            enemy.attack_animation_config(),
                            Sprite {
                                image: asset_server.load(enemy.weapon_sprites()),
                                texture_atlas: Some(
                                    enemy.attack_layout(&mut texture_atlas_layouts),
                                ),
                                ..Default::default()
                            },
                        ))
                        .set_parent(entity)
                        .id();

                    commands.entity(entity).remove::<EnemyPath>().insert((
                        Attacking::new(*tower_entity, weapon_id),
                        enemy.attack_animation_config(),
                        Sprite {
                            image: asset_server.load(enemy.attack_sprites()),
                            texture_atlas: Some(
                                enemy.attack_layout(&mut texture_atlas_layouts),
                            ),
                            ..Default::default()
                        },
                    ));
                    return;
                } else if grid.enemy_goals.contains_key(&tile) {
                    if orientation != enemy.orientation {
                        enemy.orientation = orientation;
                    }
                    commands
                        .entity(entity)
                        .remove::<EnemyPath>()
                        .insert((
                            AttackingGoal,
                            enemy.attack_animation_config(),
                            Sprite {
                                image: asset_server.load(enemy.attack_sprites()),
                                texture_atlas: Some(
                                    enemy.attack_layout(&mut texture_atlas_layouts),
                                ),
                                ..Default::default()
                            },
                        ))
                        .with_child((
                            enemy.attack_animation_config(),
                            Sprite {
                                image: asset_server.load(enemy.weapon_sprites()),
                                texture_atlas: Some(
                                    enemy.attack_layout(&mut texture_atlas_layouts),
                                ),
                                ..Default::default()
                            },
                        ));
                    return;
                }

                if orientation != enemy.orientation {
                    enemy.orientation = orientation;
                    *animation = enemy.walk_animation_config();
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.index = enemy.walk_sprite_indices().0;
                    }
                }
                enemy.current = tile;
                let next = grid_to_world_coords(tile).extend(2.) + enemy.offset();
                path.next = Some(next);

                next
            }
        };
        let direction = next - pos.translation;
        if direction.element_sum() != 0. {
            pos.translation +=
                direction.normalize() * time.delta_secs() * enemy.velocity();
        }
        if pos.translation.distance(next) >= direction.length() {
            path.next = None;
        }
    }
}
