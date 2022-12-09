use std::{
    collections::{HashSet, VecDeque},
    ops,
    str::FromStr,
    string,
};

pub fn part_one(input: &str) -> Option<u32> {
    Some(calc_n_visisted::<1>(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(calc_n_visisted::<9>(input))
}

fn calc_n_visisted<const N: usize>(input: &str) -> u32 {
    let mut head = Head::new(input);
    let mut tails = vec![Tail::new(); N];

    while !head.moves.is_empty() {
        let mut pos = head.execute_move();
        for tail in tails.iter_mut() {
            pos = tail.follow(&pos);
        }
    }

    tails.last().unwrap().visited.len() as u32
}

#[derive(Debug, Clone)]
struct Head {
    pos: Coord,
    moves: VecDeque<Direction>,
}

impl Head {
    fn new(input: &str) -> Self {
        let pos = Coord { x: 0, y: 0 };
        let moves = input
            .lines()
            .flat_map(|l| {
                let (d, n) = l
                    .split_once(' ')
                    .map(|(d, n)| (d.parse::<Direction>().unwrap(), n.parse::<i8>().unwrap()))
                    .unwrap();
                vec![d; n as usize]
            })
            .collect();
        Head { pos, moves }
    }

    fn execute_move(&mut self) -> Coord {
        if let Some(mv) = self.moves.pop_front() {
            match mv {
                Direction::Up => self.pos = self.pos + Coord { x: 0, y: 1 },
                Direction::Down => self.pos = self.pos + Coord { x: 0, y: -1 },
                Direction::Right => self.pos = self.pos + Coord { x: 1, y: 0 },
                Direction::Left => self.pos = self.pos + Coord { x: -1, y: 0 },
            }
        }

        self.pos
    }
}

#[derive(Debug, Clone)]
struct Tail {
    pos: Coord,
    visited: HashSet<Coord>,
}

impl Tail {
    fn new() -> Self {
        let pos = Coord { x: 0, y: 0 };
        Tail {
            pos,
            visited: HashSet::from([pos]),
        }
    }
    fn follow(&mut self, head_pos: &Coord) -> Coord {
        let dir = *head_pos - self.pos;
        let is_touching = dir.x.abs() <= 1 && dir.y.abs() <= 1;

        match is_touching {
            true => self.pos,
            _ => {
                let x = dir.x.signum() * dir.x.abs().clamp(0, 1);
                let y = dir.y.signum() * dir.y.abs().clamp(0, 1);

                self.pos = self.pos + Coord { x, y };
                self.visited.insert(self.pos);
                self.pos
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Coord {
    x: i8,
    y: i8,
}
impl ops::Add<Coord> for Coord {
    type Output = Coord;
    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl ops::Sub<Coord> for Coord {
    type Output = Coord;
    fn sub(self, other: Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
