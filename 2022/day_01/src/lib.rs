//

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split("\n\n")
        .map(|inventory| {
            inventory
                .lines()
                .flat_map(|calories| calories.parse::<u32>())
                .sum()
        })
        .max()
}

pub fn part_two(input: &str) -> Option<u32> {
    use itertools::Itertools;
    input
        .split("\n\n")
        .map(|inventory| {
            inventory
                .lines()
                .flat_map(|calories| calories.parse::<u32>())
                .sum1::<u32>()
        })
        .sorted_by(|a, b| Ord::cmp(b, a))
        .take(3)
        .sum()
}
