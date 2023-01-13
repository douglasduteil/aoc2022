//

#[derive(Debug, Eq, Hash, PartialEq, Default, Clone, Copy)]
pub struct Position(pub isize, pub isize);

impl Position {
    pub fn distance(&self, target: &Position) -> u8 {
        let dx = self.0 - target.0;
        let dy = self.1 - target.1;
        dx.abs().max(dy.abs()) as u8
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
mod test_position {
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
