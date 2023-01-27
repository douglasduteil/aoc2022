//

use std::{num::ParseIntError, ops::RangeInclusive, str::FromStr};

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
