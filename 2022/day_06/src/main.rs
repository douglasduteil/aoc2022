fn main() {
    use puzzles::{part_one, part_two};
    let input = &advent_of_code::read_input();
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests_part_one {
    use puzzles::part_one;

    #[test]
    fn test_example_0() {
        let input = advent_of_code::read_example();
        assert_eq!(part_one(&input.lines().nth(0).unwrap()), Some(7));
    }

    #[test]
    fn test_example_1() {
        let input = advent_of_code::read_example();
        assert_eq!(part_one(&input.lines().nth(1).unwrap()), Some(5));
    }

    #[test]
    fn test_example_2() {
        let input = advent_of_code::read_example();
        assert_eq!(part_one(&input.lines().nth(2).unwrap()), Some(6));
    }

    #[test]
    fn test_example_3() {
        let input = advent_of_code::read_example();
        assert_eq!(part_one(&input.lines().nth(3).unwrap()), Some(10));
    }

    #[test]
    fn test_example_4() {
        let input = advent_of_code::read_example();
        assert_eq!(part_one(&input.lines().nth(4).unwrap()), Some(11));
    }
}

#[cfg(test)]
mod tests_part_two {
    use puzzles::part_two;

    #[test]
    fn test_example_0() {
        let input = advent_of_code::read_example();
        assert_eq!(part_two(&input.lines().nth(0).unwrap()), Some(19));
    }

    #[test]
    fn test_example_1() {
        let input = advent_of_code::read_example();
        assert_eq!(part_two(&input.lines().nth(1).unwrap()), Some(23));
    }

    #[test]
    fn test_example_2() {
        let input = advent_of_code::read_example();
        assert_eq!(part_two(&input.lines().nth(2).unwrap()), Some(23));
    }

    #[test]
    fn test_example_3() {
        let input = advent_of_code::read_example();
        assert_eq!(part_two(&input.lines().nth(3).unwrap()), Some(29));
    }

    #[test]
    fn test_example_4() {
        let input = advent_of_code::read_example();
        assert_eq!(part_two(&input.lines().nth(4).unwrap()), Some(26));
    }
}
