use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

use crate::{Enemy, EnemyPath, tile::Tile};

fn try_get_target(tiles: &HashSet<&Tile>, enemy: &Enemy) -> Option<HashMap<Tile, Tile>> {
    let distance = enemy.current.distance_to(&enemy.goal);
    // This is the A* algorithm, see https://www.youtube.com/watch?v=-L-WgKMFuhE

    // open contains f_cost, g_cost and parent of every tile
    let mut open: HashMap<Tile, (usize, usize, Tile)> =
        HashMap::from([(enemy.current, (distance, 0, enemy.current))]);
    let mut closed: HashMap<Tile, Tile> = HashMap::new();

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
            if !open
                .get(&neighbor)
                .is_some_and(|(_, nb_g_cost, _)| new_nb_g_cost >= *nb_g_cost)
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
    let tiles: HashSet<&Tile> = tiles.iter().collect();
    let get_path = |closed: HashMap<Tile, Tile>, enemy: &Enemy| {
        let mut path = vec![];
        let mut current = enemy.goal;
        while current != enemy.current {
            path.push(current);
            current = closed[&current];
        }
        path
    };
    for (enemy, entity) in &enemies {
        if let Some(closed) = try_get_target(&tiles, enemy) {
            let path = get_path(closed, enemy);
            if !path.is_empty() {
                commands.entity(entity).insert(EnemyPath(path));
                return;
            }
        } else {
            println!("No path was found! Despawning!");
        }
        commands.entity(entity).despawn();
    }
}
