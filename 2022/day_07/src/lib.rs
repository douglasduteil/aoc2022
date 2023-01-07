//

mod terminal_output;

//

use std::{collections::BTreeMap, path::PathBuf, process};

use terminal_output::{Cd, Command, TerminalOutput};

use crate::terminal_output::Content;

//

pub fn part_one(input: &str) -> Option<u32> {
    let terminal_output = input.parse().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    directories_sizes(terminal_output)
        .iter()
        .filter(|(_, &size)| size < 100_000)
        .map(|(_, &size)| size as u32)
        .map(Some)
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    const TOTAL_SPACE: usize = 70_000_000;
    const UPDATE_SPACE: usize = 30_000_000;

    let terminal_output = input.parse().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let sizes = directories_sizes(terminal_output);
    let used_space = sizes.get("/").unwrap();
    let free_space = TOTAL_SPACE - used_space;
    let min_space_to_delete = UPDATE_SPACE - free_space;

    sizes
        .iter()
        .filter(|(_, &size)| size >= min_space_to_delete)
        .map(|(_, &size)| size as u32)
        .min()
}

//

fn directories_sizes(TerminalOutput(output): TerminalOutput) -> BTreeMap<String, usize> {
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
}
