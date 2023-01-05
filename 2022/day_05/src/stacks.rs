//

use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, newline},
    combinator::value,
    multi::{fold_many1, separated_list1},
    sequence::{delimited, terminated},
    IResult,
};

//

pub fn parse(input: &str) -> IResult<&str, Vec<VecDeque<char>>> {
    fold_many1(
        terminated(crate_line, newline),
        Vec::new,
        |mut col: Vec<VecDeque<char>>, row| {
            row.into_iter().enumerate().for_each(|(i, name)| {
                match col.get(i) {
                    None => col.push(VecDeque::new()),
                    Some(_) => (),
                }
                if name.is_alphanumeric() {
                    col[i].push_back(name);
                }
            });
            col
        },
    )(input)
}

#[cfg(test)]
mod test_crate_stacks {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(
            parse(&vec!["    [D]    ", "[N] [C]    ", "[Z] [M] [P]", ""].join("\n")),
            Ok((
                "",
                vec![
                    VecDeque::from(['N', 'Z']),
                    VecDeque::from(['D', 'C', 'M']),
                    VecDeque::from(['P'])
                ]
            ))
        );
    }
}

//

fn crate_line(input: &str) -> IResult<&str, Vec<char>> {
    separated_list1(
        tag(" "),
        alt((
            // the [X] crate
            delimited(tag("["), anychar, tag("]")),
            // or nothing
            value(' ', tag("   ")),
        )),
    )(input)
}

#[cfg(test)]
mod test_crate_line {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(crate_line("   "), Ok(("", vec![' '])));
    }
    #[test]
    fn test_a_b_c() {
        assert_eq!(crate_line("[A] [B] [C]"), Ok(("", vec!['A', 'B', 'C'])));
    }
    #[test]
    fn test_a_c() {
        assert_eq!(crate_line("[A]     [C]"), Ok(("", vec!['A', ' ', 'C'])));
    }
    #[test]
    fn test_d() {
        assert_eq!(crate_line("    [D]    "), Ok(("", vec![' ', 'D', ' '])));
    }
}
