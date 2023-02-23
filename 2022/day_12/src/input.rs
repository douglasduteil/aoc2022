//

use grid::Grid;
use std::{convert::Infallible, str::FromStr};

use crate::position::Pos;

pub struct Input {
    pub height_map: Grid<char>,
    pub start: Pos,
    pub end: Pos,
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();

        let heightmap = Grid::from_vec(
            lines
                .iter()
                .flat_map(|line| line.chars().collect::<Vec<_>>())
                .collect(),
            lines[0].len(),
        );

        let start = heightmap
            .iter()
            .position(|elevation| elevation == &'S')
            .map(|index| Pos::from_grid_position(&heightmap, index))
            .expect("Can't find start position");

        let end = heightmap
            .iter()
            .position(|elevation| elevation == &'E')
            .map(|index| Pos::from_grid_position(&heightmap, index))
            .expect("Can't find end position");

        Ok(Input {
            height_map: heightmap,
            start,
            end,
        })
    }
}
