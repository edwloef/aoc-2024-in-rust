use arrayvec::ArrayVec;
use itertools::Itertools as _;
use std::{iter::once, time::Instant};

const INPUT: &str = include_str!("../../input/10.txt");

fn solve_1(map: &[Box<[u8]>]) -> usize {
    map.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .copied()
                .enumerate()
                .filter(|&(_, c)| c == 0)
                .map(|(x, _)| trails(map, (x, y)).sorted_unstable().dedup().count())
                .sum::<usize>()
        })
        .sum()
}

fn solve_2(map: &[Box<[u8]>]) -> usize {
    map.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .copied()
                .enumerate()
                .filter(|&(_, c)| c == 0)
                .map(|(x, _)| trails(map, (x, y)).count())
                .sum::<usize>()
        })
        .sum()
}

fn trails(
    map: &[Box<[u8]>],
    (x, y): (usize, usize),
) -> impl Iterator<Item = (usize, usize)> + use<'_> {
    once((x, y))
        .flat_map(|a| adjacent(map, a, 1))
        .flat_map(|a| adjacent(map, a, 2))
        .flat_map(|a| adjacent(map, a, 3))
        .flat_map(|a| adjacent(map, a, 4))
        .flat_map(|a| adjacent(map, a, 5))
        .flat_map(|a| adjacent(map, a, 6))
        .flat_map(|a| adjacent(map, a, 7))
        .flat_map(|a| adjacent(map, a, 8))
        .flat_map(|a| adjacent(map, a, 9))
}

fn adjacent(map: &[Box<[u8]>], (x, y): (usize, usize), val: u8) -> ArrayVec<(usize, usize), 4> {
    let mut result = ArrayVec::new();

    if x > 0 && map[y][x - 1] == val {
        result.push((x - 1, y));
    }

    if x < map[y].len() - 1 && map[y][x + 1] == val {
        result.push((x + 1, y));
    }

    if y > 0 && map[y - 1][x] == val {
        result.push((x, y - 1));
    }

    if y < map.len() - 1 && map[y + 1][x] == val {
        result.push((x, y + 1));
    }

    result
}

fn parse(input: &str) -> Box<[Box<[u8]>]> {
    input
        .lines()
        .map(|line| line.bytes().map(|c| c - b'0').collect())
        .collect()
}

fn main() {
    println!("Advent of Code 2024 - Day 10:");

    let now = Instant::now();
    let input = parse(INPUT);
    println!("- Parsed in {:#?}", now.elapsed());

    let now = Instant::now();
    let output = solve_1(&input);
    println!("- Problem 1 solved in {:#?}: {output}", now.elapsed());

    let now = Instant::now();
    let output = solve_2(&input);
    println!("- Problem 2 solved in {:#?}: {output}", now.elapsed());
}

#[cfg(test)]
mod test {
    use crate::{parse, solve_1, solve_2};

    const INPUT: &str = include_str!("../../input/10_test.txt");
    const RESULT_1: usize = 36;
    const RESULT_2: usize = 81;

    #[test]
    fn test_1() {
        let input = parse(INPUT);
        let output = solve_1(&input);
        assert_eq!(output, RESULT_1);
    }

    #[test]
    fn test_2() {
        let input = parse(INPUT);
        let output = solve_2(&input);
        assert_eq!(output, RESULT_2);
    }
}
