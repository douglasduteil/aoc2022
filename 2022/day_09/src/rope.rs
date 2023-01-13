//

use crate::{motion::Motion, position::Position};

//

type Head = Position;
type Tail = Position;

#[derive(Default, PartialEq, Clone, Copy)]
pub struct Rope(pub Head, pub Tail);

impl std::fmt::Debug for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Rope")
            .field("Head", &self.0)
            .field("Tail", &self.1)
            .finish()
    }
}

//

impl Rope {
    pub fn step_iter<'a>(&'a self, motion: &'a Motion) -> Box<dyn Iterator<Item = Rope> + '_> {
        use Motion::*;
        //

        let Rope(start_head, start_tail) = self;

        let steps_iter = match motion {
            Up(steps) | Down(steps) | Right(steps) | Left(steps) => steps.to_owned(),
        };

        let head_positions_iter = steps_iter.map(move |delta| match motion {
            Up(_) => Position(start_head.0, start_head.1 + delta as isize),
            Down(_) => Position(start_head.0, start_head.1 - delta as isize),
            Left(_) => Position(start_head.0 - delta as isize, start_head.1),
            Right(_) => Position(start_head.0 + delta as isize, start_head.1),
        });

        let mut last_head = start_head.to_owned();
        let tail_positions_iter =
            head_positions_iter
                .clone()
                .fold(Vec::new(), |mut next_tail_positions, next_head| {
                    let last_tail = next_tail_positions.last().unwrap_or(start_tail);

                    let next_tail = if next_head.distance(last_tail) > 1u8 {
                        // The head (`H`) and tail (`T`) are not close enough
                        // the tail takes the last head position
                        last_head.to_owned()
                    } else {
                        // The head (`H`) and tail (`T`) are still in touch
                        // the tail shouldn't move
                        last_tail.to_owned()
                    };

                    next_tail_positions.push(next_tail);
                    last_head = next_head;

                    next_tail_positions
                });
        Box::new(head_positions_iter.zip(tail_positions_iter).map(Rope::from))
    }
}

//

impl From<(Position, Position)> for Rope {
    fn from((x, y): (Position, Position)) -> Self {
        Rope(x, y)
    }
}
impl From<((isize, isize), (isize, isize))> for Rope {
    fn from((head, tail): ((isize, isize), (isize, isize))) -> Self {
        Rope(Position::from(head), Position::from(tail))
    }
}

//

#[cfg(test)]
mod test_motion {
    use super::*;

    #[test]
    fn test_step_iter_nothing() {
        assert_eq!(
            Rope::default()
                .step_iter(&"R 0".parse().unwrap())
                .collect::<Vec<Rope>>(),
            &[]
        )
    }

    #[test]
    fn test_step_iter_right_4() {
        assert_eq!(
            Rope::default()
                .step_iter(&"R 4".parse().unwrap())
                .collect::<Vec<Rope>>(),
            &[
                Rope::from(((1, 0), (0, 0))),
                Rope::from(((2, 0), (1, 0))),
                Rope::from(((3, 0), (2, 0))),
                Rope::from(((4, 0), (3, 0))),
            ]
        )
    }

    #[test]
    fn test_step_iter_up_4() {
        assert_eq!(
            Rope::default()
                .step_iter(&"U 4".parse().unwrap())
                .collect::<Vec<Rope>>(),
            &[
                Rope::from(((0, 1), (0, 0))),
                Rope::from(((0, 2), (0, 1))),
                Rope::from(((0, 3), (0, 2))),
                Rope::from(((0, 4), (0, 3))),
            ]
        )
    }

    #[test]
    fn test_step_iter_left_3() {
        assert_eq!(
            Rope::default()
                .step_iter(&"L 3".parse().unwrap())
                .collect::<Vec<Rope>>(),
            &[
                Rope::from(((-1, 0), (0, 0))),
                Rope::from(((-2, 0), (-1, 0))),
                Rope::from(((-3, 0), (-2, 0))),
            ]
        )
    }

    #[test]
    fn test_step_iter_down_1() {
        assert_eq!(
            Rope::default()
                .step_iter(&"D 1".parse().unwrap())
                .collect::<Vec<Rope>>(),
            &[Rope::from(((0, -1), (0, 0))),]
        )
    }

    #[test]
    fn test_step_iter_unmovable_tail() {
        //. . .
        //. T .
        //. . .
        let mut rope = Rope::from(((1, 1), (1, 1)));

        //. . .
        //. H .
        //. . .
        assert_eq!(rope, Rope::from(((1, 1), (1, 1))));

        //. H .
        //. T .
        //. . .
        rope = rope.step_iter(&"U 1".parse().unwrap()).last().unwrap();
        assert_eq!(rope, Rope::from(((1, 2), (1, 1))));

        //. . H
        //. T .
        //. . .
        rope = rope.step_iter(&"R 1".parse().unwrap()).last().unwrap();
        assert_eq!(rope, Rope::from(((2, 2), (1, 1))));

        //. . .
        //. T H
        //. . .
        rope = rope.step_iter(&"D 1".parse().unwrap()).last().unwrap();
        assert_eq!(rope, Rope::from(((2, 1), (1, 1))));

        //. . .
        //. T .
        //. . H
        rope = rope.step_iter(&"D 1".parse().unwrap()).last().unwrap();
        assert_eq!(rope, Rope::from(((2, 0), (1, 1))));

        //. . .
        //. T .
        //. H .
        rope = rope.step_iter(&"L 1".parse().unwrap()).last().unwrap();
        assert_eq!(rope, Rope::from(((1, 0), (1, 1))));

        //. . .
        //. T .
        //H . .
        rope = rope.step_iter(&"L 1".parse().unwrap()).last().unwrap();
        assert_eq!(rope, Rope::from(((0, 0), (1, 1))));

        //. . .
        //H T .
        //. . .
        rope = rope.step_iter(&"U 1".parse().unwrap()).last().unwrap();
        assert_eq!(rope, Rope::from(((0, 1), (1, 1))));

        //. . .
        //. H .
        //. . .
        rope = rope.step_iter(&"R 1".parse().unwrap()).last().unwrap();
        assert_eq!(rope, Rope::from(((1, 1), (1, 1))));
    }

    #[test]
    fn test_step_iter_diagonal() {
        let mut rope = Rope::default();

        //. . .
        //. . .
        //H . .
        assert_eq!(rope, Rope::from(((0, 0), (0, 0))));

        //. . .
        //H . .
        //T . .
        rope = rope.step_iter(&"U 1".parse().unwrap()).last().unwrap();
        assert_eq!(rope, Rope::from(((0, 1), (0, 0))));

        //. . .
        //. H .
        //T . .
        rope = rope.step_iter(&"R 1".parse().unwrap()).last().unwrap();
        assert_eq!(rope, Rope::from(((1, 1), (0, 0))));

        //. H .
        //. T .
        //. . .
        rope = rope.step_iter(&"U 1".parse().unwrap()).last().unwrap();
        assert_eq!(rope, Rope::from(((1, 2), (1, 1))));

        //. . H
        //. T .
        //. . .
        rope = rope.step_iter(&"R 1".parse().unwrap()).last().unwrap();
        assert_eq!(rope, Rope::from(((2, 2), (1, 1))));
    }
}
