//

use std::{iter, num::ParseIntError, ops::RangeInclusive, str::FromStr};

use crate::position::Position;

//

#[derive(Debug, PartialEq, Clone)]
pub enum Motion {
    Up(RangeInclusive<usize>),
    Down(RangeInclusive<usize>),
    Left(RangeInclusive<usize>),
    Right(RangeInclusive<usize>),
}

//

impl Motion {
    pub fn from_line(s: &str) -> Result<Motion, ParseIntError> {
        Self::from_str(s)
    }

    pub fn as_vectors(&self) -> impl Iterator<Item = Position> {
        use Motion::*;
        let range = match self {
            Up(steps) | Down(steps) | Right(steps) | Left(steps) => steps.to_owned(),
        };

        let vector = match self {
            Up(_) => Position(0, 1),
            Down(_) => Position(0, -1),
            Right(_) => Position(1, 0),
            Left(_) => Position(-1, 0),
        };

        iter::repeat(vector).take(range.count())
    }
}

impl FromStr for Motion {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [direction, steps] = s.split(' ')
            .collect::<Vec<&str>>()[..] else { unreachable!() };
        let steps: usize = steps.parse()?;
        let motion = match direction {
            "D" => Self::Down(1..=steps),
            "L" => Self::Left(1..=steps),
            "R" => Self::Right(1..=steps),
            "U" => Self::Up(1..=steps),
            _ => unreachable!(),
        };
        Ok(motion)
    }
}

#[cfg(test)]
mod test_motion_as_vectors {
    use super::*;

    #[test]
    fn test_repeat_up_positions() {
        let iter = Motion::Up(1..=2).as_vectors();
        assert_eq!(iter.collect::<Vec<_>>(), &[Position(0, 1), Position(0, 1)])
    }

    #[test]
    fn test_repeat_down_positions() {
        let iter = Motion::Down(1..=2).as_vectors();
        assert_eq!(
            iter.collect::<Vec<_>>(),
            &[Position(0, -1), Position(0, -1)]
        )
    }

    #[test]
    fn test_repeat_left_positions() {
        let iter = Motion::Left(1..=2).as_vectors();
        assert_eq!(
            iter.collect::<Vec<_>>(),
            &[Position(-1, 0), Position(-1, 0)]
        )
    }

    #[test]
    fn test_repeat_right_positions() {
        let iter = Motion::Right(1..=2).as_vectors();
        assert_eq!(iter.collect::<Vec<_>>(), &[Position(1, 0), Position(1, 0)])
    }
}

#[cfg(test)]
mod test_motion {
    use super::*;

    #[test]
    fn test_parse_right_4() {
        assert_eq!("R 4".parse::<Motion>().unwrap(), Motion::Right(1..=4))
    }

    #[test]
    fn test_parse_up_4() {
        assert_eq!("U 4".parse::<Motion>().unwrap(), Motion::Up(1..=4))
    }

    #[test]
    fn test_parse_left_4() {
        assert_eq!("L 3".parse::<Motion>().unwrap(), Motion::Left(1..=3))
    }

    #[test]
    fn test_parse_down_1() {
        assert_eq!("D 1".parse::<Motion>().unwrap(), Motion::Down(1..=1))
    }
}
