use std::time::Instant;

const INPUT: &str = include_str!("../../input/20.txt");

fn solve_1(input: &[Box<[u8]>], savings: usize) -> usize {
    solve(input, 2, savings)
}

fn solve_2(input: &[Box<[u8]>], savings: usize) -> usize {
    solve(input, 20, savings)
}

fn solve(input: &[Box<[u8]>], depth: usize, savings: usize) -> usize {
    let distances = build_distances(
        input,
        input
            .iter()
            .enumerate()
            .find_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .find(|&(_, &c)| c == b'E')
                    .map(|(x, _)| (x, y))
            })
            .unwrap(),
    );

    distances
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(x, d1)| d1.map(|d1| cheat(&distances, (x, y), d1, depth, savings)))
                .sum::<usize>()
        })
        .sum()
}

fn build_distances(map: &[Box<[u8]>], (x, y): (usize, usize)) -> Box<[Box<[Option<usize>]>]> {
    let mut distances =
        vec![vec![None; map[0].len()].into_boxed_slice(); map.len()].into_boxed_slice();
    distances[y][x] = Some(0);

    let mut pos = (x, y);

    while let Some(new_pos) = next(map, &distances, pos) {
        distances[new_pos.1][new_pos.0] = Some(distances[pos.1][pos.0].unwrap() + 1);
        pos = new_pos;
    }

    distances
}

fn next(
    map: &[Box<[u8]>],
    distances: &[Box<[Option<usize>]>],
    (x, y): (usize, usize),
) -> Option<(usize, usize)> {
    if x != 0 && map[y][x - 1] != b'#' && distances[y][x - 1].is_none() {
        return Some((x - 1, y));
    }

    if y != 0 && map[y - 1][x] != b'#' && distances[y - 1][x].is_none() {
        return Some((x, y - 1));
    }

    if x != map[y].len() - 1 && map[y][x + 1] != b'#' && distances[y][x + 1].is_none() {
        return Some((x + 1, y));
    }

    if y != map.len() - 1 && map[y + 1][x] != b'#' && distances[y + 1][x].is_none() {
        return Some((x, y + 1));
    }

    None
}

fn cheat(
    distances: &[Box<[Option<usize>]>],
    (sx, sy): (usize, usize),
    d1: usize,
    depth: usize,
    savings: usize,
) -> usize {
    distances
        .iter()
        .enumerate()
        .take(sy + depth + 1)
        .skip(sy.saturating_sub(depth))
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .take(sx + depth + 1 - sy.abs_diff(y))
                .skip((sx + sy.abs_diff(y)).saturating_sub(depth))
                .filter_map(|(x, d2)| d2.map(|c| (x, c)))
                .filter(|&(x, d2)| d1 > d2 && d1 - d2 >= savings + sx.abs_diff(x) + sy.abs_diff(y))
                .count()
        })
        .sum()
}

fn parse(input: &str) -> Box<[Box<[u8]>]> {
    input.lines().map(|line| line.as_bytes().into()).collect()
}

fn main() {
    println!("Advent of Code 2024 - Day 20:");

    let now = Instant::now();
    let input = parse(INPUT);
    println!("- Parsed in {:#?}", now.elapsed());

    let now = Instant::now();
    let output = solve_1(&input, 100);
    println!("- Problem 1 solved in {:#?}: {output}", now.elapsed());

    let now = Instant::now();
    let output = solve_2(&input, 100);
    println!("- Problem 2 solved in {:#?}: {output}", now.elapsed());
}

#[cfg(test)]
mod test {
    use crate::{parse, solve_1, solve_2};

    const INPUT: &str = include_str!("../../input/20_test.txt");
    const RESULT_1: usize = 5;
    const RESULT_2: usize = 29;

    #[test]
    fn test_1() {
        let input = parse(INPUT);
        let output = solve_1(&input, 20);
        assert_eq!(output, RESULT_1);
    }

    #[test]
    fn test_2() {
        let input = parse(INPUT);
        let output = solve_2(&input, 72);
        assert_eq!(output, RESULT_2);
    }
}
