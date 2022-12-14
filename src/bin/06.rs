use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    Some(find_index(input, 4) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(find_index(input, 14) as u32)
}

fn find_index(input: &str, offset: usize) -> usize {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(offset)
        .position(|w| w.iter().all_unique())
        .unwrap()
        + offset
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
