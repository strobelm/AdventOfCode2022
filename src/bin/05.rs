use itertools::Itertools;
use regex::Regex;
use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<String> {
    let mut harbor = Harbor::build(input);
    harbor.apply_moves();
    Some(harbor.get_top_of_stack())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut harbor = Harbor::build(input);
    harbor.apply_moves_9001();
    Some(harbor.get_top_of_stack())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Debug, Clone)]
struct Harbor {
    stacks: Vec<VecDeque<String>>,
    moves: Vec<Move>,
}
impl Harbor {
    fn build(iinput: &str) -> Harbor {
        let (harbor_str, moves_str) = iinput.split("\n\n").next_tuple().unwrap();

        let last_line = harbor_str.lines().last().unwrap();
        let n_stacks: usize = last_line
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let mut stacks = vec![VecDeque::new(); n_stacks];

        for line in harbor_str.lines() {
            let re = Regex::new(r"\[[A-Z]\]").unwrap();
            for z in line.chars().chunks(4).into_iter().zip(stacks.iter_mut()) {
                let (chunk, stack) = z;
                let item = chunk.collect::<String>();
                if re.is_match(&item) {
                    stack.push_front(item)
                }
            }
        }

        let moves: Vec<Move> = moves_str.lines().map(Move::from).collect();
        Harbor { stacks, moves }
    }

    fn apply_moves(&mut self) {
        for mv in &self.moves {
            for _ in 0..mv.amount {
                let it = &self.stacks[mv.from].pop_back().unwrap();
                let _ = &self.stacks[mv.to].push_back(it.to_string());
            }
        }
    }

    fn apply_moves_9001(&mut self) {
        for mv in &self.moves {
            let len = self.stacks[mv.from].len();
            let drained = &self.stacks[mv.from]
                .drain(len - mv.amount..)
                .collect::<VecDeque<String>>();
            for it in drained {
                let _ = &self.stacks[mv.to].push_back(it.to_string());
            }
        }
    }

    fn get_top_of_stack(self) -> String {
        let str = self
            .stacks
            .iter()
            .map(|s| s.back().unwrap().to_string())
            .collect_vec()
            .concat();

        let re = Regex::new(r"[^A-Z]").unwrap();
        re.replace_all(&str, "").into()
    }
}

#[derive(Debug, Copy, Clone)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Move {
    fn from(input: &str) -> Self {
        let re = Regex::new(r"\d+").unwrap();
        let nums = re
            .find_iter(input)
            .filter_map(|digits| digits.as_str().parse().ok());

        let (amount, from, to) = nums.collect_tuple().unwrap();

        Move {
            amount,
            from: from - 1,
            to: to - 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".into()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".into()));
    }
}
