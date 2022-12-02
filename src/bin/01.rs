pub fn part_one(input: &str) -> Option<u32> {
    let nums = split_and_parse(input);
    match nums {
        Ok(nums) => nums.iter().max().copied(),
        Err(_) => None,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = split_and_parse(input);
    match res {
        Ok(res) if res.len() > 2 => {
            let nums: Vec<u32> = res.to_vec();
            let sum = nums.into_iter().rev().take(3).sum::<u32>();
            Some(sum)
        }
        _ => None,
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn split_and_parse(input: &str) -> Result<Vec<u32>, std::num::ParseIntError> {
    let result: Result<Vec<u32>, std::num::ParseIntError> = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<u32>()).sum())
        .collect();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_input() {
        let input = "invalid";
        assert_eq!(part_one(input), None);
    }

    #[test]
    fn test_too_few_input_values() {
        let input = "100\n100\n\n200\n200";
        assert_eq!(part_two(input), None);
    }

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
