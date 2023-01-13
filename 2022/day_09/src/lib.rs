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
    let mut rope = Rope::default();
    let series_of_motions: Vec<Motion> = input.lines().flat_map(Motion::from_line).collect();

    let tail_positions =
        series_of_motions
            .iter()
            .fold(HashSet::new(), |mut tail_positions, motion| {
                rope = rope
                    .step_iter(motion)
                    .map(|next_rope| {
                        let Rope(_, tail) = next_rope;
                        tail_positions.insert(tail);
                        next_rope
                    })
                    .last()
                    .unwrap_or_default();

                tail_positions
            });
    Some(tail_positions.len() as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}
