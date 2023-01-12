//

use crate::{direction::Direction, tree::Tree, visibility_check::VisibilityCheck};

pub struct ScenicScore {}

impl ScenicScore {
    pub fn calculate_from(visibility_checker: &VisibilityCheck, tree: &Tree) -> u32 {
        Direction::iter()
            .map(|direction| visibility_checker.count_visible_trees(tree, direction))
            // NOTE(douglasduteil): One does not simply multiply by 0 but ...
            // """
            // If a tree is right on the edge, at least one of its viewing
            // distances will be zero.
            // """
            // --- README.md
            .product::<u32>()
    }
}

//
//
//

#[cfg(test)]
mod test_scenic_score_count_visible_trees {
    use crate::ugrid::UGrid;

    use super::*;

    #[test]
    fn test_best_1x1() {
        let forest: UGrid = "5".parse().unwrap();

        for (index, &score) in vec![0u32].iter().enumerate() {
            let position = forest.index_to_coord(index);
            assert_eq!(
                (
                    index,
                    ScenicScore::calculate_from(
                        &VisibilityCheck::in_forest(&forest),
                        &Tree {
                            position,
                            height: forest[position],
                        }
                    )
                ),
                (index, score)
            );
        }
    }

    #[test]
    fn test_best_3x3() {
        let forest: UGrid = "533
                           \n354
                           \n539"
            .parse()
            .unwrap();

        for (index, &score) in vec![0u32, 0, 0, 0, 1, 0, 0, 0].iter().enumerate() {
            let position = forest.index_to_coord(index);
            assert_eq!(
                (
                    index,
                    ScenicScore::calculate_from(
                        &VisibilityCheck::in_forest(&forest),
                        &Tree {
                            position,
                            height: forest[position],
                        }
                    )
                ),
                (index, score)
            );
        }
    }

    #[test]
    fn test_best_5x5() {
        let forest: UGrid = "30373
                           \n25512
                           \n65332
                           \n33549
                           \n35390"
            .parse()
            .unwrap();

        for (index, &score) in vec![
            // 1  2  3  4
            0, 0, 0, 0, 0, // 0
            0, 1, 4, 1, 0, // 1
            0, 6, 1, 2, 0, // 2
            0, 1, 8, 3, 0, // 3
            0, 0, 0, 0, 0, // 4
        ]
        .iter()
        .enumerate()
        {
            let position = forest.index_to_coord(index);
            assert_eq!(
                (
                    (index, position),
                    ScenicScore::calculate_from(
                        &VisibilityCheck::in_forest(&forest),
                        &Tree {
                            position,
                            height: forest[position],
                        }
                    )
                ),
                ((index, position), score)
            );
        }
    }
}
