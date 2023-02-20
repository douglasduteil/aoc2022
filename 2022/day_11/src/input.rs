//

use std::str::FromStr;

use nom::{
    branch::{alt, permutation},
    bytes::complete::tag,
    character::complete::{digit1, multispace1, newline, space0, space1},
    combinator::map,
    error::{context, convert_error, VerboseError},
    multi::{count, separated_list0, separated_list1},
    number::complete::float,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    Finish,
};

use crate::monkey::{Item, Monkey};
use crate::monkey::{Operation, Test, Worry};

//

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Reader(pub Vec<Monkey>);

//

impl FromStr for Reader {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse(s).finish() {
            Ok((_, monkeys)) => Ok(Reader(monkeys)),
            Err(e) => Err(convert_error(s, e)),
        }
    }
}

//

type IResult<T, U> = nom::IResult<T, U, VerboseError<T>>;

fn parse(input: &str) -> IResult<&str, Vec<Monkey>> {
    context("notes", separated_list1(count(newline, 2), monkey))(input)
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    context(
        "monkey",
        map(
            permutation((
                terminated(monkey_id, newline),
                preceded(multispace1, starting_items),
                preceded(multispace1, operation),
                preceded(multispace1, test),
            )),
            |(id, items, operation, test)| Monkey {
                id,
                items,
                operation,
                test,
            },
        ),
    )(input)
}

fn monkey_id(input: &str) -> IResult<&str, usize> {
    context(
        "monkey_id",
        map(
            separated_pair(tag("Monkey"), space1, terminated(float, tag(":"))),
            |(_, id)| id as usize,
        ),
    )(input)
}

fn starting_items(input: &str) -> IResult<&str, Vec<Item>> {
    context(
        "starting_items",
        preceded(
            terminated(tag("Starting items:"), space0),
            separated_list0(
                tag(", "),
                map(digit1, |value| Item(u64::from_str(value).unwrap())),
            ),
        ),
    )(input)
}

fn operation(input: &str) -> IResult<&str, Operation> {
    context(
        "operation",
        preceded(
            tag("Operation: new = "),
            map(
                tuple((
                    worry_level,
                    delimited(space1, alt((tag("+"), tag("*"))), space1),
                    worry_level,
                )),
                |(left, operator, right)| match operator {
                    "+" => Operation::Add(left, right),
                    "*" => Operation::Multiply(left, right),
                    _ => unimplemented!(),
                },
            ),
        ),
    )(input)
}

fn worry_level(input: &str) -> IResult<&str, Worry> {
    context(
        "worry_level",
        alt((
            map(tag("old"), |_| Worry::Old),
            map(digit1, |value| Worry::Level(u64::from_str(value).unwrap())),
        )),
    )(input)
}

fn test(input: &str) -> IResult<&str, Test> {
    context(
        "test",
        map(
            permutation((
                preceded(tag("Test: divisible by "), digit1),
                preceded(
                    multispace1,
                    preceded(tag("If true: throw to monkey "), digit1),
                ),
                preceded(
                    multispace1,
                    preceded(tag("If false: throw to monkey "), digit1),
                ),
            )),
            |(divisible_by, if_true_monkey_id, if_false_monkey_id)| {
                Test::DivisibleBy(
                    u64::from_str(divisible_by).unwrap(),
                    (
                        usize::from_str(if_true_monkey_id).unwrap(),
                        usize::from_str(if_false_monkey_id).unwrap(),
                    ),
                )
            },
        ),
    )(input)
}

#[cfg(test)]
mod test_notes_parse {
    type ITestResult = Result<(), VerboseError<&'static str>>;

    use super::*;

    #[test]
    fn test_parse() -> ITestResult {
        assert_eq!(
            parse(
                r#"
Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 5:
  Starting items: 89, 87, 60, 78, 54, 77, 98
  Operation: new = old * 7
  Test: divisible by 2
    If true: throw to monkey 1
    If false: throw to monkey 4
"#
                .trim()
            )
            .finish()?,
            (
                "",
                vec![
                    Monkey {
                        id: 1,
                        items: vec![Item(54), Item(65), Item(75), Item(74)],
                        operation: Operation::Add(Worry::Old, Worry::Level(6)),
                        test: Test::DivisibleBy(19, (2, 0))
                    },
                    Monkey {
                        id: 5,
                        items: vec![
                            Item(89),
                            Item(87),
                            Item(60),
                            Item(78),
                            Item(54),
                            Item(77),
                            Item(98)
                        ],
                        operation: Operation::Multiply(Worry::Old, Worry::Level(7)),
                        test: Test::DivisibleBy(2, (1, 4))
                    }
                ]
            )
        );
        //
        Ok(())
    }

    #[test]
    fn test_monkey() -> ITestResult {
        assert_eq!(
            monkey(
                r#"
Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0
"#
                .trim()
            )
            .finish()?,
            (
                "",
                Monkey {
                    id: 1,
                    items: vec![Item(54), Item(65), Item(75), Item(74)],
                    operation: Operation::Add(Worry::Old, Worry::Level(6)),
                    test: Test::DivisibleBy(19, (2, 0))
                }
            )
        );
        //
        Ok(())
    }

    #[test]
    fn test_monkey_id() -> ITestResult {
        assert_eq!(monkey_id("Monkey 0:").finish()?, ("", 0));
        assert_eq!(monkey_id("Monkey 1:").finish()?, ("", 1));
        assert_eq!(monkey_id("Monkey 30:").finish()?, ("", 30));
        assert_eq!(monkey_id("Monkey 42:").finish()?, ("", 42));
        assert_eq!(monkey_id("Monkey 1000:").finish()?, ("", 1_000));
        //
        Ok(())
    }

    #[test]
    fn test_starting_items() -> ITestResult {
        assert_eq!(starting_items("Starting items:").finish()?, ("", vec![]));
        assert_eq!(starting_items("Starting items: ").finish()?, ("", vec![]));
        assert_eq!(
            starting_items("Starting items: 79, 98").finish()?,
            ("", vec![Item(79), Item(98)])
        );
        assert_eq!(
            starting_items("Starting items: 54, 65, 75, 74").finish()?,
            ("", vec![Item(54), Item(65), Item(75), Item(74)])
        );
        assert_eq!(
            starting_items("Starting items: 79, 60, 97").finish()?,
            ("", vec![Item(79), Item(60), Item(97)])
        );
        assert_eq!(
            starting_items("Starting items: 74").finish()?,
            ("", vec![Item(74)])
        );
        //
        Ok(())
    }

    #[test]
    fn test_operation() -> ITestResult {
        assert_eq!(
            operation("Operation: new = old * 19").finish()?,
            ("", Operation::Multiply(Worry::Old, Worry::Level(19)))
        );
        assert_eq!(
            operation("Operation: new = old + 6").finish()?,
            ("", Operation::Add(Worry::Old, Worry::Level(6)))
        );
        assert_eq!(
            operation("Operation: new = old * old").finish()?,
            ("", Operation::Multiply(Worry::Old, Worry::Old))
        );
        assert_eq!(
            operation("Operation: new = old + 3").finish()?,
            ("", Operation::Add(Worry::Old, Worry::Level(3)))
        );
        //
        Ok(())
    }

    #[test]
    fn test_worry_level() -> ITestResult {
        assert_eq!(worry_level("old").finish()?, ("", Worry::Old));
        assert_eq!(worry_level("0").finish()?, ("", Worry::Level(0)));
        assert_eq!(worry_level("6").finish()?, ("", Worry::Level(6)));
        assert_eq!(worry_level("42").finish()?, ("", Worry::Level(42)));
        //
        Ok(())
    }

    #[test]
    fn test_test() -> ITestResult {
        assert_eq!(
            test(
                "Test: divisible by 1
            If true: throw to monkey 2
            If false: throw to monkey 3"
            )
            .finish()?,
            ("", Test::DivisibleBy(1, (2, 3)))
        );
        //
        Ok(())
    }
}
