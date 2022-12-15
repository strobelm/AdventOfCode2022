use regex::Regex;

pub fn part_one(input: &str) -> Option<usize> {
    let y = 2000000;
    let sensors: Vec<Sensor> = parse_sensors(input);
    let beacons_in_row: Vec<Point> = sensors
        .iter()
        .map(|s| s.closest_beacon)
        .filter(|b| b.y == y)
        .collect();

    let coords = -9000000..9000000;
    let mut covered: Vec<i32> = vec![];
    'outer: for c in coords {
        for i in 0..sensors.len() {
            let sensor = sensors[i];
            if manhattan_dist(&sensor.pos, &Point { x: c, y }) <= sensor.manh_dist {
                covered.push(c);
                continue 'outer;
            }
        }
    }

    Some(covered.len() - beacons_in_row.len())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Debug, Clone, Copy)]
struct Sensor {
    pos: Point,
    closest_beacon: Point,
    manh_dist: i32,
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
fn manhattan_dist(p: &Point, q: &Point) -> i32 {
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
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
