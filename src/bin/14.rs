use std::collections::HashSet;

use itertools::Itertools;
use regex::{self, Regex};

pub fn part_one(input: &str) -> Option<u32> {
    let use_floor = false;
    let cave = Cave::new(input, use_floor);
    Some(cave.into_iter().last().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let use_floor = true;
    let cave = Cave::new(input, use_floor);
    Some(cave.into_iter().last().unwrap())
}

type Coord = (i32, i32);
#[derive(Debug)]
struct Cave {
    obstacles: HashSet<Coord>,
    max_depth: i32,
    sand: Sand,
    sand_counter: u32,
    use_floor: bool,
}

impl Cave {
    fn new(input: &str, use_floor: bool) -> Self {
        let re = Regex::new(r#"(\d+),(\d+)"#).unwrap();
        let obstacles: HashSet<(i32, i32)> = HashSet::from_iter(input.lines().flat_map(|l| {
            re.captures_iter(l)
                .map(|cap| {
                    (
                        cap[1].parse::<i32>().unwrap(),
                        cap[2].parse::<i32>().unwrap(),
                    )
                })
                .tuple_windows()
                .flat_map(|(a, b)| {
                    let x_range = if (a.0..=b.0).is_empty() {
                        b.0..=a.0
                    } else {
                        a.0..=b.0
                    };
                    let y_range = if (a.1..=b.1).is_empty() {
                        b.1..=a.1
                    } else {
                        a.1..=b.1
                    };
                    x_range.cartesian_product(y_range)
                })
                .collect::<Vec<(i32, i32)>>()
        }));

        let max_depth = obstacles.iter().map(|c| c.1).max().unwrap();

        let sand = Sand { pos: (500, 0) };

        Cave {
            obstacles,
            max_depth,
            sand,
            sand_counter: 0,
            use_floor,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Sand {
    pos: Coord,
}

impl Iterator for Cave {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let Cave {
            sand,
            obstacles,
            max_depth,
            sand_counter,
            use_floor,
        } = self;

        let sand_pos = sand.pos;
        let new_sand_posses = vec![
            (sand_pos.0, sand_pos.1 + 1),
            (sand_pos.0 - 1, sand_pos.1 + 1),
            (sand_pos.0 + 1, sand_pos.1 + 1),
        ];

        let floor_y = *max_depth + 2;

        let new_sand_pos = new_sand_posses
            .iter()
            .filter(|p| p.1 < floor_y)
            .find(|el| !obstacles.contains(el));

        if obstacles.contains(&(500, 0)) {
            return None;
        }
        match new_sand_pos {
            None => {
                obstacles.insert(self.sand.pos);
                *sand_counter += 1;
                self.sand = Sand { pos: (500, 0) };
            }
            Some(new_sand_pos) => {
                if !*use_floor && new_sand_pos.1 > *max_depth {
                    return None;
                }
                self.sand = Sand { pos: *new_sand_pos };
            }
        }

        Some(self.sand_counter)
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
