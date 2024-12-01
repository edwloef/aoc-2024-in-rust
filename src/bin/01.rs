use ahash::{HashMap, HashMapExt as _};
use std::time::Instant;

const INPUT: &str = include_str!("../../input/01.txt");

fn solve_1(input: &[(u32, u32)]) -> u32 {
    let mut v1 = Vec::with_capacity(input.len());
    let mut v2 = Vec::with_capacity(input.len());

    for (a, b) in input {
        v1.push(a);
        v2.push(b);
    }

    v1.sort_unstable();
    v2.sort_unstable();

    v1.iter().zip(v2).map(|(&a, &b)| a.abs_diff(b)).sum()
}

fn solve_2(input: &[(u32, u32)]) -> u32 {
    let mut v = Vec::with_capacity(input.len());
    let mut c = HashMap::with_capacity(input.len());

    for (a, b) in input {
        v.push(a);
        c.entry(b).and_modify(|a| *a += 1).or_insert(1);
    }

    v.iter()
        .filter(|&a| c.contains_key(a))
        .map(|&a| a * c[a])
        .sum()
}

fn parse(input: &str) -> Box<[(u32, u32)]> {
    input
        .lines()
        .map(|a| a.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect()
}

fn main() {
    println!("Advent of Code 2024 - Day 01:");

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

    const INPUT: &str = include_str!("../../input/01_test.txt");
    const RESULT_1: u32 = 11;
    const RESULT_2: u32 = 31;

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
