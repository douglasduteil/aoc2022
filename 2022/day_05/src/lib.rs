//

mod procedure;
mod stacks;

//

use nom::{
    character::{
        self,
        complete::{newline, space0},
    },
    multi::many1,
    sequence::{delimited, terminated},
    IResult,
};
use procedure::Procedure;
use std::collections::VecDeque;

//

pub fn part_one(input: &str) -> Option<String> {
    let (crate_stacks, rearrangement_procedure) = input_parser(input);

    let mut crates = crate_stacks;
    for Procedure { moves, from, to } in rearrangement_procedure.into_iter() {
        let moved_crates: Vec<char> = crates[from].drain(..moves).collect();
        for crate_ in moved_crates.iter() {
            crates[to].push_front(*crate_);
        }
    }
    Some(
        crates
            .into_iter()
            .fold(String::new(), |tops, stack| tops + &stack[0].to_string()),
    )
}

pub fn part_two(_input: &str) -> Option<String> {
    None
}

//

fn input_parser(input: &str) -> (Vec<VecDeque<char>>, Vec<Procedure>) {
    let (input, crate_stacks) = stacks::parse(input).expect("invalid crate stack");
    let (input, _) = crate_number_parsing(input).expect("invalide number lines");
    let (_, procedure) = procedure::parse(input).expect("invalide procedure");
    (
        crate_stacks,
        procedure
            .iter()
            .map(|&(moves, from, to)| Procedure {
                moves: moves as usize,
                from: from as usize - 1,
                to: to as usize - 1,
            })
            .collect(),
    )
}

fn crate_number_parsing(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, result) = terminated(
        terminated(
            many1(delimited(space0, character::complete::u8, space0)),
            newline,
        ),
        newline,
    )(input)?;
    Ok((input, result))
}
