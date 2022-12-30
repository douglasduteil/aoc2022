//

mod beats;
mod hand_shape;
mod round;
mod strategy;

//

use std::convert::Infallible;

use crate::round::Round;

//

pub fn part_one(input: &str) -> Option<u32> {
    let line_to_round = |round_line: &str| -> Result<Round, Infallible> {
        let (opponent_letter, player_move) = round_line
            .split_once(" ")
            .expect("Could not split opponent and player letters");
        Ok(Round {
            opponent: opponent_letter.parse()?,
            player: player_move.parse()?,
        })
    };

    input
        .lines()
        .flat_map(line_to_round)
        .map(|round| round.get_player_score())
        .map(Some)
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    use crate::strategy::Strategy;

    let line_to_round = |round_line: &str| -> Result<Round, Infallible> {
        let (opponent_letter, strategy_letter) = round_line
            .split_once(" ")
            .expect("Could not split opponent and strategy letters");
        let opponent = opponent_letter.parse()?;
        let player = strategy_letter
            .parse::<Strategy>()
            .unwrap()
            .winning_hand(&opponent);
        Ok(Round { opponent, player })
    };

    input
        .lines()
        .flat_map(line_to_round)
        .map(|round| round.get_player_score())
        .map(Some)
        .sum()
}
