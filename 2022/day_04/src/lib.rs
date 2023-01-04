//

use std::convert::Infallible;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .flat_map(|pairs| pairs.parse())
            .filter(|pairs: &ElfPair| pairs.is_fully_overlapping())
            .count() as u32,
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

//

struct ElfPair {
    first: std::ops::RangeInclusive<usize>,
    second: std::ops::RangeInclusive<usize>,
}

impl ElfPair {
    fn is_fully_overlapping(&self) -> bool {
        let first_contains_second =
            self.first.contains(self.second.start()) && self.first.contains(self.second.end());
        let second_contains_first =
            self.second.contains(self.first.start()) && self.second.contains(self.first.end());
        first_contains_second || second_contains_first
    }
}

impl std::str::FromStr for ElfPair {
    type Err = Infallible;

    fn from_str(section_pair: &str) -> Result<Self, Self::Err> {
        let assignments_to_range = |section: &str| {
            let (section_start, section_end) = section
                .split_once('-')
                .expect("invalid section assignments : {section_1}");
            let start = section_start.parse().expect("invalid section id");
            let end = section_end.parse().expect("invalid section id");
            std::ops::RangeInclusive::new(start, end)
        };

        let (section_1, section_2) = section_pair
            .split_once(',')
            .expect("invalid section assignments pair : {section_pair}");
        let first = assignments_to_range(section_1);
        let second = assignments_to_range(section_2);
        Ok(ElfPair { first, second })
    }
}
