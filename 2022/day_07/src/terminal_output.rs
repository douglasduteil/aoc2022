//

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline, one_of, space1},
    combinator::{map, recognize},
    error::{context, convert_error, VerboseError},
    multi::{many1, separated_list0, separated_list1},
    number::complete::float,
    sequence::{preceded, separated_pair},
    Finish,
};
use std::str::FromStr;

//

pub struct TerminalOutput(pub Vec<Command>);

//

impl FromStr for TerminalOutput {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse(s).finish() {
            Ok((_, commands)) => Ok(Self(commands)),
            Err(e) => Err(convert_error(s, e)),
        }
    }
}

//

type IResult<T, U> = nom::IResult<T, U, VerboseError<T>>;

fn parse(input: &str) -> IResult<&str, Vec<Command>> {
    context(
        "commands",
        separated_list1(
            newline,
            alt((map(command_cd, Command::Cd), map(command_ls, Command::Ls))),
        ),
    )(input)
}

//

fn command_cd(input: &str) -> IResult<&str, Cd> {
    context(
        "cd",
        preceded(
            tag("$ cd "),
            alt((
                map(tag("/"), |_| Cd::RootDir),
                map(tag(".."), |_| Cd::Out),
                map(context("directory", alphanumeric1), |directory| {
                    Cd::In(String::from(directory))
                }),
            )),
        ),
    )(input)
}

fn command_ls(input: &str) -> IResult<&str, Vec<Content>> {
    context(
        "ls",
        preceded(
            tag("$ ls\n"),
            separated_list0(
                newline,
                alt((
                    // dir a
                    map(
                        separated_pair(tag("dir"), space1, is_filename),
                        |(_, dir)| Content::Directory(String::from(dir)),
                    ),
                    // 14848514 b.txt
                    map(
                        separated_pair(float, space1, is_filename),
                        |(size, name)| Content::File {
                            name: String::from(name),
                            size: size as usize,
                        },
                    ),
                )),
            ),
        ),
    )(input)
}

//

fn is_filename(input: &str) -> IResult<&str, &str> {
    let allowed_chars = alt((alphanumeric1, recognize(one_of("._-"))));
    recognize(many1(allowed_chars))(input)
}

//

pub enum Command {
    Cd(Cd),
    Ls(Vec<Content>),
}

pub enum Cd {
    In(String),
    Out,
    RootDir,
}

pub enum Content {
    Directory(String),
    File { name: String, size: usize },
}
