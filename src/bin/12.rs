use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<i32> {
    let (start_pos, end_pos) = find_start_end(input);
    let result = exec_dijsktra(input, start_pos, end_pos);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, end_pos) = find_start_end(input);
    let all_a = find_a(input);
    let mut results: Vec<i32> = all_a
        .iter()
        .map(|&start_pos_a| exec_dijsktra(input, start_pos_a, end_pos))
        .collect();

    results.sort();
    Some(*results.iter().min().unwrap() as u32)
}

fn exec_dijsktra(input: &str, start_pos: (i32, i32), end_pos: (i32, i32)) -> i32 {
    let map: Vec<Vec<char>> = input
        .replace('S', "a")
        .replace('E', "z")
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let calc_weight = |cur_pos: (i32, i32), next_pos: (i32, i32)| -> i32 {
        let map_size = (map.len() as i32, map[0].len() as i32);
        let x_valid = (0..map_size.0).contains(&next_pos.0);
        let y_valid = (0..map_size.1).contains(&next_pos.1);
        if !x_valid || !y_valid {
            return 99999;
        }

        let cur_pos_char = map
            .get(cur_pos.0 as usize)
            .unwrap()
            .get(cur_pos.1 as usize)
            .unwrap();
        let pos_char = map
            .get(next_pos.0 as usize)
            .unwrap()
            .get(next_pos.1 as usize)
            .unwrap();

        calc_dist(cur_pos_char, pos_char)
    };

    let result = dijkstra(
        &start_pos,
        |&(x, y)| {
            vec![(x, y + 1), (x - 1, y), (x + 1, y), (x, y - 1)]
                .into_iter()
                .map(move |p| (p, calc_weight((x, y), p)))
        },
        |&p| p == end_pos,
    )
    .unwrap();
    result.1
}

fn find_start_end(input: &str) -> ((i32, i32), (i32, i32)) {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let find_char = |c: char| -> (i32, i32) {
        map.iter()
            .position(|v| v.contains(&c))
            .map(|i| {
                let j = map[i].iter().position(|&r| r == c).unwrap();
                (i as i32, j as i32)
            })
            .unwrap()
    };

    let start: (i32, i32) = find_char('S');
    let end: (i32, i32) = find_char('E');

    (start, end)
}

fn find_a(input: &str) -> Vec<(i32, i32)> {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let (m, n) = (map.len(), map[0].len());

    let mut a_pos = vec![];
    for (i, v) in map.iter().enumerate().take(m) {
        for (j, el) in v.iter().enumerate().take(n) {
            if *el == 'a' || *el == 'S' {
                a_pos.push((i as i32, j as i32));
            }
        }
    }

    a_pos
}

fn calc_dist(cur_pos_char: &char, pos_char: &char) -> i32 {
    let value_map: HashMap<_, _> = HashMap::from_iter(
        ('a'..='z')
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v, i as i32)),
    );

    let dist = value_map.get(pos_char).unwrap() - value_map.get(cur_pos_char).unwrap();

    if dist > 1 {
        9999
    } else {
        1
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
