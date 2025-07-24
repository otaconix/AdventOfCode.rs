use aoc_utils::EnumVariants;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Write;
use std::iter::successors;

use aoc_macros::EnumVariants;

#[derive(Clone, Copy, PartialEq, Eq, Hash, EnumVariants)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(char::from(*self))
    }
}

impl Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(char::from(*self))
    }
}

type Coord = (usize, usize);

impl Direction {
    /// Determine the direction to go from `from` to `to`.
    ///
    /// Returns `None` if:
    ///   - `from == to`
    ///   - `from` and `to` aren't on either the same horizontal or vertical plane (`from.x != to.x
    ///   && from.y != to.y`)
    #[must_use] pub fn determine(from: &Coord, to: &Coord) -> Option<Self> {
        use std::cmp::Ordering::{Greater, Equal, Less};
        match (to.0.cmp(&from.0), to.1.cmp(&from.1)) {
            (Greater, Equal) => Self::Right.into(),
            (Less, Equal) => Self::Left.into(),
            (Equal, Greater) => Self::Down.into(),
            (Equal, Less) => Self::Up.into(),
            _ => None, // Coordinates are equal or not on the same horizontal/vertical axis
        }
    }

    #[must_use] pub fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    #[must_use] pub fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    #[must_use] pub fn advance(&self, coord: &Coord, steps: usize) -> Option<Coord> {
        match self {
            Direction::Up => coord.1.checked_sub(steps).map(|y| (coord.0, y)),
            Direction::Down => Some((coord.0, coord.1 + steps)),
            Direction::Left => coord.0.checked_sub(steps).map(|x| (x, coord.1)),
            Direction::Right => Some((coord.0 + steps, coord.1)),
        }
    }

    #[must_use] pub fn advance_with_intermediate_coords(
        &self,
        coord: &Coord,
        steps: usize,
    ) -> Option<Vec<Coord>> {
        let result = successors(self.advance(coord, 1), |next| self.advance(next, 1))
            .take(steps)
            .collect::<Vec<_>>();

        if result.len() == steps {
            Some(result)
        } else {
            None
        }
    }
}

impl From<Direction> for char {
    fn from(val: Direction) -> Self {
        match val {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}
