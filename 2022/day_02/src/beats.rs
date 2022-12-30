//

use crate::hand_shape::{
    HandShape,
    HandShape::{Paper, Rock, Scissors},
};

//

pub trait Beats {
    fn beats(&self) -> Self;
}

impl Beats for HandShape {
    fn beats(&self) -> Self {
        match *self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
}
