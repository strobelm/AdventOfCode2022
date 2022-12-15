use regex::Regex;

pub fn part_one(input: &str) -> Option<usize> {
    let y: i64 = 2000000;

    let sensors: Vec<Sensor> = parse_sensors(input);

    let lower = sensors.iter().map(|s| s.pos.x - s.manh_dist).min().unwrap();
    let upper = sensors.iter().map(|s| s.pos.x + s.manh_dist).max().unwrap();
    let res = (lower..=upper)
        .filter(|&x| sensors.iter().any(|s| s.is_inside_range(Point { x, y })))
        .count();

    Some(res)
}

pub fn part_two(input: &str) -> Option<i64> {
    let sensors: Vec<Sensor> = parse_sensors(input);
    let max: i64 = 4000000;

    sensors.iter().find_map(|s| {
        ((s.pos.x - s.manh_dist - 1).max(0)..=s.pos.x.min(max))
            .zip(s.pos.y..=max)
            .find_map(|p| {
                sensors
                    .iter()
                    .all(|s| !s.is_inside_range(Point { x: p.0, y: p.1 }))
                    .then_some(p.0 * 4000000 + p.1)
            })
    })
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}
#[derive(Debug, Clone, Copy)]
struct Sensor {
    pos: Point,
    closest_beacon: Point,
    manh_dist: i64,
}
impl Sensor {
    pub fn is_inside_range(&self, p: Point) -> bool {
        if self.closest_beacon == p {
            return false;
        }
        self.manh_dist
            >= (self.pos.x.abs_diff(p.x) + self.pos.y.abs_diff(p.y))
                .try_into()
                .unwrap()
    }
}

fn parse_sensors(input: &str) -> Vec<Sensor> {
    let re =
        Regex::new(r#"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"#)
            .unwrap();
    input
        .lines()
        .map(|l| {
            re.captures(l)
                .map(|c| {
                    let pos = Point {
                        x: c[1].parse().unwrap(),
                        y: c[2].parse().unwrap(),
                    };
                    let closest_beacon = Point {
                        x: c[3].parse().unwrap(),
                        y: c[4].parse().unwrap(),
                    };
                    let manh_dist = manhattan_dist(&pos, &closest_beacon);
                    Sensor {
                        pos,
                        closest_beacon,
                        manh_dist,
                    }
                })
                .unwrap()
        })
        .collect()
}

#[inline]
fn manhattan_dist(p: &Point, q: &Point) -> i64 {
    (p.x - q.x).abs() + (p.y - q.y).abs()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(0));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(28000022));
    }
}
