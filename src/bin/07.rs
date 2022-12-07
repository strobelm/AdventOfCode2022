pub fn part_one(input: &str) -> Option<u32> {
    let sum = parse(&mut input.lines())
        .into_iter()
        .filter(|&s| s <= 100_000)
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let filesystem_size = 70_000_000;
    let needed_size = 30_000_000;

    let mut subdirs = parse(&mut input.lines());
    subdirs.sort();

    let all_dir_sized_added_up = subdirs.pop().unwrap();
    let unused_space = filesystem_size - all_dir_sized_added_up;
    let least_amount_to_free = needed_size - unused_space;

    Some(
        *subdirs
            .iter()
            .find(|&d| d >= &least_amount_to_free)
            .unwrap(),
    )
}

fn parse<'a>(input: &mut impl Iterator<Item = &'a str>) -> Vec<u32> {
    let mut total = 0;
    let mut subdirs = vec![];
    loop {
        match input.next() {
            Some("$ cd ..") | None => break,
            Some(l) if l.starts_with("dir") || l.starts_with("$ ls") => (),
            Some(l) if l.starts_with("$ cd") => {
                subdirs.append(&mut parse(input));
                total += subdirs.last().unwrap();
            }
            Some(l) => {
                let (size, _) = l.split_once(' ').unwrap();
                total += size.parse::<u32>().unwrap();
            }
        }
    }
    subdirs.push(total);
    subdirs
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
