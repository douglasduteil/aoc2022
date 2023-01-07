//

mod terminal_output;

//

use std::{collections::BTreeMap, path::PathBuf, process};

use terminal_output::{Cd, Command, TerminalOutput};

use crate::terminal_output::Content;

//

pub fn part_one(input: &str) -> Option<u32> {
    let TerminalOutput(output) = input.parse().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let (_, sizes) = output.iter().fold(
        (PathBuf::from("/"), BTreeMap::<String, usize>::new()),
        |(mut path, mut sizes), command| {
            match command {
                Command::Cd(Cd::RootDir) => path.push("/"),
                Command::Cd(Cd::Out) => {
                    path.pop();
                }
                Command::Cd(Cd::In(directory)) => {
                    path.push(directory);
                }
                Command::Ls(contents) => {
                    let files_size = contents
                        .iter()
                        .filter_map(|file| {
                            if let Content::File { size, .. } = file {
                                Some(size)
                            } else {
                                None
                            }
                        })
                        .sum();

                    path.ancestors()
                        .flat_map(|p| p.as_os_str().to_str())
                        .map(String::from)
                        .for_each(|p| {
                            sizes
                                .entry(p)
                                .and_modify(|size| *size += files_size)
                                .or_insert(files_size);
                        });
                }
            }
            (path, sizes)
        },
    );

    sizes
        .iter()
        .filter(|(_, &size)| size < 100_000)
        .map(|(_, &size)| size as u32)
        .map(Some)
        .sum()
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}
