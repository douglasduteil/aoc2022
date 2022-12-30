//

use crate::beats::Beats;
use crate::hand_shape::HandShape;

pub struct Round {
    pub opponent: HandShape,
    pub player: HandShape,
}

impl Round {
    pub fn get_player_score(&self) -> u32 {
        self.get_move_score() + self.get_battle_score()
    }

    //

    fn get_move_score(&self) -> u32 {
        self.player as u32
    }

    fn get_battle_score(&self) -> u32 {
        match self {
            _ if self.opponent.beats() == self.player => 0, // Lose
            _ if self.player.beats() == self.opponent => 6, // Win
            _ => 3,                                         // Draw
        }
    }
}
