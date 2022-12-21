fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

//

pub fn part_one(input: &str) -> Option<u32> {
    // Find the item type that appears in both compartments of each rucksack.
    // What is the sum of the priorities of those item types?
    sum_of_the_item_type_that_appears_in_both_compartments_of_each_rucksack(input)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

//
//
//

// Find the item type that appears in both compartments of each rucksack.
pub fn sum_of_the_item_type_that_appears_in_both_compartments_of_each_rucksack(
    input: &str,
) -> Option<u32> {
    use itertools::Itertools;
    use std::collections::HashMap;

    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect::<HashMap<char, usize>>();

    input
        .lines()
        .flat_map(|rucksack| {
            let (left, right) = rucksack.split_at(rucksack.len() / 2);
            left.chars()
                .filter(|item| right.contains(*item))
                .unique()
                .collect_vec()
        })
        .map(|item| letter_scores.get(&item).unwrap())
        .map(|priority| Some(*priority as u32))
        .sum()
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
    #[ignore]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
