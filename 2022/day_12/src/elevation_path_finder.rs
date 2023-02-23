//

use grid::Grid;
use pathfinding::prelude::astar;

use crate::position::Pos;
pub struct ElevationPathFinder {
    elevation_grid: Grid<char>,
}

//

impl ElevationPathFinder {
    pub fn new(height_map: Grid<char>) -> Self {
        Self {
            elevation_grid: height_map,
        }
    }

    pub fn shortest(&self, start: Pos, end: Pos) -> Option<Vec<Pos>> {
        let is_in_grid = |Pos(x, y): &&Pos| self.elevation_grid.get(*x, *y).is_some();
        let is_in_elevation_range = |current: Pos| {
            move |p: &&Pos| {
                let e = current.elevation(&self.elevation_grid);
                let range = 0..=e + 1;
                range.contains(&p.elevation(&self.elevation_grid))
            }
        };

        let successors = |position: &Pos| -> Vec<(Pos, u32)> {
            let Pos(row, col) = position.to_owned();

            let possibilities = [
                Pos(row, col + 1),
                Pos(row + 1, col),
                Pos(row, col.saturating_sub(1)),
                Pos(row.saturating_sub(1), col),
            ];
            possibilities
                .iter()
                .filter(is_in_grid)
                .filter(is_in_elevation_range(position.to_owned()))
                .map(|p: &Pos| (p.to_owned(), 1))
                .collect()
        };

        let result = astar(
            &start,
            |p| successors(p),
            |p| p.distance(&end),
            |p| *p == end,
        );

        if let Some((shortest_path, _)) = result {
            return Some(shortest_path);
        } else {
            return None;
        }
    }
}

//

#[cfg(test)]
mod test_shortest {
    use super::*;

    #[test]
    fn test_shortest_same_start_end() {
        let grid = Grid::from_vec(vec!['a'; 9], 3);
        let pathfinder = ElevationPathFinder::new(grid);
        let path = pathfinder.shortest(Pos(1, 1), Pos(1, 1)).unwrap();
        assert_eq!(path, vec![Pos(1, 1)]);
    }

    #[test]
    fn test_shortest_zero_to_center() {
        let grid = Grid::from_vec(vec!['a'; 9], 3);
        let pathfinder = ElevationPathFinder::new(grid);
        let path = pathfinder.shortest(Pos(0, 0), Pos(1, 1)).unwrap();
        assert_eq!(path, vec![Pos(0, 0), Pos(0, 1), Pos(1, 1)]);
    }

    #[test]
    fn test_shortest_zero_to_bottom_right() {
        let range = std::iter::empty()
            .chain('a'..='c')
            .chain(('d'..='f').rev())
            .chain('g'..='i')
            .map(|c| c)
            .collect();
        let grid = Grid::from_vec(range, 3);

        let pathfinder = ElevationPathFinder::new(grid);
        let path = pathfinder.shortest(Pos(0, 0), Pos(2, 2)).unwrap();
        assert_eq!(
            path,
            vec![
                Pos(0, 0),
                Pos(0, 1),
                Pos(0, 2),
                Pos(1, 2),
                Pos(1, 1),
                Pos(1, 0),
                Pos(2, 0),
                Pos(2, 1),
                Pos(2, 2)
            ]
        );
    }
}
