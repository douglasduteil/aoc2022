//

use std::slice::Iter;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn iter() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Right,
        ];
        DIRECTIONS.iter()
    }
}
