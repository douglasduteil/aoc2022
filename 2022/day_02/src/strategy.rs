//

use std::convert::Infallible;

use crate::{beats::Beats, hand_shape::HandShape};

//

pub enum Strategy {
    Lost,
    Draw,
    Won,
}

impl Strategy {
    pub fn winning_hand(&self, opponent_hand: &HandShape) -> HandShape {
        match *self {
            Self::Draw => *opponent_hand,
            Self::Lost => opponent_hand.beats().beats(),
            Self::Won => opponent_hand.beats(),
        }
    }
}

impl std::str::FromStr for Strategy {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Won),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Lost),
            _ => unreachable!("invalid strategy : {s}"),
        }
    }
}
