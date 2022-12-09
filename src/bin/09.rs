use std::{collections::HashSet, ops, str::FromStr, string};

pub fn part_one(input: &str) -> Option<u32> {
    let mut head = Head::new(input);
    let mut tail = Tail::new();

    for i in 0..head.moves.len() {
        let pos = head.execute_move(head.moves[i]);
        tail.follow(&pos);
    }

    Some(tail.visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut head = Head::new(input);
    let mut tails = vec![Tail::new(); 9];

    for i in 0..head.moves.len() {
        let mut pos = head.execute_move(head.moves[i]);
        for tail in tails.iter_mut() {
            pos = tail.follow(&pos);
        }
    }

    Some(tails.last().unwrap().visited.len() as u32)
}

#[derive(Debug, Clone)]
struct Head {
    pos: Coord,
    moves: Vec<Direction>,
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

    fn execute_move(&mut self, mv: Direction) -> Coord {
        match mv {
            Direction::Up => self.pos = self.pos + Coord { x: 0, y: 1 },
            Direction::Down => self.pos = self.pos + Coord { x: 0, y: -1 },
            Direction::Right => self.pos = self.pos + Coord { x: 1, y: 0 },
            Direction::Left => self.pos = self.pos + Coord { x: -1, y: 0 },
        }

        self.get_pos()
    }

    fn get_pos(&self) -> Coord {
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
                let x = if dir.x == 0 { 0 } else { dir.x / dir.x.abs() };
                let y = if dir.y == 0 { 0 } else { dir.y / dir.y.abs() };

                let new_pos = self.pos + Coord { x, y };
                self.visited.insert(new_pos);
                self.pos = new_pos;

                new_pos
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
