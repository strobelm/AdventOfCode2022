use core::cmp::{max, min};
use core::ops::Range;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let range_pairs = parse_ranges(input);
    let count = range_pairs
        .iter()
        .map(|t| ranges_contain_each_other(*t))
        .filter(|b| *b)
        .count();

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let range_pairs = parse_ranges(input);
    let number_of_intersections: u32 = range_pairs
        .iter()
        .map(|t| {
            let a = t.0.start..t.0.end;
            let b = t.1.start..t.1.end;
            intersect(&a, &b)
        })
        .filter(|r| !r.is_empty())
        .count() as u32;

    Some(number_of_intersections)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Copy, Clone, Debug)]
pub struct StartEnd {
    pub start: u32,
    pub end: u32,
}
pub type RangeTuple = (std::ops::Range<u32>, std::ops::Range<u32>);
fn parse_ranges(input: &str) -> Vec<(StartEnd, StartEnd)> {
    let mut vec = vec![];
    for line in input.lines() {
        let records = line.split(',');
        let pairs: _ = records
            .map(|r| {
                let start_end: _ = r
                    .split('-')
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let (start, end) = start_end.iter().copied().collect_tuple().unwrap();

                StartEnd { start, end }
            })
            .collect::<Vec<StartEnd>>();

        vec.push((pairs[0], pairs[1]))
    }
    vec
}

pub fn ranges_contain_each_other(tuple: (StartEnd, StartEnd)) -> bool {
    let (fst, scd) = tuple;
    let fst_range = fst.start..fst.end + 1;
    let scd_range = scd.start..scd.end + 1;

    let fst_contains_scd = fst_range.contains(&scd.start) && fst_range.contains(&scd.end);
    let scd_contains_fst = scd_range.contains(&fst.start) && scd_range.contains(&fst.end);

    fst_contains_scd || scd_contains_fst
}

fn intersect(a: &Range<u32>, b: &Range<u32>) -> Range<u32> {
    let start = max(a.start, b.start);
    let end = min(a.end, b.end);

    start..end + 1
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
