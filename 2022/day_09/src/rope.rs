//

use std::{collections::LinkedList, ops::RangeInclusive};

use crate::{motion::Motion, position::Position};

//

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Rope {
    pub capacity: usize,
    pub knots: LinkedList<Position>,
}

//

#[derive(Debug)]
struct EachPositionFromMotion {
    motion: Motion,
    range: RangeInclusive<usize>,
    start: Position,
}

impl EachPositionFromMotion {
    fn new(start: Position, motion: Motion) -> EachPositionFromMotion {
        use Motion::*;

        let range = match &motion {
            Up(steps) | Down(steps) | Right(steps) | Left(steps) => steps.to_owned(),
        };

        EachPositionFromMotion {
            motion,
            range,
            start,
        }
    }
}

impl Iterator for EachPositionFromMotion {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let delta = self.range.next();

        if delta.is_none() {
            return None;
        }

        let Position(x, y) = self.start;
        let delta = delta.unwrap() as isize;

        use Motion::*;
        let position = match self.motion {
            Up(_) => Position(x, y + delta),
            Down(_) => Position(x, y - delta),
            Left(_) => Position(x - delta, y),
            Right(_) => Position(x + delta, y),
        };

        Some(position)
    }
}

//

#[derive(Debug)]
pub struct MotionRopeIterMut {
    position_iter: EachPositionFromMotion,
    rope: Rope,
}

impl Iterator for MotionRopeIterMut {
    type Item = Rope;

    fn next(&mut self) -> Option<Self::Item> {
        let next_head = self.position_iter.next();

        if next_head.is_none() {
            return None;
        }
        let next_head = next_head.unwrap();
        let mut last_knot: Option<Position> = None;
        let mut current_head: Option<Position> = None;

        let rope = Rope {
            capacity: self.rope.capacity.clone(),
            knots: self
                .rope
                .knots
                .iter()
                .map(|current_tail| {
                    let next_position = match last_knot {
                        Some(last_knot_position) => {
                            if current_tail.distance(&last_knot_position) > 1u8 {
                                current_head.unwrap()
                            } else {
                                current_tail.clone()
                            }
                        }
                        None => next_head,
                    };

                    last_knot = Some(next_position);
                    current_head = Some(current_tail.clone());

                    next_position
                })
                .collect(),
        };
        self.rope = rope.clone();
        Some(rope)
    }
}

//

impl Rope {
    pub fn new(capacity: usize, initial_position: Position) -> Self {
        assert!(capacity > 1, "A Rope must have more the 2 nobs");

        let knots = std::iter::repeat(initial_position).take(capacity).collect();

        Rope { capacity, knots }
    }

    pub fn apply_motion_iter<'a>(&'a self, motion: &'a Motion) -> MotionRopeIterMut {
        //

        let Rope { knots, .. } = self;
        let start_head = knots.front().unwrap();

        MotionRopeIterMut {
            position_iter: EachPositionFromMotion::new(start_head.clone(), motion.clone()),
            rope: self.clone(),
        }
    }
}

//

#[cfg(test)]
mod test_motion {
    use super::*;

    #[test]
    fn test_simple_rope() {
        let Rope { knots, .. } = Rope::new(2, Position::default());
        assert_eq!(
            knots.into_iter().collect::<Vec<_>>(),
            &[Position(0, 0), Position(0, 0)]
        )
    }

    #[test]
    fn test_apply_motion_iter_right_4() {
        let rope = Rope::new(2, Position::default());
        let motion = "R 4".parse().unwrap();
        let mut iter = rope.apply_motion_iter(&motion);

        {
            let Rope { knots, .. } = iter.next().unwrap();
            assert_eq!(
                knots.into_iter().collect::<Vec<_>>(),
                &[Position(1, 0), Position(0, 0)]
            )
        }
        {
            let Rope { knots, .. } = iter.next().unwrap();
            assert_eq!(
                knots.into_iter().collect::<Vec<_>>(),
                &[Position(2, 0), Position(1, 0)]
            )
        }
        {
            let Rope { knots, .. } = iter.next().unwrap();
            assert_eq!(
                knots.into_iter().collect::<Vec<_>>(),
                &[Position(3, 0), Position(2, 0)]
            )
        }
        {
            let Rope { knots, .. } = iter.next().unwrap();
            assert_eq!(
                knots.into_iter().collect::<Vec<_>>(),
                &[Position(4, 0), Position(3, 0)]
            )
        }
        {
            assert_eq!(iter.next(), None);
        }
    }

    #[test]
    fn test_apply_motion_iter_up_4() {
        let rope = Rope::new(2, Position::default());
        let motion = "U 4".parse().unwrap();
        let mut iter = rope.apply_motion_iter(&motion);

        {
            let Rope { knots, .. } = iter.next().unwrap();
            assert_eq!(
                knots.into_iter().collect::<Vec<_>>(),
                &[Position(0, 1), Position(0, 0)]
            )
        }
        {
            let Rope { knots, .. } = iter.next().unwrap();
            assert_eq!(
                knots.into_iter().collect::<Vec<_>>(),
                &[Position(0, 2), Position(0, 1)]
            )
        }
        {
            let Rope { knots, .. } = iter.next().unwrap();
            assert_eq!(
                knots.into_iter().collect::<Vec<_>>(),
                &[Position(0, 3), Position(0, 2)]
            )
        }
        {
            let Rope { knots, .. } = iter.next().unwrap();
            assert_eq!(
                knots.into_iter().collect::<Vec<_>>(),
                &[Position(0, 4), Position(0, 3)]
            )
        }
        {
            assert_eq!(iter.next(), None);
        }
    }

    #[test]
    fn test_apply_motion_iter_left_3() {
        let rope = Rope::new(2, Position::default());
        let motion = "L 3".parse().unwrap();
        let mut iter = rope.apply_motion_iter(&motion);

        {
            let Rope { knots, .. } = iter.next().unwrap();
            assert_eq!(
                knots.into_iter().collect::<Vec<_>>(),
                &[Position(-1, 0), Position(0, 0)]
            )
        }
        {
            let Rope { knots, .. } = iter.next().unwrap();
            assert_eq!(
                knots.into_iter().collect::<Vec<_>>(),
                &[Position(-2, 0), Position(-1, 0)]
            )
        }
        {
            let Rope { knots, .. } = iter.next().unwrap();
            assert_eq!(
                knots.into_iter().collect::<Vec<_>>(),
                &[Position(-3, 0), Position(-2, 0)]
            )
        }
        {
            assert_eq!(iter.next(), None);
        }
    }

    #[test]
    fn test_apply_motion_iter_down_1() {
        let rope = Rope::new(2, Position::default());
        let motion = "D 1".parse().unwrap();
        let mut iter = rope.apply_motion_iter(&motion);

        {
            let Rope { knots, .. } = iter.next().unwrap();
            assert_eq!(
                knots.into_iter().collect::<Vec<_>>(),
                &[Position(0, -1), Position(0, 0)]
            )
        }
        {
            assert_eq!(iter.next(), None);
        }
    }
}

#[cfg(test)]
#[path = "./tests/two_knots/unmovable_tail_test.rs"]
mod two_knots_unmovable_tail_test;

#[cfg(test)]
#[path = "./tests/ten_knots/cercle_tail_test.rs"]
mod ten_knots_cercle_tail_test;
