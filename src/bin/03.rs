fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

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
    use itertools::Itertools;
    let letter_to_priority = generate_letter_to_priority();
    input
        .lines()
        .flat_map(|rucksack| {
            let (left, right) = rucksack.split_at(rucksack.len() / 2);
            left.chars()
                .filter(|item| right.contains(*item))
                .unique()
                .collect_vec()
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

    let letter_to_priority = generate_letter_to_priority();

    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|team_of_3_rucksack| {
            // HACK(douglasduteil): Chunk is not Debug friendly...
            // HACK(douglasduteil): Chunk doesn't have a fixed size...
            let mut chunk_it = team_of_3_rucksack;
            match [chunk_it.next(), chunk_it.next(), chunk_it.next()] {
                [Some(rucksack_1), Some(rucksack_2), Some(rucksack_3)] => {
                    [rucksack_1, rucksack_2, rucksack_3]
                }
                _ => ["", "", ""],
            }
        })
        .flat_map(|[rucksack_1, rucksack_2, rucksack_3]| {
            rucksack_1
                .chars()
                .find(|item| rucksack_2.contains(*item) && rucksack_3.contains(*item))
        })
        .map(|item| *letter_to_priority.get(&item).unwrap())
        .map(|priority| Some(priority as u32))
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

//
//
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
