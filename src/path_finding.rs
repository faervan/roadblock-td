use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

use crate::{
    enemy::{Enemy, EnemyPath},
    grid::{Grid, GridPos},
    tower::place_tower,
};

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PathChangedEvent>().add_systems(
            Update,
            (
                check_for_broken_paths
                    .run_if(on_event::<PathChangedEvent>)
                    .after(place_tower),
                enemy_get_path.after(check_for_broken_paths),
            ),
        );
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

fn try_get_target(tiles: &HashSet<&GridPos>, enemy: &Enemy) -> Option<HashMap<GridPos, GridPos>> {
    let distance = enemy.current.distance_to(&enemy.goal);
    // This is the A* algorithm, see https://www.youtube.com/watch?v=-L-WgKMFuhE

    // open contains f_cost, g_cost and parent of every tile
    let mut open: HashMap<GridPos, (usize, usize, GridPos)> =
        HashMap::from([(enemy.current, (distance, 0, enemy.current))]);
    let mut closed: HashMap<GridPos, GridPos> = HashMap::new();

    while let Some((tile, (_, g_cost, parent))) = open
        .iter()
        .min_by(|x, y| x.1.0.cmp(&y.1.0))
        .map(|(tile, data)| (*tile, *data))
    {
        open.remove(&tile);
        closed.insert(tile, parent);

        if tile == enemy.goal {
            return Some(closed);
        }

        for neighbor in tile.neighbors(tiles) {
            if closed.contains_key(&neighbor) {
                continue;
            }
            let new_nb_g_cost = g_cost + 1;
            if open
                .get(&neighbor)
                .is_none_or(|(_, nb_g_cost, _)| new_nb_g_cost < *nb_g_cost)
            {
                open.insert(
                    neighbor,
                    (
                        new_nb_g_cost + neighbor.distance_to(&enemy.goal),
                        new_nb_g_cost,
                        tile,
                    ),
                );
            }
        }
    }
    None
}

fn enemy_get_path(
    mut commands: Commands,
    enemies: Query<(&Enemy, Entity), Without<EnemyPath>>,
    grid: Res<Grid>,
) {
    let get_path = |closed: HashMap<GridPos, GridPos>, enemy: &Enemy| {
        let mut path = vec![];
        let mut current = enemy.goal;
        while current != enemy.current {
            path.push(current);
            current = closed[&current];
        }
        path
    };
    for (enemy, entity) in &enemies {
        if let Some(closed) = try_get_target(&grid.blocked_tiles(), enemy) {
            let path = get_path(closed, enemy);
            if !path.is_empty() {
                commands.entity(entity).insert(EnemyPath::new(path));
                return;
            }
        } else {
            info!("No path was found! Despawning!");
            commands.entity(entity).despawn();
        }
    }
}

fn check_for_broken_paths(
    mut events: EventReader<PathChangedEvent>,
    mut commands: Commands,
    enemies: Query<(&EnemyPath, Entity), With<Enemy>>,
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
            commands.entity(entity).remove::<EnemyPath>();
        }
        return;
    }
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
