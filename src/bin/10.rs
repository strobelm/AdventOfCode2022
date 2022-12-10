use std::collections::VecDeque;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<i32> {
    let mut cpu = Cpu::new(input);
    let cycles = cpu.stack.len();

    let sum: i32 = (1..=cycles)
        .into_iter()
        .filter_map(|i| match i as i32 {
            i if i % 40 == 20 => {
                let val = i * cpu.reg;
                cpu.tick();
                Some(val)
            }
            _ => {
                cpu.tick();
                None
            }
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut cpu = Cpu::new(input);

    let cycles = 1..=cpu.stack.len();

    let res: Vec<String> = cycles
        .chunks(40)
        .into_iter()
        .map(|c| {
            return c
                .into_iter()
                .enumerate()
                .map(|(i, _)| {
                    cpu.tick();
                    if (cpu.reg - 1..=cpu.reg + 1).contains(&((i + 1) as i32)) {
                        return "#";
                    } else {
                        return ".";
                    }
                })
                .collect();
        })
        .collect();

    Some(res.into_iter().collect())
}

#[derive(Debug)]
struct Cpu {
    reg: i32,
    stack: VecDeque<i32>,
}

impl Cpu {
    fn new(input: &str) -> Cpu {
        let stack = input
            .lines()
            .flat_map(|s| match s {
                "noop" => vec![0],
                s if s.starts_with("addx") => {
                    vec![0, s.split_once(' ').unwrap().1.parse::<i32>().unwrap()]
                }
                _ => unreachable!(),
            })
            .collect();

        Cpu { reg: 1, stack }
    }

    fn tick(&mut self) -> i32 {
        if let Some(el) = self.stack.pop_front() {
            self.reg += el;
        }
        self.reg
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some("#..##..##..##..##..##..##..##..##..##...##...###...###...###...###...###...###..###....####....####....####....####.....####.....#####.....#####.....#####......#####......######......######......####.######.......#######.......#######......".to_string()));
    }
}
