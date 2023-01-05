//

use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1, u8},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

//
#[derive(Debug)]
pub struct Procedure {
    pub from: usize,
    pub moves: usize,
    pub to: usize,
}

//

pub fn parse(input: &str) -> IResult<&str, Vec<(u8, u8, u8)>> {
    separated_list1(
        newline,
        tuple((
            preceded(tag("move"), delimited(space1, u8, space1)),
            preceded(tag("from"), delimited(space1, u8, space1)),
            preceded(tag("to"), preceded(space1, u8)),
        )),
    )(input)
}
