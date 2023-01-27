//

mod motion;
mod position;
mod rope;

//

use std::collections::HashSet;

use motion::Motion;
use position::Position;
use rope::Rope;

//

pub fn part_one(input: &str) -> Option<u32> {
    let mut rope = Rope::new(2, Position::default());

    let series_of_motions: Vec<Motion> = input.lines().flat_map(Motion::from_line).collect();

    let tail_positions =
        series_of_motions
            .iter()
            .fold(HashSet::new(), |mut tail_positions, motion| {
                for _rope in rope.apply_motion_iter(motion) {
                    let Rope { knots, .. } = _rope.to_owned();
                    let tail = knots.back().unwrap().clone();
                    tail_positions.insert(tail);

                    rope = _rope;
                }

                tail_positions
            });
    Some(tail_positions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rope = Rope::new(10, Position::default());

    let series_of_motions: Vec<Motion> = input.lines().flat_map(Motion::from_line).collect();

    let tail_positions =
        series_of_motions
            .iter()
            .fold(HashSet::new(), |mut tail_positions, motion| {
                for _rope in rope.apply_motion_iter(motion) {
                    let Rope { knots, .. } = _rope.to_owned();
                    let tail = knots.back().unwrap().clone();
                    tail_positions.insert(tail);

                    rope = _rope;
                }

                tail_positions
            });
    Some(tail_positions.len() as u32)
}
