//

use grid::Grid;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub usize, pub usize);

impl Pos {
    pub fn from_grid_position<T>(grid: &Grid<T>, index: usize) -> Self {
        let grid_cols = grid.cols();
        Pos(index / grid_cols, index % grid_cols)
    }

    pub fn elevation(&self, grid: &Grid<char>) -> Option<u8> {
        let Self(row, col) = *self;
        grid.get(row, col).map(|c| char_to_elevation(*c))
    }

    pub fn distance(&self, other: &Pos) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }
}

fn char_to_elevation(elevation_char: char) -> u8 {
    const TO_DIGIT_A: u8 = 10; // 'a'.to_digit(36).unwrap()
    const TO_DIGIT_Z: u8 = 35; // 'z'.to_digit(36).unwrap()

    let digit = match elevation_char {
        'E' => TO_DIGIT_Z,
        'S' => TO_DIGIT_A,
        _ => elevation_char.to_digit(36).unwrap() as u8,
    };

    digit - TO_DIGIT_A
}

#[cfg(test)]
mod test_char_to_elevation {
    use super::*;

    #[test]
    fn test_a_to_digit() {
        assert_eq!('a'.to_digit(36).unwrap(), 10);
        assert_eq!('z'.to_digit(36).unwrap(), 35);
    }

    #[test]
    fn test_a_0() {
        assert_eq!(char_to_elevation('a'), 0);
    }

    #[test]
    fn test_uppercase_s_is_a_0() {
        assert_eq!(char_to_elevation('S'), 0);
    }

    #[test]
    fn test_uppercase_e_is_z_0() {
        assert_eq!(char_to_elevation('E'), 25);
    }
}
