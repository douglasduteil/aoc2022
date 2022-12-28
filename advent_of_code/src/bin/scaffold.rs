/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 */
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
    process::{self},
};

use advent_of_code::helpers::LATEST_AOC_YEAR;

struct Args {
    day: u8,
    year: Option<u16>,
}

fn parse_args() -> Result<Args, pico_args::Error> {
    let mut args = pico_args::Arguments::from_env();

    let year = args.opt_value_from_str::<[&str; 2], u16>(["-y", "--year"])?;
    Ok(Args {
        day: args.free_from_str()?,
        year,
    })
}

fn create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create(true).open(path)
}

fn replace_module_name(path: &str, name: &str) {
    let contents =
        std::fs::read_to_string(path).expect("Should have been able to read the file : '{path}");

    match create_file(path)
        .unwrap()
        .write(contents.replace("%%NAME%%", name).as_bytes())
    {
        Ok(_) => {
            println!("Edited {path}");
        }
        Err(_e) => {
            eprintln!("Failed to edit : {path}");
        }
    }
}

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Failed to process arguments:\n  {}", e);
            process::exit(1);
        }
    };

    let year = args.year.unwrap_or(LATEST_AOC_YEAR).to_string();
    let day = format!("day_{:02}", args.day);
    let module_name = format!("day_{}_{:02}", year, args.day);

    let cwd = PathBuf::from(year);
    let module_path = cwd.join(day);

    //

    if module_path.exists() {
        eprintln!("destination `{}` already exists", module_path.display());
        process::exit(1);
    }

    println!("$ mkdir -p {:?}", module_path.join("src"));
    std::fs::create_dir_all(module_path.join("src")).unwrap();

    //

    let root_files = std::fs::read_dir(PathBuf::from(".templates/")).unwrap();
    let src_files = std::fs::read_dir(PathBuf::from(".templates/src")).unwrap();

    root_files
        .chain(src_files)
        .flat_map(|res| res.map(|e| e.path()))
        .filter(|meta| meta.is_file())
        .map(|path| {
            (
                path.clone(),
                module_path.join(path.strip_prefix(".templates/").unwrap()), // .strip_prefix("./.template").unwrap()),
            )
        })
        .for_each(|(from, to)| {
            println!("$ cp {} {}", from.display(), to.display());
            std::fs::copy(from, to).unwrap();
        });

    //

    replace_module_name(
        &module_path.join("Cargo.toml").to_string_lossy(),
        &module_name,
    );

    //

    println!("Created workspace \"{}\"", &module_path.to_string_lossy());

    println!("---");
    println!(
        "🎄 Type `cargo solve {}` to run your solution.",
        &module_name
    );
}
