//

mod elevation_path_finder;
mod input;
mod position;

//

use crate::{elevation_path_finder::ElevationPathFinder, input::Input};

//

pub fn part_one(input: &str) -> Option<usize> {
    let Input {
        height_map,
        start,
        end,
    } = input.parse().expect("Heightmap parsing error");

    let pathfinder = ElevationPathFinder::new(height_map);

    let path = pathfinder.shortest(start, end);

    Some(path.len() - 1)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}
