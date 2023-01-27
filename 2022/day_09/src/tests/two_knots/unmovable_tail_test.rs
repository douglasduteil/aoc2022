//

use crate::{position::Position, rope::Rope};

//

#[test]
fn test_unmovable_tail() {
    let mut rope = Rope::<2>::default();
    rope.move_head_by(Position(1, 1));
    rope.move_head_by(Position(1, 1));
    rope.move_head_by(Position(-1, -1));

    //

    //. . .
    //. T .
    //. . .
    assert_eq!(&rope.knots, &[Position(1, 1), Position(1, 1)]);

    //

    rope.move_head_by(Position(0, 1));
    //. H .
    //. T .
    //. . .
    assert_eq!(&rope.knots, &[Position(1, 2), Position(1, 1)]);

    //

    rope.move_head_by(Position(1, 0));
    //. . H
    //. T .
    //. . .
    assert_eq!(&rope.knots, &[Position(2, 2), Position(1, 1)]);

    //

    rope.move_head_by(Position(0, -1));
    //. . .
    //. T H
    //. . .
    assert_eq!(&rope.knots, &[Position(2, 1), Position(1, 1)]);

    //

    rope.move_head_by(Position(0, -1));
    //. . .
    //. T .
    //. . H
    assert_eq!(&rope.knots, &[Position(2, 0), Position(1, 1)]);

    //

    rope.move_head_by(Position(-1, 0));
    //. . .
    //. T .
    //. H .
    assert_eq!(&rope.knots, &[Position(1, 0), Position(1, 1)]);

    //

    rope.move_head_by(Position(-1, 0));
    //. . .
    //. T .
    //H . .
    assert_eq!(&rope.knots, &[Position(0, 0), Position(1, 1)]);

    //

    rope.move_head_by(Position(1, 0));
    //. . .
    //H T .
    //. . .
    assert_eq!(&rope.knots, &[Position(1, 0), Position(1, 1)]);

    //

    rope.move_head_by(Position(0, 1));
    //. . .
    //. H .
    //. . .
    assert_eq!(&rope.knots, &[Position(1, 1), Position(1, 1)]);
}
