use ahash::{HashMap, HashMapExt as _};
use memoize::memoize;
use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator as _};
use std::time::Instant;

const INPUT: &str = include_str!("../../input/19.txt");

struct Towels<'a> {
    towels: Box<[&'a str]>,
    designs: Box<[&'a str]>,
}

fn solve_1(input: &'static Towels<'_>) -> usize {
    input
        .designs
        .par_iter()
        .filter(|&design| possible(design, &input.towels))
        .count()
}

fn solve_2(input: &'static Towels<'_>) -> u64 {
    input
        .designs
        .par_iter()
        .map(|design| count(design, &input.towels))
        .sum()
}

#[memoize(CustomHasher: HashMap, Ignore: towels)]
fn possible(design: &'static str, towels: &[&str]) -> bool {
    design.is_empty()
        || towels
            .iter()
            .filter_map(|towel| design.strip_prefix(towel))
            .any(|design| possible(design, towels))
}

#[memoize(CustomHasher: HashMap, Ignore: towels)]
fn count(design: &'static str, towels: &[&str]) -> u64 {
    if design.is_empty() {
        return 1;
    }

    towels
        .iter()
        .filter_map(|towel| design.strip_prefix(towel))
        .map(|design| count(design, towels))
        .sum()
}

fn parse(input: &str) -> &'static Towels<'_> {
    let (towels, designs) = input.split_once("\n\n").unwrap();

    let towels = towels.split(", ").collect();
    let designs = designs.lines().collect();

    Box::leak(Box::new(Towels { towels, designs }))
}

fn main() {
    println!("Advent of Code 2024 - Day 19:");

    let now = Instant::now();
    let input = parse(INPUT);
    println!("- Parsed in {:#?}", now.elapsed());

    let now = Instant::now();
    let output = solve_1(input);
    println!("- Problem 1 solved in {:#?}: {output}", now.elapsed());

    let now = Instant::now();
    let output = solve_2(input);
    println!("- Problem 2 solved in {:#?}: {output}", now.elapsed());
}

#[cfg(test)]
mod test {
    use crate::{parse, solve_1, solve_2};

    const INPUT: &str = include_str!("../../input/19_test.txt");
    const RESULT_1: usize = 6;
    const RESULT_2: u64 = 16;

    #[test]
    fn test_1() {
        let input = parse(INPUT);
        let output = solve_1(input);
        assert_eq!(output, RESULT_1);
    }

    #[test]
    fn test_2() {
        let input = parse(INPUT);
        let output = solve_2(input);
        assert_eq!(output, RESULT_2);
    }
}
