//

pub fn part_one(input: &str) -> Option<u32> {
    // Find the item type that appears in both compartments of each rucksack.
    // What is the sum of the priorities of those item types?
    sum_of_the_priorities_of_item_type_that_appears_in_both_compartments_of_each_rucksack(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Find the item type that corresponds to the badges of each three-Elf group.
    // What is the sum of the priorities of those item types?
    sum_of_the_priorities_of_item_type_that_corresponds_to_the_badges_of_each_three_elf_group(input)
}

//
//
//

// Find the item type that appears in both compartments of each rucksack.
fn sum_of_the_priorities_of_item_type_that_appears_in_both_compartments_of_each_rucksack(
    input: &str,
) -> Option<u32> {
    use std::collections::HashSet;

    let letter_to_priority = generate_letter_to_priority();
    input
        .lines()
        .flat_map(|rucksack| {
            let (left, right) = rucksack.split_at(rucksack.len() / 2);
            let left_set: HashSet<char> = left.chars().collect();
            let right_set: HashSet<char> = right.chars().collect();
            let common_itmes: Vec<char> = left_set.intersection(&right_set).cloned().collect();
            common_itmes
        })
        .map(|item| *letter_to_priority.get(&item).unwrap())
        .map(|priority| Some(priority as u32))
        .sum()
}

// Find the item type that corresponds to the badges of each three-Elf group.
fn sum_of_the_priorities_of_item_type_that_corresponds_to_the_badges_of_each_three_elf_group(
    input: &str,
) -> Option<u32> {
    use itertools::Itertools;

    input
        .lines()
        .tuples::<(&str, &str, &str)>()
        .flat_map(|(rucksack_1, rucksack_2, rucksack_3)| {
            rucksack_1
                .chars()
                .find(|item| rucksack_2.contains(*item) && rucksack_3.contains(*item))
        })
        .map(|item| Some(ascii_char_to_priority(item)))
        .sum()
}

//
//
//

fn generate_letter_to_priority() -> std::collections::HashMap<char, usize> {
    use std::collections::HashMap;

    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect::<HashMap<char, usize>>()
}

fn ascii_char_to_priority(character: char) -> u32 {
    // ascii 'A' starts at 65, mapping to values 27-52
    const UPPERCASE_SUBTRAHEND: u8 = b'A' - 27;
    // ascii 'a' starts at 97, mapping to values 0-26
    const LOWERCASE_SUBTRAHEND: u8 = b'a' - 1;
    match character {
        _ if character.is_ascii_uppercase() => character as u32 - UPPERCASE_SUBTRAHEND as u32,
        _ if character.is_ascii_lowercase() => character as u32 - LOWERCASE_SUBTRAHEND as u32,
        _ => panic!("Unsupported character"),
    }
}
