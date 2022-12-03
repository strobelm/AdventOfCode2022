use itertools::Itertools;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum_of_all_values = 0;
    for line in input.lines() {
        let intersection = get_intersection_one_line(line);
        sum_of_all_values += calculate_value(intersection);
    }
    Some(sum_of_all_values as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum_of_all_values = 0;
    for chunk in &input.lines().into_iter().chunks(3) {
        let vec = Vec::from_iter(chunk);
        let h_fst: HashSet<char> = vec[0].chars().clone().collect();
        let h_scd: HashSet<char> = vec[1].chars().clone().collect();
        let h_thr: HashSet<char> = vec[2].chars().clone().collect();

        let intersection: Vec<char> = h_fst
            .intersection(&h_scd.intersection(&h_thr).cloned().collect())
            .cloned()
            .collect();

        sum_of_all_values += calculate_value(intersection);
    }
    Some(sum_of_all_values as u32)
}

pub fn calculate_value(intersection: Vec<char>) -> usize {
    let lower_chars = ('a'..='z').into_iter().collect::<Vec<char>>();
    let upper_chars = ('A'..='Z').into_iter().collect::<Vec<char>>();
    let all_chars = [lower_chars, upper_chars].concat();

    let values: Vec<usize> = intersection
        .iter()
        .map(|c| all_chars.iter().position(|&x| x == *c).unwrap() + 1) // +1 since index start at 0 in rust
        .collect();

    let sum_local: usize = values.iter().sum();
    sum_local
}

pub fn get_intersection_one_line(line: &str) -> Vec<char> {
    let len_half = line.len() / 2;

    let first_half = &line[..len_half];
    let second_half = &line[len_half..];

    let hash_first_half: HashSet<char> = first_half.chars().clone().collect();
    let hash_second_half: HashSet<char> = second_half.chars().clone().collect();

    let intersection = hash_first_half
        .intersection(&hash_second_half)
        .cloned()
        .collect();
    intersection
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
