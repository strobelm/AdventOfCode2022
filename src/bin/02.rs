pub fn part_one(input: &str) -> Option<u32> {
    let hands = parse_input_string_to_hands_tuple(input);
    let scores: Vec<u32> = hands.iter().map(calculate_score_one_hand).collect();

    Some(scores.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let hands = parse_input_string_to_hands_tuple(input);
    let hands_after_adapting_strategy: Vec<(Hand, Hand)> = hands
        .iter()
        .map(|input_hands| choose_strategy_one_hand(*input_hands))
        .collect();
    let scores: Vec<u32> = hands_after_adapting_strategy
        .iter()
        .map(calculate_score_one_hand)
        .collect();

    Some(scores.iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Hand {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

pub fn parse_hand(s: &str) -> Result<Hand, String> {
    let t = s.trim();

    match t {
        "X" | "A" => Ok(Hand::Rock),
        "Y" | "B" => Ok(Hand::Paper),
        "Z" | "C" => Ok(Hand::Scissor),
        _ => Err(format!("{} is not a valid hand alias", s)),
    }
}

pub fn choose_strategy_one_hand(input_hands: (Hand, Hand)) -> (Hand, Hand) {
    let other_hand = input_hands.0;
    let my_hand = input_hands.1;

    let desired_result = match my_hand {
        Hand::Rock => HandResult::Lose,
        Hand::Paper => HandResult::Draw,
        Hand::Scissor => HandResult::Win,
    };

    let new_hand = match desired_result {
        HandResult::Win => other_hand.find_beater(),
        HandResult::Draw => other_hand,
        HandResult::Lose => other_hand.beats(),
    };

    (other_hand, new_hand)
}

pub trait Beats {
    fn beats(&self) -> Self;
    fn find_beater(&self) -> Self;
}

impl Beats for Hand {
    fn beats(&self) -> Self {
        match *self {
            Hand::Rock => Hand::Scissor,
            Hand::Paper => Hand::Rock,
            Hand::Scissor => Hand::Paper,
        }
    }

    fn find_beater(&self) -> Self {
        match *self {
            Hand::Scissor => Hand::Rock,
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissor,
        }
    }
}

pub enum HandResult {
    Win,
    Lose,
    Draw,
}

pub fn play_hand(other_hand: Hand, own_hand: Hand) -> HandResult {
    let (my_beats, other_beats) = (own_hand.beats(), other_hand.beats());

    match (my_beats, other_beats) {
        _ if my_beats == other_hand => HandResult::Win,
        _ if other_beats == own_hand => HandResult::Lose,
        _ => HandResult::Draw,
    }
}

pub fn parse_input_string_to_hands_tuple(input: &str) -> Vec<(Hand, Hand)> {
    let lines: Vec<String> = input
        .lines()
        .map(|line| line.parse::<String>().unwrap())
        .collect();
    let hand_pairs: Vec<Vec<&str>> = lines
        .iter()
        .map(|l| l.split_whitespace().collect())
        .collect();
    let hand_vecs: Vec<Vec<Hand>> = hand_pairs
        .iter()
        .map(|h| h.iter().map(|hh| parse_hand(hh).unwrap()).collect())
        .collect();
    let hand_tuple_vecs = hand_vecs.iter().map(|v| (v[0], v[1])).collect();

    hand_tuple_vecs
}

pub fn calculate_score_one_hand(hands: &(Hand, Hand)) -> u32 {
    let result = play_hand(hands.0, hands.1);
    let score_outcome = match result {
        HandResult::Win => 6,
        HandResult::Lose => 0,
        HandResult::Draw => 3,
    };
    let score_hand: u32 = hands.1 as u32;

    score_outcome + score_hand
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input).unwrap(), 15);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input).unwrap(), 12);
    }
}
