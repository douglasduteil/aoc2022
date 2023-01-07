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
    fn test_part_one_example1() {
        let input = advent_of_code::read_example();
        assert_eq!(part_one(&input.lines().nth(0).unwrap()), Some(7));
    }

    #[test]
    fn test_part_one_example2() {
        let input = advent_of_code::read_example();
        assert_eq!(part_one(&input.lines().nth(1).unwrap()), Some(5));
    }

    #[test]
    fn test_part_one_example3() {
        let input = advent_of_code::read_example();
        assert_eq!(part_one(&input.lines().nth(2).unwrap()), Some(6));
    }

    #[test]
    fn test_part_one_example4() {
        let input = advent_of_code::read_example();
        assert_eq!(part_one(&input.lines().nth(3).unwrap()), Some(10));
    }

    #[test]
    fn test_part_one_example5() {
        let input = advent_of_code::read_example();
        assert_eq!(part_one(&input.lines().nth(4).unwrap()), Some(11));
    }

    //

    #[test]
    fn test_part_two_example1() {
        let input = advent_of_code::read_example();
        assert_eq!(part_two(&input.lines().nth(0).unwrap()), None);
    }

    #[test]
    fn test_part_two_example2() {
        let input = advent_of_code::read_example();
        assert_eq!(part_two(&input.lines().nth(1).unwrap()), None);
    }

    #[test]
    fn test_part_two_example3() {
        let input = advent_of_code::read_example();
        assert_eq!(part_two(&input.lines().nth(2).unwrap()), None);
    }

    #[test]
    fn test_part_two_example4() {
        let input = advent_of_code::read_example();
        assert_eq!(part_two(&input.lines().nth(3).unwrap()), None);
    }

    #[test]
    fn test_part_two_example5() {
        let input = advent_of_code::read_example();
        assert_eq!(part_two(&input.lines().nth(4).unwrap()), None);
    }
}
