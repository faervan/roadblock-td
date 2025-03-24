use std::fmt::{Debug, Display};

use bevy::{prelude::Component, utils::HashSet};

#[derive(Component, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Tile {
    row: isize,
    col: isize,
}

impl Tile {
    pub fn new(row: isize, col: isize) -> Self {
        Tile { row, col }
    }
    pub fn distance_to(&self, other: &Tile) -> usize {
        other.row.abs_diff(self.row) + other.col.abs_diff(self.col)
    }
    pub fn neighbors(&self, tiles: &HashSet<&Tile>) -> Vec<Tile> {
        let mut neighbors = vec![];

        let mut push_maybe = |row, col| {
            let tile = Tile::new(row, col);
            tiles.contains(&tile).then(|| neighbors.push(tile));
        };

        push_maybe(self.row + 1, self.col);
        push_maybe(self.row - 1, self.col);
        push_maybe(self.row, self.col + 1);
        push_maybe(self.row, self.col - 1);

        neighbors
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tile ({}, {})", self.row, self.col)
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
