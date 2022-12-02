pub fn part_one(input: &str) -> Option<u32> {
    let nums = split_and_parse(input);
    let max = nums.iter().max().copied();
    max
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut nums = split_and_parse(input);
    nums.sort();
    let sum = nums.iter().rev().take(3).sum();
    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn split_and_parse(input: &str) -> Vec<u32> {
    let result = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<u32>().expect("could not parse int value"))
                .sum()
        })
        .collect();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
