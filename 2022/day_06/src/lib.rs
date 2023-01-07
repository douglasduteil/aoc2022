//

use std::collections::HashSet;

//

pub fn part_one(input: &str) -> Option<usize> {
    const SEQUENCE_LEN: usize = 4;

    input
        .chars()
        .collect::<Vec<_>>()
        .as_slice()
        .windows(SEQUENCE_LEN)
        .position(|datastream| HashSet::<_>::from_iter(datastream).len() == datastream.len())
        .map(|start| start + SEQUENCE_LEN)
}

pub fn part_two(input: &str) -> Option<usize> {
    const SEQUENCE_LEN: usize = 14;

    input
        .chars()
        .collect::<Vec<_>>()
        .as_slice()
        .windows(SEQUENCE_LEN)
        .position(|datastream| HashSet::<_>::from_iter(datastream).len() == datastream.len())
        .map(|start| start + SEQUENCE_LEN)
}
