//

use std::ops::{AddAssign, Sub};

#[derive(Debug, Eq, Hash, PartialEq, Default, Clone, Copy)]
pub struct Position(pub isize, pub isize);

impl Position {
    pub fn normalize(&self) -> Self {
        let &Position(x, y) = self;
        Position(x.clamp(-1, 1), y.clamp(-1, 1))
    }
    pub fn distance(&self, target: &Position) -> u8 {
        let dx = self.0 - target.0;
        let dy = self.1 - target.1;
        dx.abs().max(dy.abs()) as u8
    }
}

//

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}
//

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

//

impl From<(isize, isize)> for Position {
    fn from((x, y): (isize, isize)) -> Self {
        Position(x, y)
    }
}

//

#[cfg(test)]
mod test_add_assign {
    use super::*;

    #[test]
    fn test_zero() {
        let mut x = Position::default();
        x += Position::default();
        assert_eq!(x, Position::default());
    }

    #[test]
    fn test_one() {
        let mut x = Position::default();
        x += Position(1, 1);
        assert_eq!(x, Position(1, 1));
    }
}

#[cfg(test)]
mod test_distance {
    use super::*;

    #[test]
    fn test_distance_zero() {
        assert_eq!(Position::default().distance(&Position::default()), 0);
    }

    #[test]
    fn test_distance_one() {
        assert_eq!(Position::default().distance(&Position(0, 1)), 1);
        assert_eq!(Position::default().distance(&Position(1, 1)), 1);
        assert_eq!(Position::default().distance(&Position(1, 0)), 1);
        assert_eq!(Position::default().distance(&Position(1, -1)), 1);
        assert_eq!(Position::default().distance(&Position(0, -1)), 1);
        assert_eq!(Position::default().distance(&Position(-1, -1)), 1);

        assert_eq!(Position(2, 2).distance(&Position(1, 1)), 1);
        assert_eq!(Position(2, 1).distance(&Position(1, 1)), 1);
        assert_eq!(Position(2, 0).distance(&Position(1, 1)), 1);
        assert_eq!(Position(1, 2).distance(&Position(1, 1)), 1);
        assert_eq!(Position(1, 0).distance(&Position(1, 1)), 1);
        assert_eq!(Position(0, 2).distance(&Position(1, 1)), 1);
        assert_eq!(Position(0, 1).distance(&Position(1, 1)), 1);
        assert_eq!(Position(0, 0).distance(&Position(1, 1)), 1);

        assert_eq!(Position(1, 1).distance(&Position(2, 2)), 1);
    }

    #[test]
    fn test_distance_two() {
        assert_eq!(Position::default().distance(&Position(0, 2)), 2);
        assert_eq!(Position::default().distance(&Position(2, 2)), 2);
        assert_eq!(Position::default().distance(&Position(2, 0)), 2);
        assert_eq!(Position::default().distance(&Position(2, -2)), 2);
        assert_eq!(Position::default().distance(&Position(0, -2)), 2);
        assert_eq!(Position::default().distance(&Position(-2, -2)), 2);
    }
}
