//

use crate::{position::Position, rope::Rope};

//

#[test]
fn test_apply_motion_iter_unmovable_tail() {
    let rope = Rope::new(2, Position(1, 1));

    //. . .
    //. T .
    //. . .
    let rope = rope.clone();
    assert_eq!(
        rope.clone().knots.into_iter().collect::<Vec<_>>(),
        &[Position(1, 1), Position(1, 1)]
    );

    //. H .
    //. T .
    //. . .
    let motion = "U 1".parse().unwrap();
    let mut iter = rope.apply_motion_iter(&motion);
    let rope = iter.next().unwrap();
    assert_eq!(
        rope.clone().knots.into_iter().collect::<Vec<_>>(),
        &[Position(1, 2), Position(1, 1)]
    );
    assert_eq!(iter.next(), None);

    //. . H
    //. T .
    //. . .
    let motion = "R 1".parse().unwrap();
    let mut iter = rope.apply_motion_iter(&motion);
    let rope = iter.next().unwrap();
    assert_eq!(
        rope.clone().knots.into_iter().collect::<Vec<_>>(),
        &[Position(2, 2), Position(1, 1)]
    );
    assert_eq!(iter.next(), None);

    //. . .
    //. T H
    //. . .
    let motion = "D 1".parse().unwrap();
    let mut iter = rope.apply_motion_iter(&motion);
    let rope = iter.next().unwrap();
    assert_eq!(
        rope.clone().knots.into_iter().collect::<Vec<_>>(),
        &[Position(2, 1), Position(1, 1)]
    );
    assert_eq!(iter.next(), None);

    //. . .
    //. T .
    //. . H
    let motion = "D 1".parse().unwrap();
    let mut iter = rope.apply_motion_iter(&motion);
    let rope = iter.next().unwrap();
    assert_eq!(
        rope.clone().knots.into_iter().collect::<Vec<_>>(),
        &[Position(2, 0), Position(1, 1)]
    );
    assert_eq!(iter.next(), None);

    //. . .
    //. T .
    //. H .
    let motion = "L 1".parse().unwrap();
    let mut iter = rope.apply_motion_iter(&motion);
    let rope = iter.next().unwrap();
    assert_eq!(
        rope.clone().knots.into_iter().collect::<Vec<_>>(),
        &[Position(1, 0), Position(1, 1)]
    );
    assert_eq!(iter.next(), None);

    //. . .
    //. T .
    //H . .
    let motion = "L 1".parse().unwrap();
    let mut iter = rope.apply_motion_iter(&motion);
    let rope = iter.next().unwrap();
    assert_eq!(
        rope.clone().knots.into_iter().collect::<Vec<_>>(),
        &[Position(0, 0), Position(1, 1)]
    );
    assert_eq!(iter.next(), None);

    //. . .
    //H T .
    //. . .
    let motion = "U 1".parse().unwrap();
    let mut iter = rope.apply_motion_iter(&motion);
    let rope = iter.next().unwrap();
    assert_eq!(
        rope.clone().knots.into_iter().collect::<Vec<_>>(),
        &[Position(0, 1), Position(1, 1)]
    );
    assert_eq!(iter.next(), None);

    //. . .
    //. H .
    //. . .
    let motion = "R 1".parse().unwrap();
    let mut iter = rope.apply_motion_iter(&motion);
    let rope = iter.next().unwrap();
    assert_eq!(
        rope.clone().knots.into_iter().collect::<Vec<_>>(),
        &[Position(1, 1), Position(1, 1)]
    );
    assert_eq!(iter.next(), None);
}
