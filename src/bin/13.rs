use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        parse_block(input)
            .iter()
            .enumerate()
            .filter(|(_, v)| v.0 < v.1)
            .map(|(i, _)| i + 1)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let input_without_empty_lines = input.replace("\n\n", "\n");

    let sig1: Data = serde_json::from_str("[[2]]").unwrap();
    let sig2: Data = serde_json::from_str("[[6]]").unwrap();

    let mut parsed: Vec<Data> = input_without_empty_lines
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    parsed.extend(vec![sig1.clone(), sig2.clone()]);
    parsed.sort_unstable();

    let idx1 = parsed.iter().position(|x| *x == sig1).unwrap() + 1;
    let idx2 = parsed.iter().position(|x| *x == sig2).unwrap() + 1;

    Some(idx1 * idx2)
}

fn parse_block(input: &str) -> Vec<(Data, Data)> {
    input
        .split("\n\n")
        .map(|block| {
            let (a, b) = block.split_once('\n').unwrap();
            (
                serde_json::from_str(a).unwrap(),
                serde_json::from_str(b).unwrap(),
            )
        })
        .collect()
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(untagged)]
enum Data {
    Int(u8),
    List(Vec<Data>),
}
impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Data::Int(a), Data::Int(b)) => a.cmp(b),
            (Data::Int(a), Data::List(_)) => Data::List(vec![Data::Int(*a)]).cmp(other),
            (Data::List(_), Data::Int(a)) => self.cmp(&Data::List(vec![Data::Int(*a)])),
            (Data::List(a), Data::List(b)) => {
                let min_len = a.len().min(b.len());
                for i in 0..min_len {
                    let order = a[i].cmp(&b[i]);
                    match order {
                        Ordering::Equal => (),
                        _ => return order,
                    }
                }

                a.len().cmp(&b.len())
            }
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }

    #[test]
    fn compare_ints() {
        let data_a = "1";
        let data_b = "2";
        let a: Data = serde_json::from_str(data_a).unwrap();
        let b: Data = serde_json::from_str(data_b).unwrap();
        assert!(a < b);
    }

    #[test]
    fn compare_lists_same_lenght() {
        let data_a = "[1,2,3]";
        let data_b = "[2,3,4]";
        let a: Data = serde_json::from_str(data_a).unwrap();
        let b: Data = serde_json::from_str(data_b).unwrap();
        assert!(a < b);
    }

    #[test]
    fn compare_lists_first_shorter() {
        let data_a = "[1,2]";
        let data_b = "[2,3,4]";
        let a: Data = serde_json::from_str(data_a).unwrap();
        let b: Data = serde_json::from_str(data_b).unwrap();
        assert!(a < b);
    }

    #[test]
    fn compare_lists_second_shorter() {
        let data_a = "[1,2,3]";
        let data_b = "[2,3]";
        let a: Data = serde_json::from_str(data_a).unwrap();
        let b: Data = serde_json::from_str(data_b).unwrap();
        assert!(a < b);
    }
    #[test]
    fn example_1() {
        let data_a = "[1,1,3,1,1]";
        let data_b = "[1,1,5,1,1]";
        let a: Data = serde_json::from_str(data_a).unwrap();
        let b: Data = serde_json::from_str(data_b).unwrap();
        assert!(a < b);
    }

    #[test]
    fn example_2() {
        let data_a = "[[1],[2,3,4]]";
        let data_b = "[[1],4]";
        let a: Data = serde_json::from_str(data_a).unwrap();
        let b: Data = serde_json::from_str(data_b).unwrap();
        assert!(a < b);
    }
}
