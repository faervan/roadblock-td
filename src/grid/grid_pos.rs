use std::{
    fmt::{Debug, Display},
    ops::Add,
};

use bevy::{prelude::*, utils::HashMap};
use fastrand::Rng;

use super::{COLUMNS, ROWS};

#[derive(Reflect, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct GridPos {
    pub row: isize,
    pub col: isize,
}

impl Add<GridPos> for GridPos {
    type Output = GridPos;
    fn add(self, rhs: GridPos) -> Self::Output {
        GridPos {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}
impl Add<&GridPos> for GridPos {
    type Output = GridPos;
    fn add(self, rhs: &GridPos) -> Self::Output {
        GridPos {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}
impl Add<GridPos> for &GridPos {
    type Output = GridPos;
    fn add(self, rhs: GridPos) -> Self::Output {
        GridPos {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}
impl Add<&GridPos> for &GridPos {
    type Output = GridPos;
    fn add(self, rhs: &GridPos) -> Self::Output {
        GridPos {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}
impl Add<[isize; 2]> for GridPos {
    type Output = GridPos;
    fn add(self, rhs: [isize; 2]) -> Self::Output {
        GridPos {
            row: self.row + rhs[0],
            col: self.col + rhs[1],
        }
    }
}
impl Add<GridPos> for [isize; 2] {
    type Output = GridPos;
    fn add(self, rhs: GridPos) -> Self::Output {
        GridPos {
            row: rhs.row + self[0],
            col: rhs.col + self[1],
        }
    }
}
impl Add<[isize; 2]> for &GridPos {
    type Output = GridPos;
    fn add(self, rhs: [isize; 2]) -> Self::Output {
        GridPos {
            row: self.row + rhs[0],
            col: self.col + rhs[1],
        }
    }
}
impl Add<&GridPos> for [isize; 2] {
    type Output = GridPos;
    fn add(self, rhs: &GridPos) -> Self::Output {
        GridPos {
            row: rhs.row + self[0],
            col: rhs.col + self[1],
        }
    }
}

impl GridPos {
    pub fn new(row: isize, col: isize) -> Self {
        GridPos { row, col }
    }

    pub fn random(rng: &mut Rng) -> Self {
        Self {
            row: rng.isize(0..(ROWS - 1)),
            col: rng.isize(0..(COLUMNS - 1)),
        }
    }

    pub fn inside_grid_bounds(&self) -> bool {
        (0..ROWS).contains(&self.row) && (0..COLUMNS).contains(&self.col)
    }

    pub fn distance_to(&self, other: &GridPos) -> usize {
        other.row.abs_diff(self.row) + other.col.abs_diff(self.col)
    }

    pub fn closest<'a>(&self, goals: &'a HashMap<GridPos, Entity>) -> &'a GridPos {
        goals
            .keys()
            .map(|pos| (pos, self.distance_to(pos)))
            .min_by_key(|x| x.1)
            .expect("No goals exist anymore!")
            .0
    }

    pub fn distance_to_closest(&self, goals: &HashMap<GridPos, Entity>) -> usize {
        goals
            .keys()
            .map(|pos| self.distance_to(pos))
            .min()
            .expect("No goals exist anymore!")
    }

    /// * `towers` - Every tower position mapped to its Entity and travel cost
    pub fn neighbors<'a>(
        &'a self,
        towers: &'a HashMap<GridPos, (Entity, usize)>,
        default_travel_cost: usize,
    ) -> Vec<(GridPos, Option<&'a Entity>, usize)> {
        let mut neighbors = vec![];

        let mut push_maybe = |row, col| {
            let tile = GridPos::new(row, col);
            if tile.inside_grid_bounds() {
                match towers.get(&tile) {
                    Some((entity, travel_cost)) => {
                        neighbors.push((tile, Some(entity), *travel_cost))
                    }
                    None => neighbors.push((tile, None, default_travel_cost)),
                }
            }
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
        write!(f, "GridPos ({}, {})", self.row, self.col)
    }
}

impl Debug for GridPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
