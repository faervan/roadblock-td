use std::{
    fmt::{Debug, Display},
    ops::Add,
};

use bevy::{prelude::*, utils::HashMap};

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

impl GridPos {
    pub fn new(row: isize, col: isize) -> Self {
        GridPos { row, col }
    }

    pub fn distance_to(&self, other: &GridPos) -> usize {
        other.row.abs_diff(self.row) + other.col.abs_diff(self.col)
    }

    pub fn neighbors<'a>(
        &'a self,
        blocked: &'a HashMap<GridPos, Entity>,
    ) -> Vec<(GridPos, Option<&'a Entity>)> {
        let mut neighbors = vec![];

        let mut push_maybe = |row, col| {
            if (0..ROWS).contains(&row) && (0..COLUMNS).contains(&col) {
                let tile = GridPos::new(row, col);
                match blocked.get(&tile) {
                    Some(entity) => neighbors.push((tile, Some(entity))),
                    None => neighbors.push((tile, None)),
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
