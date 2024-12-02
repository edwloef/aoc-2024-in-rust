use itertools::Itertools as _;
use std::time::Instant;

const INPUT: &str = include_str!("../../input/02.txt");

fn solve_1(input: &[Box<[u32]>]) -> usize {
    input.iter().filter(|a| safe(a.iter().copied())).count()
}

fn solve_2(input: &[Box<[u32]>]) -> usize {
    input
        .iter()
        .filter(|a| (0..a.len()).any(|b| safe(a[..b].iter().chain(&a[b + 1..]).copied())))
        .count()
}

fn safe(report: impl Iterator<Item = u32>) -> bool {
    let mut increasing = None;

    report.tuple_windows().all(|(a, b)| {
        if a.abs_diff(b) > 3 {
            return false;
        }

        if *increasing.get_or_insert(a > b) {
            if a <= b {
                return false;
            }
        } else if a >= b {
            return false;
        }

        true
    })
}

fn parse(input: &str) -> Box<[Box<[u32]>]> {
    input
        .lines()
        .map(|a| {
            a.split_ascii_whitespace()
                .map(|a| a.parse().unwrap())
                .collect()
        })
        .collect()
}

fn main() {
    println!("Advent of Code 2024 - Day 02:");

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

    const INPUT: &str = include_str!("../../input/02_test.txt");
    const RESULT_1: usize = 2;
    const RESULT_2: usize = 4;

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
