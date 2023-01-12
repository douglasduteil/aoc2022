//

mod direction;
mod scenic_score;
mod tree;
mod ugrid;
mod visibility_check;

//

use direction::Direction;
use tree::Tree;
use ugrid::UGrid;

//

pub fn part_one(input: &str) -> Option<u32> {
    use visibility_check::VisibilityCheck;

    let forest: UGrid = input.parse().expect("invalid input");

    let visible_interior_trees = (0..forest.rows * forest.columns)
        .map(tree_from_forest_id(&forest))
        .filter(|tree| {
            let Tree { position, .. } = tree;
            let &(x, y) = position;

            if x == 0 || x == forest.rows - 1 || y == 0 || y == forest.columns - 1 {
                // All of the trees around the edge of the grid are *visible*
                return true;
            };

            Direction::iter().any(|direction| {
                VisibilityCheck::in_forest(&forest).is_visible_from_outside(tree, direction)
            })
        })
        .count();

    Some(visible_interior_trees as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    use scenic_score::ScenicScore;
    use visibility_check::VisibilityCheck;

    //

    let forest: UGrid = input.parse().expect("invalid input");

    (0..forest.rows * forest.columns)
        .map(tree_from_forest_id(&forest))
        .map(|tree| ScenicScore::calculate_from(&VisibilityCheck::in_forest(&forest), &tree))
        .max()
}

//

fn tree_from_forest_id(forest: &UGrid) -> impl Fn(usize) -> Tree + '_ {
    move |index| {
        let position = forest.index_to_coord(index);

        Tree {
            position,
            height: forest[position],
        }
    }
}
