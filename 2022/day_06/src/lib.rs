//

use std::collections::HashSet;

//

pub fn part_one(input: &str) -> Option<usize> {
    input
        .chars()
        .collect::<Vec<_>>()
        .as_slice()
        .windows(4)
        .position(|datastream| HashSet::<_>::from_iter(datastream).len() == 4)
        .and_then(|start| Some(start + 4))
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}
