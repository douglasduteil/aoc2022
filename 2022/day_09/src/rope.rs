//

use crate::position::Position;

//

#[derive(Debug, PartialEq, Clone)]
pub struct Rope<const N: usize> {
    pub knots: [Position; N],
}
impl<const N: usize> Default for Rope<N> {
    fn default() -> Self {
        Self {
            knots: [Position::default(); N],
        }
    }
}
impl<const N: usize> Rope<N> {
    pub fn tail(&self) -> &Position {
        self.knots.get(N - 1).unwrap()
    }

    pub fn move_head_by(&mut self, vector: Position) {
        self.knots[0] += vector;

        for i in 1..N {
            let head = self.knots[i - 1];
            let tail = self.knots[i];
            if tail.distance(&head) > 1 {
                self.knots[i] += Position(head.0 - tail.0, head.1 - tail.1).normalize();
            }
        }
    }
}

//

#[cfg(test)]
mod test_rope {
    use super::*;

    #[test]
    fn test_simple_rope() {
        let Rope { knots, .. } = Rope::<2>::default();
        assert_eq!(&knots, &[Position(0, 0), Position(0, 0)])
    }

    #[test]
    fn test_move_head_by_right_4() {
        let mut rope = Rope::<2>::default();
        assert_eq!(&rope.knots, &[Position(0, 0), Position(0, 0)]);

        //

        rope.move_head_by(Position(1, 0));
        assert_eq!(&rope.knots, &[Position(1, 0), Position(0, 0)]);

        rope.move_head_by(Position(1, 0));
        assert_eq!(&rope.knots, &[Position(2, 0), Position(1, 0)]);

        rope.move_head_by(Position(1, 0));
        assert_eq!(&rope.knots, &[Position(3, 0), Position(2, 0)]);

        rope.move_head_by(Position(1, 0));
        assert_eq!(&rope.knots, &[Position(4, 0), Position(3, 0)]);
    }

    #[test]
    fn test_move_head_by_up_4() {
        let mut rope = Rope::<2>::default();
        assert_eq!(&rope.knots, &[Position(0, 0), Position(0, 0)]);

        //

        rope.move_head_by(Position(0, 1));
        assert_eq!(&rope.knots, &[Position(0, 1), Position(0, 0)]);

        rope.move_head_by(Position(0, 1));
        assert_eq!(&rope.knots, &[Position(0, 2), Position(0, 1)]);

        rope.move_head_by(Position(0, 1));
        assert_eq!(&rope.knots, &[Position(0, 3), Position(0, 2)]);

        rope.move_head_by(Position(0, 1));
        assert_eq!(&rope.knots, &[Position(0, 4), Position(0, 3)]);
    }

    #[test]
    fn test_move_head_by_left_3() {
        let mut rope = Rope::<2>::default();
        assert_eq!(&rope.knots, &[Position(0, 0), Position(0, 0)]);

        //

        rope.move_head_by(Position(-1, 0));
        assert_eq!(&rope.knots, &[Position(-1, 0), Position(0, 0)]);

        rope.move_head_by(Position(-1, 0));
        assert_eq!(&rope.knots, &[Position(-2, 0), Position(-1, 0)]);

        rope.move_head_by(Position(-1, 0));
        assert_eq!(&rope.knots, &[Position(-3, 0), Position(-2, 0)]);
    }

    #[test]
    fn test_move_head_by_down_1() {
        let mut rope = Rope::<2>::default();
        assert_eq!(&rope.knots, &[Position(0, 0), Position(0, 0)]);

        //

        rope.move_head_by(Position(0, -1));
        assert_eq!(&rope.knots, &[Position(0, -1), Position(0, 0)]);
    }
}

#[cfg(test)]
#[path = "./tests/two_knots/unmovable_tail_test.rs"]
mod two_knots_unmovable_tail_test;

#[cfg(test)]
#[path = "./tests/two_knots/example_one_steps_test.rs"]
mod two_knots_example_one_steps_test;

#[cfg(test)]
#[path = "./tests/ten_knots/example_one_steps_test.rs"]
mod ten_knots_example_one_steps_test;
