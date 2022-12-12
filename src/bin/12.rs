use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let (start_pos, end_pos) = find_start_end(input);
    let map: Vec<Vec<char>> = input
        .replace('S', "a")
        .replace('E', "z")
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let map_size = (map.len() as i32, map[0].len() as i32);
    let value_map: HashMap<_, _> = HashMap::from_iter(
        ('a'..='z')
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v, i as i32)),
    );

    let calc_weight = |cur_pos: (i32, i32), pos: (i32, i32)| -> i32 {
        dbg!(cur_pos, pos);
        let is_valid_pos = pos.0 >= 0 && pos.0 < map_size.0 && pos.1 >= 0 && pos.1 < map_size.1;

        if !is_valid_pos {
            return 999;
        }

        let cur_pos_char = map
            .get(cur_pos.0 as usize)
            .unwrap()
            .get(cur_pos.1 as usize)
            .unwrap();
        // if cur_pos_char == &'S' {
        //     cur_pos_char = &'a';
        // }
        // if cur_pos_char == &'E' {
        //     cur_pos_char = &'z';
        // }
        let pos_char = map
            .get(pos.0 as usize)
            .unwrap()
            .get(pos.1 as usize)
            .unwrap();
        // if pos_char == &'E' {
        //     pos_char = &'z';
        // }
        // if pos_char == &'S' {
        //     pos_char = &'a';
        // }
        dbg!(value_map.get(cur_pos_char).unwrap());
        dbg!(value_map.get(pos_char).unwrap());
        //  , value_map.get(pos_char).unwrap())

        let dist = (value_map.get(cur_pos_char).unwrap() - value_map.get(pos_char).unwrap()).abs();
        dbg!(dist);
        if dist > 1 {
            return 999;
        } else {
            return 1;
        }
    };
    let result = dijkstra(
        &start_pos,
        |&(x, y)| {
            vec![(x, y + 1), (x - 1, y), (x + 1, y), (x, y - 1)]
                .into_iter()
                .map(move |p| (p, calc_weight((x, y), p)))
        },
        |&p| p == end_pos,
    );

    dbg!(result);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn find_start_end(input: &str) -> ((i32, i32), (i32, i32)) {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let map_size = (map.len() as i32, map[0].len() as i32);
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
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
