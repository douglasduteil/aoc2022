//

mod elevation_path_finder;
mod input;
mod position;

//

use crate::{elevation_path_finder::ElevationPathFinder, input::Input, position::Pos};

//

pub fn part_one(input: &str) -> Option<usize> {
    let Input {
        height_map,
        start,
        end,
    } = input.parse().expect("Heightmap parsing error");

    let pathfinder = ElevationPathFinder::new(height_map);

    pathfinder.shortest(start, end).map(|p| p.len() - 1)
}

pub fn part_two(input: &str) -> Option<usize> {
    let Input {
        height_map, end, ..
    } = input.parse().expect("Heightmap parsing error");

    let starting_points = height_map
        .iter()
        .enumerate()
        .filter_map(|(index, elevation)| match elevation {
            'S' | 'a' => Some(Pos::from_grid_position(&height_map, index)),
            _ => None,
        });

    let pathfinder = ElevationPathFinder::new(height_map.clone());

    starting_points
        .flat_map(|start| pathfinder.shortest(start, end.clone()))
        .min_by(|a, b| a.len().cmp(&b.len()))
        .map(|p| p.len() - 1)
}
