#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate2D {
    pub x: i64,
    pub y: i64,
}

impl Coordinate2D {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}
