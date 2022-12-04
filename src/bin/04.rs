use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let count = input
        .lines()
        .filter(|line| {
            let (a, b) = line.split(',').map(MyRange::from).next_tuple().unwrap();
            a.is_subrange(b) || b.is_subrange(a)
        })
        .count();
    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let number_of_intersections = input
        .lines()
        .filter(|line| {
            let (a, b) = line.split(',').map(MyRange::from).next_tuple().unwrap();
            a.is_overlapping(b) || b.is_overlapping(a)
        })
        .count();
    Some(number_of_intersections as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Copy, Clone, Debug)]
struct MyRange {
    start: usize,
    end: usize,
}

impl MyRange {
    fn is_overlapping(self, other: MyRange) -> bool {
        self.start >= other.start && self.start <= other.end
            || self.end >= other.start && self.end <= other.end
    }

    fn is_subrange(self, other: MyRange) -> bool {
        self.start >= other.start && self.end <= other.end
    }
}

impl From<&str> for MyRange {
    fn from(input: &str) -> Self {
        let (start, end) = input.split_once('-').unwrap();
        MyRange {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
