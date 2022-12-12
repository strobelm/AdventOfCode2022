use itertools::Itertools;
use regex::Regex;
use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<u128> {
    let raw_monkeys: Vec<&str> = input.split("\n\n").collect();
    let mut monkeys: Vec<Monkey> = raw_monkeys.iter().map(|m| Monkey::build(m)).collect();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                let monkey = &monkeys[i];
                let item = match monkey.operation {
                    Operation::Mult(factor) => item * factor,
                    Operation::Square => item * item,
                    Operation::Add(term) => item + term,
                } / 3;

                let target = if item % monkey.divisor == 0 {
                    monkey.true_target
                } else {
                    monkey.false_target
                };
                monkeys[i].count += 1;
                monkeys[target as usize].items.push_back(item);
            }
        }
    }
    monkeys.sort_unstable_by_key(|m| m.count);
    let monkey_business = monkeys
        .iter()
        .rev()
        .take(2)
        .collect_vec()
        .iter()
        .map(|m| m.count as u128)
        .product::<u128>() as u128;

    Some(monkey_business)
}

pub fn part_two(input: &str) -> Option<u128> {
    let raw_monkeys: Vec<&str> = input.split("\n\n").collect();
    let mut monkeys: Vec<Monkey> = raw_monkeys.iter().map(|m| Monkey::build(m)).collect();
    let modulo = monkeys.iter().fold(1, |acc, monkey| {
        if acc % monkey.divisor == 0 {
            acc
        } else {
            acc * monkey.divisor
        }
    });

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                let monkey = &monkeys[i];
                let item = match monkey.operation {
                    Operation::Mult(factor) => item * factor,
                    Operation::Square => item * item,
                    Operation::Add(term) => item + term,
                } % modulo;

                let target = if item % monkey.divisor == 0 {
                    monkey.true_target
                } else {
                    monkey.false_target
                };
                monkeys[i].count += 1;
                monkeys[target as usize].items.push_back(item);
            }
        }
    }
    monkeys.sort_unstable_by_key(|m| m.count);
    let monkey_business = monkeys
        .iter()
        .rev()
        .take(2)
        .collect_vec()
        .iter()
        .map(|m| m.count as u128)
        .product::<u128>() as u128;

    Some(monkey_business)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Debug)]
enum Operation {
    Add(u64),
    Mult(u64),
    Square,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    divisor: u64,
    true_target: u64,
    false_target: u64,
    count: u64,
}

impl Monkey {
    fn build(input: &str) -> Self {
        let re = Regex::new(r#"(?m)^Monkey (\d+):\n  Starting items: (\d+(?:, \d+)*)\n  Operation: new = old (?:\* (?:(\d+)|(old))|\+ (\d+))\n  Test: divisible by (\d+)\n    If true: throw to monkey (\d+)\n    If false: throw to monkey (\d+)$"#).unwrap();
        let matches = re.captures(input).unwrap();
        dbg!(matches);

        let items: VecDeque<u64> = Regex::new(r#"Starting items: (\d+(?:, \d+)*)\n"#)
            .unwrap()
            .captures(input)
            .unwrap()[1]
            .split(',')
            .map(|it| it.trim().parse::<u64>().unwrap())
            .collect();

        let operation = input
            .lines()
            .find(|l| l.trim_start().starts_with("Operation: new = old "))
            .unwrap()
            .strip_prefix("  Operation: new = old ")
            .unwrap()
            .split_once(' ')
            .map(|(o, x)| match (o, x) {
                ("*", "old") => Operation::Square, // todo!
                ("*", _) => Operation::Mult(x.parse::<u64>().unwrap()),
                ("+", _) => Operation::Add(x.parse::<u64>().unwrap()),
                _ => unreachable!(),
            })
            .unwrap();

        let divisor = Regex::new(r#"Test: divisible by (\d*)"#)
            .unwrap()
            .captures(input)
            .unwrap()[1]
            .parse::<u64>()
            .unwrap();

        let true_target = Regex::new(r#"If true: throw to monkey (\d*)"#)
            .unwrap()
            .captures(input)
            .unwrap()[1]
            .parse::<u64>()
            .unwrap();

        let false_target = Regex::new(r#"If false: throw to monkey (\d*)"#)
            .unwrap()
            .captures(input)
            .unwrap()[1]
            .parse::<u64>()
            .unwrap();

        Monkey {
            items,
            divisor,
            true_target,
            false_target,
            operation,
            count: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
