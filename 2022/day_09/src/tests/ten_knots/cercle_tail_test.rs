//

use crate::{position::Position, rope::Rope};

//

#[test]
fn test_example_motion_series() {
    let rope = Rope::new(10, Position(0, 0));

    // ......
    // ......
    // ......
    // ......
    // H.....  (H covers 1, 2, 3, 4, 5, 6, 7, 8, 9, s)
    let rope = rope.clone();
    assert_eq!(
        rope.clone().knots.into_iter().collect::<Vec<_>>(),
        &[
            (0, 0), // H
            (0, 0), // 1
            (0, 0), // 2
            (0, 0), // 3
            (0, 0), // 4
            (0, 0), // 5
            (0, 0), // 6
            (0, 0), // 7
            (0, 0), // 8
            (0, 0)  // 9
        ]
        .map(Position::from)
    );

    // == R 4 ==
    let motion = "R 4".parse().unwrap();
    let iter = rope.apply_motion_iter(&motion);

    // ......
    // ......
    // ......
    // ......
    // 4321H.  (4 covers 5, 6, 7, 8, 9, s)
    let rope = iter.into_iter().last().unwrap();
    assert_eq!(
        rope.clone().knots.into_iter().collect::<Vec<_>>(),
        &[
            (4, 0), // H
            (3, 0), // 1
            (2, 0), // 2
            (1, 0), // 3
            (0, 0), // 4
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0)
        ]
        .map(Position::from)
    );

    // == U 4 ==
    let motion = "U 2".parse().unwrap();
    let mut iter = rope.apply_motion_iter(&motion);

    // ......
    // ......
    // ......
    // ....H.
    // 4321..  (4 covers 5, 6, 7, 8, 9, s)
    let rope = iter.next().unwrap();
    assert_eq!(
        rope.clone().knots.into_iter().collect::<Vec<_>>(),
        &[
            (4, 1), // H
            (3, 0), // 1
            (2, 0), // 2
            (1, 0), // 3
            (0, 0), // 4
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0)
        ]
        .map(Position::from)
    );

    // ......
    // ......
    // ....H.
    // ....1.
    // 5432..  (5 covers 6, 7, 8, 9, s)
    // let rope = iter.next().unwrap();
    // assert_eq!(
    //     rope.clone().knots.into_iter().collect::<Vec<_>>(),
    //     &[
    //         (4, 2), // H
    //         (4, 1), // 1
    //         (2, 0), // 2
    //         (1, 0), // 3
    //         (0, 0), // 4
    //         (0, 0),
    //         (0, 0),
    //         (0, 0),
    //         (0, 0),
    //         (0, 0)
    //     ]
    //     .map(Position::from)
    // );
}
