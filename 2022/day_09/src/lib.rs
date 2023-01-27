//

mod motion;
mod position;
mod rope;

//

use std::collections::HashSet;

use motion::Motion;
use rope::Rope;

//

pub fn part_one(input: &str) -> Option<u32> {
    let mut rope = Rope::<2>::default();
    let mut tail_positions = HashSet::new();

    let series_of_motions: Vec<Motion> = input.lines().flat_map(Motion::from_line).collect();

    for motion in series_of_motions {
        for vector in motion.as_vectors() {
            rope.move_head_by(vector);
            tail_positions.insert(rope.tail().clone());
        }
    }

    Some(tail_positions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rope = Rope::<10>::default();
    let mut tail_positions = HashSet::new();

    let series_of_motions: Vec<Motion> = input.lines().flat_map(Motion::from_line).collect();

    for motion in series_of_motions {
        for vector in motion.as_vectors() {
            rope.move_head_by(vector);
            tail_positions.insert(rope.tail().clone());
        }
    }

    Some(tail_positions.len() as u32)
}
