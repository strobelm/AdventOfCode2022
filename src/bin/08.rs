pub fn part_one(input: &str) -> Option<u32> {
    let mat = build_matrix(input.trim());
    let res: usize = mat
        .iter()
        .enumerate()
        .map(|(j, r)| {
            r.iter()
                .enumerate()
                .map(|(i, _)| match (i, j) {
                    (_, j) if j == 0 || j == mat.len() - 1 => true,
                    _ => is_visible(r.to_vec(), i) || is_visible(get_column(mat.clone(), i), j),
                })
                .filter(|b| *b)
                .count()
        })
        .sum();
    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mat = build_matrix(input.trim());
    let res: Vec<Vec<((usize, usize), (usize, usize))>> = mat
        .iter()
        .enumerate()
        .map(|(j, r)| {
            r.iter()
                .enumerate()
                .map(|(i, _)| match (i, j) {
                    (_, j) if j == 0 || j == mat.len() - 1 => ((0, 0), (0, 0)),
                    _ => (
                        get_view(r.to_vec(), i),
                        get_view(get_column(mat.clone(), i), j),
                    ),
                })
                .collect()
        })
        .collect();
    // .collect::<Vec<u32>>();
    dbg!(res);
    // .product();
    // Some(res as u32)
    None
}

type Matrix = Vec<Vec<u32>>;
fn get_column(mat: Matrix, j: usize) -> Vec<u32> {
    mat.iter().map(|v| v[j]).collect()
}

fn build_matrix(input: &str) -> Matrix {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn is_visible(input: Vec<u32>, idx: usize) -> bool {
    let el: u32 = input[idx];
    if idx == 0 || idx == input.len() - 1 {
        return true;
    }

    let lower_visible = input[..idx].iter().all(|t| *t < el);
    let upper_visible = input[idx + 1..].iter().all(|t| *t < el);

    lower_visible || upper_visible
}

fn get_view(input: Vec<u32>, idx: usize) -> (usize, usize) {
    let el: u32 = input[idx];
    if idx == 0 || idx == input.len() - 1 {
        return (0, 0);
    }

    let lower_visible = input[..idx]
        .iter()
        .rev()
        .enumerate()
        .find(|(_, t)| *t >= &el)
        .map(|(i, _)| i + 1)
        // .unwrap_or(99);
        .unwrap_or(idx);

    let upper_visible = input[idx + 1..]
        .iter()
        .enumerate()
        .find(|(_, t)| *t >= &el)
        .map(|(i, _)| i + 1)
        // .unwrap_or(99);
        .unwrap_or(input.len() - idx - 1);

    // dbg!(lower_visible);
    // dbg!(upper_visible);
    // 1
    // let upper_visible = input[idx + 1..].iter().all(|t| *t < el);

    // lower_visible || upper_visible
    // if el == 5 && idx == 2 {
    if el == 4 {
        dbg!(input);
        dbg!((idx, el));
        dbg!((lower_visible, upper_visible));
    }
    (lower_visible, upper_visible)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
