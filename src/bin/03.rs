use regex::Regex;
use std::time::Instant;

const INPUT: &str = include_str!("../../input/03.txt");

fn solve_1(input: impl Iterator<Item = (u32, u32, bool)>) -> u32 {
    input.map(|(a, b, _)| a * b).sum()
}

fn solve_2(input: &[(u32, u32, bool)]) -> u32 {
    solve_1(input.iter().copied().filter(|&(_, _, a)| a))
}

fn parse(input: &str) -> Box<[(u32, u32, bool)]> {
    let mut last = true;

    Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)")
        .unwrap()
        .captures_iter(input)
        .filter_map(|a| match &a[0] {
            "do()" => {
                last = true;
                None
            }
            "don't()" => {
                last = false;
                None
            }
            _ => Some((a[1].parse().unwrap(), a[2].parse().unwrap(), last)),
        })
        .collect()
}

fn main() {
    println!("Advent of Code 2024 - Day 03:");

    let now = Instant::now();
    let input = parse(INPUT);
    println!("- Parsed in {:#?}", now.elapsed());

    let now = Instant::now();
    let output = solve_1(input.iter().copied());
    println!("- Problem 1 solved in {:#?}: {output}", now.elapsed());

    let now = Instant::now();
    let output = solve_2(&input);
    println!("- Problem 2 solved in {:#?}: {output}", now.elapsed());
}

#[cfg(test)]
mod test {
    use crate::{parse, solve_1, solve_2};

    const INPUT: &str = include_str!("../../input/03_test.txt");
    const RESULT_1: u32 = 161;
    const RESULT_2: u32 = 48;

    #[test]
    fn test_1() {
        let input = parse(INPUT);
        let output = solve_1(input.iter().copied());
        assert_eq!(output, RESULT_1);
    }

    #[test]
    fn test_2() {
        let input = parse(INPUT);
        let output = solve_2(&input);
        assert_eq!(output, RESULT_2);
    }
}
