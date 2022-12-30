//

pub fn part_one(input: &str) -> Option<u32> {
    use std::collections::HashSet;

    input
        .lines()
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2))
        .flat_map(|(left, right)| {
            let left_set: HashSet<char> = left.chars().collect();
            let right_set: HashSet<char> = right.chars().collect();
            let common_itmes: Vec<char> = left_set.intersection(&right_set).cloned().collect();
            common_itmes
        })
        .map(|char| char_to_priority(char))
        .map(Some)
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    use itertools::Itertools;

    input
        .lines()
        .tuples::<(_, _, _)>()
        .flat_map(|(rucksack_1, rucksack_2, rucksack_3)| {
            rucksack_1
                .chars()
                .find(|char| rucksack_2.contains(*char) && rucksack_3.contains(*char))
        })
        .map(|char| char_to_priority(char))
        .map(Some)
        .sum()
}

//

fn char_to_priority(char: char) -> u32 {
    const TO_DIGIT_9: u32 = 9; // '9'.to_digit(36).unwrap()
    const TO_DIGIT_Z: u32 = 35; // 'z'.to_digit(36).unwrap()
    const TO_DIGIT_UPPER_A: u32 = TO_DIGIT_Z - TO_DIGIT_9;

    let start_priority = if char.is_lowercase() {
        0
    } else {
        TO_DIGIT_UPPER_A
    };
    char.to_digit(36).unwrap() - TO_DIGIT_9 + start_priority
}
