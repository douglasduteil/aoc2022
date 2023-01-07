//

use std::{convert::Infallible, str::FromStr};

//

pub fn part_one(input: &str) -> Option<u32> {
    let TreeGrid(tree_grid) = input.parse().expect("invalid input");

    let trees_on_the_edge = tree_grid.len() * 2 + tree_grid[0].len() * 2 - 4;
    let x_max = tree_grid[0].len();
    let y_max = tree_grid.len();
    let visible_interior_trees = tree_grid[..tree_grid.len() - 1]
        .iter()
        .enumerate()
        .skip(1)
        .flat_map(|(y, rows)| {
            rows[..rows.len() - 1]
                .iter()
                .enumerate()
                .skip(1)
                .map(|(x, height)| (x, y, *height))
                .collect::<Vec<_>>()
        })
        .filter(|&(x, y, height)| {
            false || (0..y).all(|y_| tree_grid[y_][x] < height) //top
            || (x + 1..x_max).all(|x_| tree_grid[y][x_] < height) // right
            || (y + 1..y_max).all(|y_| tree_grid[y_][x] < height) // down
            || (0..x).all(|x_| tree_grid[y][x_] < height) // left
        })
        .count();

    Some(trees_on_the_edge as u32 + visible_interior_trees as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

//

struct TreeGrid(Vec<Vec<usize>>);

impl FromStr for TreeGrid {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TreeGrid(s.lines().fold(Vec::new(), |mut rows, s| {
            rows.push(
                s.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect(),
            );
            rows
        })))
    }
}
