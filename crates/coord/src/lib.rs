#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate2D {
    pub x: i64,
    pub y: i64,
}

impl Coordinate2D {
    #[must_use] pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    /// Return the manhattan distance from `self` to `other`.
    ///
    /// The manhattan distance is the sum of the `x` and `y` distances.
    #[must_use] pub fn manhattan_distance(&self, other: &Self) -> u64 {
        let x_distance = self.x.abs_diff(other.x);
        let y_distance = self.y.abs_diff(other.y);

        x_distance + y_distance
    }

    /// Translate by `other`.
    #[must_use] pub fn translate(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
