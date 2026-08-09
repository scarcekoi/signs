/// A cardinal direction.
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

/// A location on the sign.
pub enum Location {
    Direction(Direction),
    Center,
}

/// An arrow on the sign.
pub struct Arrow {
    pub direction: Direction,
    pub location: Location,
}
