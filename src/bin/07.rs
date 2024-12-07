use std::time::Instant;

const INPUT: &str = include_str!("../../input/07.txt");

fn solve_1(input: &[(u64, Box<[u64]>)]) -> u64 {
    input
        .iter()
        .filter(|line| is_solveable(line.0, &line.1, false))
        .map(|line| line.0)
        .sum()
}

fn solve_2(input: &[(u64, Box<[u64]>)]) -> u64 {
    input
        .iter()
        .filter(|line| is_solveable(line.0, &line.1, true))
        .map(|line| line.0)
        .sum()
}

fn is_solveable(so_far: u64, nums: &[u64], concat: bool) -> bool {
    if nums.is_empty() {
        return so_far == 0;
    }

    let last = *nums.last().unwrap();
    if so_far < last {
        return false;
    }

    let l = nums.len() - 1;

    if concat && l > 0 {
        let digits = digits(last);
        if (so_far - last) % digits == 0 && is_solveable(so_far / digits, &nums[..l], concat) {
            return true;
        }
    }

    is_solveable(so_far - nums.last().unwrap(), &nums[..l], concat)
        || (so_far % nums.last().unwrap() == 0
            && is_solveable(so_far / nums.last().unwrap(), &nums[..l], concat))
}

fn digits(a: u64) -> u64 {
    match a {
        _ if a < 10 => 10,
        _ if a < 100 => 100,
        _ => 1000,
    }
}

fn parse(input: &str) -> Box<[(u64, Box<[u64]>)]> {
    input
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(a, b)| {
            (
                a.parse().unwrap(),
                b.split_ascii_whitespace()
                    .map(|a| a.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn main() {
    println!("Advent of Code 2024 - Day 07:");

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

    const INPUT: &str = include_str!("../../input/07_test.txt");
    const RESULT_1: u64 = 3749;
    const RESULT_2: u64 = 11387;

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
