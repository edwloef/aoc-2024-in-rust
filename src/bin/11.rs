use ahash::{HashMap, HashMapExt as _};
use memoize::memoize;
use std::time::Instant;

const INPUT: &str = include_str!("../../input/11.txt");

fn solve_1(input: &[u64]) -> u64 {
    input.iter().map(|&a| recursive(a, 25)).sum()
}

fn solve_2(input: &[u64]) -> u64 {
    input.iter().map(|&a| recursive(a, 75)).sum()
}

#[memoize(CustomHasher: HashMap)]
fn recursive(s: u64, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    if s == 0 {
        recursive(1, depth - 1)
    } else {
        let digits = s.ilog10() + 1;
        if digits % 2 == 0 {
            let factor = 10u64.pow(digits / 2);
            recursive(s / factor, depth - 1) + recursive(s % factor, depth - 1)
        } else {
            recursive(s * 2024, depth - 1)
        }
    }
}

fn parse(input: &str) -> Vec<u64> {
    input
        .trim()
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn main() {
    println!("Advent of Code 2024 - Day 11:");

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

    const INPUT: &str = include_str!("../../input/11_test.txt");
    const RESULT_1: u64 = 55312;
    const RESULT_2: u64 = 65_601_038_650_482;

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
