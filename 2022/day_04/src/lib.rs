//

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .flat_map(|pairs| pairs.parse())
            .filter(|pairs: &ElfPair| pairs.is_fully_overlapping())
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .flat_map(|pairs| pairs.parse())
            .filter(|pairs: &ElfPair| pairs.is_overlapping())
            .count() as u32,
    )
}

//

use std::{convert::Infallible, ops::RangeInclusive, str::FromStr};

struct Elf(RangeInclusive<usize>);

impl FromStr for Elf {
    type Err = Infallible;

    fn from_str(section: &str) -> Result<Self, Self::Err> {
        let (section_start, section_end) = section
            .split_once('-')
            .expect("invalid section assignments : {section_1}");
        let start = section_start.parse().expect("invalid section id");
        let end = section_end.parse().expect("invalid section id");
        Ok(Elf(std::ops::RangeInclusive::new(start, end)))
    }
}
struct ElfPair(Elf, Elf);

impl ElfPair {
    fn is_overlapping(&self) -> bool {
        let ElfPair(first, second) = self;
        let Elf(first) = first;
        let Elf(second) = second;
        first.to_owned().any(|section| second.contains(&section))
    }

    fn is_fully_overlapping(&self) -> bool {
        let ElfPair(first, second) = self;
        let Elf(first) = first;
        let Elf(second) = second;
        let first_contains_second = first.contains(second.start()) && first.contains(second.end());
        let second_contains_first = second.contains(first.start()) && second.contains(first.end());
        first_contains_second || second_contains_first
    }
}

impl FromStr for ElfPair {
    type Err = Infallible;

    fn from_str(section_pair: &str) -> Result<Self, Self::Err> {
        let (section_1, section_2) = section_pair
            .split_once(',')
            .expect("invalid section assignments pair : {section_pair}");
        let first = section_1.parse().expect("invalid elf");
        let second = section_2.parse().expect("invalid elf");
        Ok(ElfPair(first, second))
    }
}
