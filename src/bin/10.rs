use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<i32> {
    let mut cpu = CPU::new(input);
    let cycles = cpu.stack.len();

    let sum: i32 = (1..=cycles)
        .into_iter()
        .filter_map(|i| match i as i32 {
            i if i == 20 || (i - 20) % 40 == 0 => {
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

pub fn part_two(input: &str) -> Option<u32> {
    let mut cpu = CPU::new(input);

    let mut cyc = 1;
    while !cpu.stack.is_empty() {
        cpu.tick();
        if (cpu.reg - 1..=cpu.reg + 1).contains(&cyc) {
            print!("#")
        } else {
            print!(".");
        }
        if cyc % 40 == 0 {
            println!();
            cyc = 0;
        }
        cyc += 1;
    }
    None
}

#[derive(Debug)]
struct CPU {
    reg: i32,
    stack: VecDeque<i32>,
}

impl CPU {
    fn new(input: &str) -> CPU {
        let stack = input
            .lines()
            .map(|s| match s {
                "noop" => vec![0],
                s if s.starts_with("addx") => {
                    vec![0, s.split_once(" ").unwrap().1.parse::<i32>().unwrap()]
                }
                _ => unreachable!(),
            })
            .flatten()
            .collect();

        CPU { reg: 1, stack }
    }

    fn tick(&mut self) {
        if let Some(el) = self.stack.pop_front() {
            self.reg += el;
        }
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
        assert_eq!(part_two(&input), None);
    }
}
