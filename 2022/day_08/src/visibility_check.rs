//

use crate::{direction::Direction, tree::Tree, ugrid::UGrid};

pub struct VisibilityCheck<'a> {
    forest: &'a UGrid,
}
impl<'a> VisibilityCheck<'a> {
    pub fn in_forest(forest: &'a UGrid) -> Self {
        Self { forest }
    }

    pub fn is_visible_from_outside(&self, tree: &Tree, direction: &Direction) -> bool {
        !self
            .iter_from_tree(tree, direction)
            .map(|index| self.nth_tree_height(tree, direction, &index))
            // A tree is *visible* if all of the other trees [...] are *shorter* than it.
            // A tree is is visible from outside if we can't find a *taller* tree then it.
            .any(|near_tree_height| near_tree_height >= tree.height)
    }

    pub fn count_visible_trees(&self, tree: &Tree, direction: &Direction) -> u32 {
        use itertools::Itertools;

        self.iter_from_tree(tree, direction)
            // [markdown]
            //
            // # NOTE(douglasduteil): take_while_inclusive like
            //
            // Since `take_while_inclusive` or `take_until_inclusive` are nowhere
            // to be found, I'm storing the last viewing state during the fold_while
            //
            // \cf https://github.com/rust-itertools/itertools/issues/597
            .fold_while(
                (Vec::<usize>::new(), false),
                |(mut visible_trees, the_previous_tree_is_blocking_the_view), index| {
                    use itertools::FoldWhile::{Continue, Done};

                    // every tree we see is a visible tree
                    visible_trees.push(index);

                    let is_blocking_the_view =
                        self.nth_tree_height(tree, direction, &index) >= tree.height;

                    let mut view_state = (visible_trees, is_blocking_the_view);

                    if the_previous_tree_is_blocking_the_view {
                        // expect the ones hidden behind another trees...
                        view_state.0.pop();
                        Done(view_state)
                    } else {
                        Continue(view_state)
                    }
                },
            )
            .into_inner()
            .0
            .len() as u32
    }

    //
    //
    //

    fn iter_from_tree(
        &self,
        tree: &Tree,
        direction: &Direction,
    ) -> Box<dyn Iterator<Item = usize> + '_> {
        match direction {
            Direction::Up => Box::new(tree.iter_looking_up()),
            Direction::Down => Box::new(
                tree.iter_looking_down()
                    .take_while(|&y| y < self.forest.rows),
            ),
            Direction::Left => Box::new(tree.iter_looking_left()),
            Direction::Right => Box::new(
                tree.iter_looking_right()
                    .take_while(|&x| x < self.forest.columns),
            ),
        }
    }

    fn nth_tree_height(&self, tree: &Tree, direction: &Direction, &n: &usize) -> usize {
        let Tree { position, .. } = tree;
        let &(start_x, start_y) = position;
        match direction {
            Direction::Up | Direction::Down => self.forest[(start_x, n)],
            Direction::Left | Direction::Right => self.forest[(n, start_y)],
        }
    }
}

#[cfg(test)]
mod test_visibility_check_visible_from_outside {
    use super::*;

    #[test]
    fn test_center_visible_3x3() {
        let forest: UGrid = "123
                           \n495
                           \n678"
            .parse()
            .unwrap();
        let index = 4;
        for direction in Direction::iter() {
            let position = forest.index_to_coord(index);
            assert_eq!(
                (
                    (index, position),
                    VisibilityCheck::in_forest(&forest).is_visible_from_outside(
                        &Tree {
                            position,
                            height: forest[position],
                        },
                        direction
                    )
                ),
                ((index, position), true)
            );
        }
    }

    #[test]
    fn test_center_not_visible_3x3() {
        let forest: UGrid = "123
                           \n405
                           \n678"
            .parse()
            .unwrap();

        let index = 4;
        for direction in Direction::iter() {
            dbg!(direction);
            let position = forest.index_to_coord(index);
            assert_eq!(
                (
                    direction,
                    VisibilityCheck::in_forest(&forest).is_visible_from_outside(
                        &Tree {
                            position,
                            height: forest[position],
                        },
                        direction
                    )
                ),
                (direction, false)
            );
        }
    }

    #[test]
    fn test_x_visible_3x3() {
        let forest: UGrid = "551
                           \n533
                           \n354"
            .parse()
            .unwrap();

        let index = 4;

        for (direction, &is_visible) in Direction::iter().zip(&[false, false, false, false]) {
            let position = forest.index_to_coord(index);
            assert_eq!(
                (
                    direction,
                    VisibilityCheck::in_forest(&forest).is_visible_from_outside(
                        &Tree {
                            position,
                            height: forest[position],
                        },
                        direction
                    )
                ),
                (direction, is_visible)
            );
        }
    }
}
