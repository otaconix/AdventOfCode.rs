#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate2D {
    pub x: i64,
    pub y: i64,
}

impl Coordinate2D {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn manhattan_distance(&self, other: &Self) -> u64 {
        let x_distance = self.x.abs_diff(other.x);
        let y_distance = self.y.abs_diff(other.y);

        x_distance + y_distance
    }
}
