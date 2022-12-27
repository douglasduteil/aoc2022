//

pub fn part_one(input: &str) -> Option<u32> {
    input.parse::<u32>().ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    input.chars().rev().collect::<String>().parse::<u32>().ok()
}
