//

use std::ops::{Range, RangeFrom};

//
#[derive(Debug)]
pub struct Tree {
    pub position: (usize, usize),
    pub height: usize,
}

impl Tree {
    pub fn iter_looking_up(&self) -> std::iter::Rev<Range<usize>> {
        let Tree { position, .. } = self;
        let (.., y) = position;

        (0..*y).rev()
    }

    pub fn iter_looking_down(&self) -> RangeFrom<usize> {
        let Tree { position, .. } = self;
        let (.., y) = position;
        (y + 1)..
    }

    pub fn iter_looking_left(&self) -> std::iter::Rev<Range<usize>> {
        let Tree { position, .. } = self;
        let (x, ..) = position;
        (0..*x).rev()
    }

    pub fn iter_looking_right(&self) -> RangeFrom<usize> {
        let Tree { position, .. } = self;
        let (x, ..) = position;
        (x + 1)..
    }
}
