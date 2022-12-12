use pathfinding::prelude::{bfs, Matrix};
use std::str::FromStr;

pub fn part_one(input: &str) -> Option<u32> {
    let problem = input.parse::<PathFinding>().ok()?;
    let start = &problem.start;
    let end = &problem.end;
    let mat = &problem.mat;

    let result = bfs(
        start,
        |&p| {
            mat.neighbours(p, false)
                .filter(move |&q| mat[q] <= mat[p] + 1)
        },
        |&p| p == *end,
    )?
    .len()
        - 1;

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let problem = input.parse::<PathFinding>().ok()?;
    let end = &problem.end;
    let mat = &problem.mat;

    let result = bfs(
        end,
        |&p| {
            mat.neighbours(p, false)
                .filter(move |&q| mat[p] <= mat[q] + 1)
        },
        |&i| mat[i] == b'a',
    )?
    .len()
        - 1;
    Some(result as u32)
}

struct PathFinding {
    mat: Matrix<u8>,
    start: (usize, usize),
    end: (usize, usize),
}

impl FromStr for PathFinding {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut mat = Matrix::from_rows(input.lines().map(str::bytes)).unwrap();

        let start = mat.indices().find(|p| mat[*p] == b'S').unwrap();
        let end = mat.indices().find(|p| mat[*p] == b'E').unwrap();

        mat[start] = b'a';
        mat[end] = b'z';

        Ok(PathFinding { mat, start, end })
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
