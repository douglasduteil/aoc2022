fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

//

pub fn part_one(input: &str) -> Option<u32> {
    println!("= Find the Elf carrying the most Calories.");
    match find_the_elf_carrying_the_most_calories(input) {
        Some(result) => println!("the {}th elf", result),
        None => println!("..."),
    };

    println!("= How many total Calories is that Elf carrying?");
    how_many_total_calories_is_that_elf_carrying(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    println!("= Find the top three Elves carrying the most Calories.");
    match find_the_top_three_elves_carrying_the_most_calories(input) {
        Some(result) => println!("the elves {:?} ", result),
        None => println!("..."),
    };
    println!("= How many Calories are those Elves carrying in total?");
    how_many_calories_are_those_elves_carrying_in_total(input)
}

//

pub fn find_the_elf_carrying_the_most_calories(input: &str) -> Option<usize> {
    let max_calories = how_many_total_calories_is_that_elf_carrying(input).unwrap();

    sum_calories_by_elf(input)
        .into_iter()
        .position(|elf_calories| elf_calories == max_calories)
}

pub fn how_many_total_calories_is_that_elf_carrying(input: &str) -> Option<u32> {
    sum_calories_by_elf(input).into_iter().max()
}

//

pub fn find_the_top_three_elves_carrying_the_most_calories(input: &str) -> Option<Vec<usize>> {
    let elves = sum_calories_by_elf(input);
    let top_three_elves = top_three_elves(input);

    top_three_elves
        .into_iter()
        .map(|top_calories| {
            elves
                .iter()
                .position(|elf_calories| elf_calories == &top_calories)
        })
        .collect()
}

pub fn how_many_calories_are_those_elves_carrying_in_total(input: &str) -> Option<u32> {
    Some(top_three_elves(input).into_iter().sum())
}

//

fn sum_calories_by_elf(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|inventory| {
            inventory
                .lines()
                .flat_map(|calories| calories.parse::<u32>())
                .sum()
        })
        .collect()
}

fn top_three_elves(input: &str) -> Vec<u32> {
    use itertools::Itertools;
    sum_calories_by_elf(input)
        .into_iter()
        .sorted_by(|a, b| Ord::cmp(b, a))
        .take(3)
        .collect()
}

//
//
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_the_elf_carrying_the_most_calories() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(find_the_elf_carrying_the_most_calories(&input), Some(3));
    }

    #[test]
    fn test_how_many_total_calories_is_that_elf_carrying() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(
            how_many_total_calories_is_that_elf_carrying(&input),
            Some(24000)
        );
    }

    #[test]
    fn test_find_the_top_three_elves_carrying_the_most_calories() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(
            find_the_top_three_elves_carrying_the_most_calories(&input),
            Some(vec![3, 2, 4])
        );
    }

    #[test]
    fn test_how_many_calories_are_those_elves_carrying_in_total() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(
            how_many_calories_are_those_elves_carrying_in_total(&input),
            Some(45000)
        );
    }
}
