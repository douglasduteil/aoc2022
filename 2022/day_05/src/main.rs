fn main() {
    use puzzles::{part_one, part_two};
    let input = &advent_of_code::read_input();
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use puzzles::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_example();
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_example();
        assert_eq!(part_two(&input), None);
    }
}
