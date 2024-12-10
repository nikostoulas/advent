#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Right,
    RightDown,
    Down,
    DownLeft,
    Left,
    LeftUp,
    Up,
    UpRight,
}

use Direction::{Down, DownLeft, Left, LeftUp, Right, RightDown, Up, UpRight};

impl Direction {
    pub const VALUES_8: [Self; 8] = [Right, RightDown, Down, DownLeft, Left, LeftUp, Up, UpRight];
    pub const VALUES_4: [Self; 4] = [Right, Down, Left, Up];

    pub fn opposite(&self) -> Self {
        match self {
            Right => Left,
            RightDown => LeftUp,
            Down => Up,
            DownLeft => UpRight,
            Left => Right,
            LeftUp => RightDown,
            Up => Down,
            UpRight => RightDown,
        }
    }

    pub fn next_4(&self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            _ => panic!("wrong direction"),
        }
    }
}
