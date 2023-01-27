//

use crate::{position::Position, rope::Rope};

//

#[test]
fn test_example_one_steps() {
    let mut rope = Rope::<10>::default();

    // == Initial State ==

    // ......
    // ......
    // ......
    // ......
    // H.....  (H covers 1, 2, 3, 4, 5, 6, 7, 8, 9, s)
    assert_eq!(&rope.knots, &[Position(0, 0); 10]);

    // == R 4 ==

    // ......
    // ......
    // ......
    // ......
    // 1H....  (1 covers 2, 3, 4, 5, 6, 7, 8, 9, s)
    rope.move_head_by(Position(1, 0));
    assert_eq!(
        &rope.knots,
        [[Position(1, 0)].as_slice(), [Position(0, 0); 9].as_slice()]
            .concat()
            .as_slice()
    );

    // ......
    // ......
    // ......
    // ......
    // 21H...  (2 covers 3, 4, 5, 6, 7, 8, 9, s)
    rope.move_head_by(Position(1, 0));
    assert_eq!(
        &rope.knots,
        [
            [Position(2, 0), Position(1, 0)].as_slice(),
            [Position(0, 0); 8].as_slice()
        ]
        .concat()
        .as_slice()
    );

    // ......
    // ......
    // ......
    // ......
    // 321H..  (3 covers 4, 5, 6, 7, 8, 9, s)
    rope.move_head_by(Position(1, 0));
    assert_eq!(
        &rope.knots,
        [
            [Position(3, 0), Position(2, 0), Position(1, 0)].as_slice(),
            [Position(0, 0); 7].as_slice()
        ]
        .concat()
        .as_slice()
    );

    // ......
    // ......
    // ......
    // ......
    // 4321H.  (4 covers 5, 6, 7, 8, 9, s)
    rope.move_head_by(Position(1, 0));
    assert_eq!(
        &rope.knots,
        [
            [
                Position(4, 0),
                Position(3, 0),
                Position(2, 0),
                Position(1, 0)
            ]
            .as_slice(),
            [Position(0, 0); 6].as_slice()
        ]
        .concat()
        .as_slice()
    );

    // == U 4 ==

    // ......
    // ......
    // ......
    // ....H.
    // 4321..  (4 covers 5, 6, 7, 8, 9, s)
    rope.move_head_by(Position(0, 1));
    assert_eq!(
        &rope.knots,
        [
            [
                Position(4, 1),
                Position(3, 0),
                Position(2, 0),
                Position(1, 0)
            ]
            .as_slice(),
            [Position(0, 0); 6].as_slice()
        ]
        .concat()
        .as_slice()
    );

    // ......
    // ......
    // ....H.
    // .4321.
    // 5.....  (5 covers 6, 7, 8, 9, s)
    rope.move_head_by(Position(0, 1));
    assert_eq!(
        &rope.knots,
        [
            [
                Position(4, 2),
                Position(4, 1),
                Position(3, 1),
                Position(2, 1),
                Position(1, 1)
            ]
            .as_slice(),
            [Position(0, 0); 5].as_slice()
        ]
        .concat()
        .as_slice()
    );

    // ......
    // ....H.
    // ....1.
    // .432..
    // 5.....  (5 covers 6, 7, 8, 9, s)
    rope.move_head_by(Position(0, 1));
    assert_eq!(
        &rope.knots,
        [
            [
                Position(4, 3),
                Position(4, 2),
                Position(3, 1),
                Position(2, 1),
                Position(1, 1)
            ]
            .as_slice(),
            [Position(0, 0); 5].as_slice()
        ]
        .concat()
        .as_slice()
    );

    // ....H.
    // ....1.
    // ..432.
    // .5....
    // 6.....  (6 covers 7, 8, 9, s)
    rope.move_head_by(Position(0, 1));
    assert_eq!(
        &rope.knots,
        [
            [
                Position(4, 4),
                Position(4, 3),
                Position(4, 2),
                Position(3, 2),
                Position(2, 2),
                Position(1, 1)
            ]
            .as_slice(),
            [Position(0, 0); 4].as_slice()
        ]
        .concat()
        .as_slice()
    );

    // == L 3 ==

    // ...H..
    // ....1.
    // ..432.
    // .5....
    // 6.....  (6 covers 7, 8, 9, s)
    rope.move_head_by(Position(-1, 0));
    assert_eq!(
        &rope.knots,
        [
            [
                Position(3, 4),
                Position(4, 3),
                Position(4, 2),
                Position(3, 2),
                Position(2, 2),
                Position(1, 1)
            ]
            .as_slice(),
            [Position(0, 0); 4].as_slice()
        ]
        .concat()
        .as_slice()
    );

    // ..H1..
    // ...2..
    // ..43..
    // .5....
    // 6.....  (6 covers 7, 8, 9, s)
    rope.move_head_by(Position(-1, 0));
    assert_eq!(
        &rope.knots,
        [
            [
                Position(2, 4),
                Position(3, 4),
                Position(3, 3),
                Position(3, 2),
                Position(2, 2),
                Position(1, 1)
            ]
            .as_slice(),
            [Position(0, 0); 4].as_slice()
        ]
        .concat()
        .as_slice()
    );

    // .H1...
    // ...2..
    // ..43..
    // .5....
    // 6.....  (6 covers 7, 8, 9, s)
    rope.move_head_by(Position(-1, 0));
    assert_eq!(
        &rope.knots,
        [
            [
                Position(1, 4),
                Position(2, 4),
                Position(3, 3),
                Position(3, 2),
                Position(2, 2),
                Position(1, 1)
            ]
            .as_slice(),
            [Position(0, 0); 4].as_slice()
        ]
        .concat()
        .as_slice()
    );

    // == D 1 ==

    // ..1...
    // .H.2..
    // ..43..
    // .5....
    // 6.....  (6 covers 7, 8, 9, s)
    rope.move_head_by(Position(0, -1));
    assert_eq!(
        &rope.knots,
        [
            [
                Position(1, 3),
                Position(2, 4),
                Position(3, 3),
                Position(3, 2),
                Position(2, 2),
                Position(1, 1)
            ]
            .as_slice(),
            [Position(0, 0); 4].as_slice()
        ]
        .concat()
        .as_slice()
    );

    // == R 4 ==

    // ..1...
    // ..H2..
    // ..43..
    // .5....
    // 6.....  (6 covers 7, 8, 9, s)
    rope.move_head_by(Position(1, 0));
    assert_eq!(
        &rope.knots,
        [
            [
                Position(2, 3),
                Position(2, 4),
                Position(3, 3),
                Position(3, 2),
                Position(2, 2),
                Position(1, 1)
            ]
            .as_slice(),
            [Position(0, 0); 4].as_slice()
        ]
        .concat()
        .as_slice()
    );

    // ..1...
    // ...H..  (H covers 2)
    // ..43..
    // .5....
    // 6.....  (6 covers 7, 8, 9, s)
    rope.move_head_by(Position(1, 0));
    assert_eq!(
        &rope.knots,
        [
            [
                Position(3, 3),
                Position(2, 4),
                Position(3, 3),
                Position(3, 2),
                Position(2, 2),
                Position(1, 1)
            ]
            .as_slice(),
            [Position(0, 0); 4].as_slice()
        ]
        .concat()
        .as_slice()
    );

    // ......
    // ...1H.  (1 covers 2)
    // ..43..
    // .5....
    // 6.....  (6 covers 7, 8, 9, s)
    rope.move_head_by(Position(1, 0));
    assert_eq!(
        &rope.knots,
        [
            [
                Position(4, 3),
                Position(3, 3),
                Position(3, 3),
                Position(3, 2),
                Position(2, 2),
                Position(1, 1)
            ]
            .as_slice(),
            [Position(0, 0); 4].as_slice()
        ]
        .concat()
        .as_slice()
    );

    // ......
    // ...21H
    // ..43..
    // .5....
    // 6.....  (6 covers 7, 8, 9, s)
    rope.move_head_by(Position(1, 0));
    assert_eq!(
        &rope.knots,
        [
            [
                Position(5, 3),
                Position(4, 3),
                Position(3, 3),
                Position(3, 2),
                Position(2, 2),
                Position(1, 1)
            ]
            .as_slice(),
            [Position(0, 0); 4].as_slice()
        ]
        .concat()
        .as_slice()
    );

    // == D 1 ==

    // ......
    // ...21.
    // ..43.H
    // .5....
    // 6.....  (6 covers 7, 8, 9, s)
    rope.move_head_by(Position(0, -1));
    assert_eq!(
        &rope.knots,
        [
            [
                Position(5, 2),
                Position(4, 3),
                Position(3, 3),
                Position(3, 2),
                Position(2, 2),
                Position(1, 1)
            ]
            .as_slice(),
            [Position(0, 0); 4].as_slice()
        ]
        .concat()
        .as_slice()
    );

    // == L 5 ==

    // ......
    // ...21.
    // ..43H.
    // .5....
    // 6.....  (6 covers 7, 8, 9, s)
    rope.move_head_by(Position(-1, 0));
    assert_eq!(
        &rope.knots,
        [
            [
                Position(4, 2),
                Position(4, 3),
                Position(3, 3),
                Position(3, 2),
                Position(2, 2),
                Position(1, 1)
            ]
            .as_slice(),
            [Position(0, 0); 4].as_slice()
        ]
        .concat()
        .as_slice()
    );

    // ......
    // ...21.
    // ..4H..  (H covers 3)
    // .5....
    // 6.....  (6 covers 7, 8, 9, s)
    rope.move_head_by(Position(-1, 0));
    assert_eq!(
        &rope.knots,
        [
            [
                Position(3, 2),
                Position(4, 3),
                Position(3, 3),
                Position(3, 2),
                Position(2, 2),
                Position(1, 1)
            ]
            .as_slice(),
            [Position(0, 0); 4].as_slice()
        ]
        .concat()
        .as_slice()
    );

    // ......
    // ...2..
    // ..H1..  (H covers 4; 1 covers 3)
    // .5....
    // 6.....  (6 covers 7, 8, 9, s)
    rope.move_head_by(Position(-1, 0));
    assert_eq!(
        &rope.knots,
        [
            [
                Position(2, 2),
                Position(3, 2),
                Position(3, 3),
                Position(3, 2),
                Position(2, 2),
                Position(1, 1)
            ]
            .as_slice(),
            [Position(0, 0); 4].as_slice()
        ]
        .concat()
        .as_slice()
    );

    // ......
    // ...2..
    // .H13..  (1 covers 4)
    // .5....
    // 6.....  (6 covers 7, 8, 9, s)
    rope.move_head_by(Position(-1, 0));
    assert_eq!(
        &rope.knots,
        [
            [
                Position(1, 2),
                Position(2, 2),
                Position(3, 3),
                Position(3, 2),
                Position(2, 2),
                Position(1, 1)
            ]
            .as_slice(),
            [Position(0, 0); 4].as_slice()
        ]
        .concat()
        .as_slice()
    );

    // ......
    // ......
    // H123..  (2 covers 4)
    // .5....
    // 6.....  (6 covers 7, 8, 9, s)
    rope.move_head_by(Position(-1, 0));
    assert_eq!(
        &rope.knots,
        [
            [
                Position(0, 2),
                Position(1, 2),
                Position(2, 2),
                Position(3, 2),
                Position(2, 2),
                Position(1, 1)
            ]
            .as_slice(),
            [Position(0, 0); 4].as_slice()
        ]
        .concat()
        .as_slice()
    );

    // == R 2 ==

    // ......
    // ......
    // .H23..  (H covers 1; 2 covers 4)
    // .5....
    // 6.....  (6 covers 7, 8, 9, s)
    rope.move_head_by(Position(1, 0));
    assert_eq!(
        &rope.knots,
        [
            [
                Position(1, 2),
                Position(1, 2),
                Position(2, 2),
                Position(3, 2),
                Position(2, 2),
                Position(1, 1)
            ]
            .as_slice(),
            [Position(0, 0); 4].as_slice()
        ]
        .concat()
        .as_slice()
    );

    // ......
    // ......
    // .1H3..  (H covers 2, 4)
    // .5....
    // 6.....  (6 covers 7, 8, 9, s)
    rope.move_head_by(Position(1, 0));
    assert_eq!(
        &rope.knots,
        [
            [
                Position(2, 2),
                Position(1, 2),
                Position(2, 2),
                Position(3, 2),
                Position(2, 2),
                Position(1, 1)
            ]
            .as_slice(),
            [Position(0, 0); 4].as_slice()
        ]
        .concat()
        .as_slice()
    );
}
