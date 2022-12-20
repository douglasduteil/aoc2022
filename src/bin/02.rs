use std::str::FromStr;

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

//

pub fn part_one(input: &str) -> Option<u32> {
    total_score_if_one_trust_the_strategy_guide(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    total_score_if_one_understand_the_strategy_guide(input)
}

//
//
//

#[derive(Debug, PartialEq)]
enum RoundResult {
    Lost = 0,
    Draw = 3,
    Won = 6,
}

//
//
//

enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Hand {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err(format!("'{}' is unknown", s)),
        }
    }
}

//
//
//

enum Strategy {
    RoundResult(RoundResult),
}

impl FromStr for Strategy {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Strategy::RoundResult(RoundResult::Lost)),
            "Y" => Ok(Strategy::RoundResult(RoundResult::Draw)),
            "Z" => Ok(Strategy::RoundResult(RoundResult::Won)),
            _ => Err(()),
        }
    }
}

//
//
//

pub fn total_score_if_one_trust_the_strategy_guide(input: &str) -> Option<u32> {
    input.lines().map(guested_strategy).sum()
}

fn guested_strategy(round: &str) -> Option<u32> {
    let (opponent_letter, your_letter) = round.split_once(' ')?;
    let your_hand = your_letter.parse().unwrap();
    let opponent_hand = opponent_letter.parse().unwrap();
    let hand_score = score_from_hand(&your_hand);
    let round_score = score_from_matchups(&opponent_hand, &your_hand);
    Some((hand_score as u32) + (round_score as u32))
}

//

pub fn total_score_if_one_understand_the_strategy_guide(input: &str) -> Option<u32> {
    input.lines().map(real_strategy).sum()
}

fn real_strategy(round: &str) -> Option<u32> {
    let (opponent_letter, strategy_letter) = round.split_once(' ')?;
    let Strategy::RoundResult(round_result) = strategy_letter.parse().unwrap();
    let opponent_hand = opponent_letter.parse().unwrap();
    let your_hand = strategic_hand(&opponent_hand, &round_result);
    let hand_score = score_from_hand(&your_hand);
    let round_score = score_from_matchups(&opponent_hand, &your_hand);
    Some((hand_score as u32) + (round_score as u32))
}

//
//
//

fn score_from_hand(hand: &Hand) -> usize {
    match hand {
        Hand::Paper => 2,
        Hand::Rock => 1,
        Hand::Scissors => 3,
    }
}

fn score_from_matchups(opponent_hand: &Hand, your_hand: &Hand) -> RoundResult {
    match (opponent_hand, your_hand) {
        (Hand::Paper, Hand::Rock) => RoundResult::Lost,
        (Hand::Paper, Hand::Scissors) => RoundResult::Won,
        (Hand::Rock, Hand::Paper) => RoundResult::Won,
        (Hand::Rock, Hand::Scissors) => RoundResult::Lost,
        (Hand::Scissors, Hand::Paper) => RoundResult::Lost,
        (Hand::Scissors, Hand::Rock) => RoundResult::Won,
        _ => RoundResult::Draw,
    }
}

fn strategic_hand(opponent_hand: &Hand, round: &RoundResult) -> Hand {
    match (opponent_hand, round) {
        (Hand::Paper, RoundResult::Draw) => Hand::Paper,
        (Hand::Paper, RoundResult::Lost) => Hand::Rock,
        (Hand::Paper, RoundResult::Won) => Hand::Scissors,
        //
        (Hand::Rock, RoundResult::Draw) => Hand::Rock,
        (Hand::Rock, RoundResult::Lost) => Hand::Scissors,
        (Hand::Rock, RoundResult::Won) => Hand::Paper,
        //
        (Hand::Scissors, RoundResult::Draw) => Hand::Scissors,
        (Hand::Scissors, RoundResult::Lost) => Hand::Paper,
        (Hand::Scissors, RoundResult::Won) => Hand::Rock,
    }
}
//
//
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_from_matchups() {
        assert_eq!(
            score_from_matchups(&Hand::Rock, &Hand::Rock),
            RoundResult::Draw
        );
        assert_eq!(
            score_from_matchups(&Hand::Rock, &Hand::Paper),
            RoundResult::Won
        );
        assert_eq!(
            score_from_matchups(&Hand::Rock, &Hand::Scissors),
            RoundResult::Lost
        );
        assert_eq!(
            score_from_matchups(&Hand::Paper, &Hand::Rock),
            RoundResult::Lost
        );
        assert_eq!(
            score_from_matchups(&Hand::Paper, &Hand::Paper),
            RoundResult::Draw
        );
        assert_eq!(
            score_from_matchups(&Hand::Paper, &Hand::Scissors),
            RoundResult::Won
        );
        assert_eq!(
            score_from_matchups(&Hand::Scissors, &Hand::Rock),
            RoundResult::Won
        );
        assert_eq!(
            score_from_matchups(&Hand::Scissors, &Hand::Paper),
            RoundResult::Lost
        );
        assert_eq!(
            score_from_matchups(&Hand::Scissors, &Hand::Scissors),
            RoundResult::Draw
        );
    }

    #[test]
    fn test_total_score_if_one_trust_strategy_guide() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(
            total_score_if_one_trust_the_strategy_guide(&input),
            Some(15)
        );
    }

    //
    //
    //

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
