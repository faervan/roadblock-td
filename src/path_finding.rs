use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

use crate::{
    enemy::{Enemy, EnemyPath},
    grid::{GridPos, Tile},
};

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enemy_get_path);
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

pub fn enemy_get_path(
    mut commands: Commands,
    enemies: Query<(&Enemy, Entity), Without<EnemyPath>>,
    tiles: Query<&Tile>,
) {
    let tile_set: HashSet<&GridPos> = tiles.iter().map(|t| &t.pos).collect();

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
        if let Some(closed) = try_get_target(&tile_set, enemy) {
            let path = get_path(closed, enemy);
            if !path.is_empty() {
                commands.entity(entity).insert(EnemyPath(path));
                return;
            }
        } else {
            debug!("No path was found! Despawning!");
        }
        commands.entity(entity).despawn();
    }
}
