use std::time::Instant;

const INPUT: &str = include_str!("../../input/XX.txt");

fn solve_1(input: impl Iterator<Item = u32>) -> u32 {
    input.sum()
}

fn solve_2(input: impl Iterator<Item = u32>) -> u32 {
    input.sum()
}

fn parse(input: &str) -> Box<[u32]> {
    input.lines().map(|_| 0).collect()
}

fn main() {
    println!("Advent of Code 2024 - Day XX:");

    let now = Instant::now();
    let input = parse(INPUT);
    println!("- Parsed in {:#?}", now.elapsed());

    let now = Instant::now();
    let output = solve_1(input.iter().copied());
    println!("- Problem 1 solved in {:#?}: {output}", now.elapsed());

    let now = Instant::now();
    let output = solve_2(input.iter().copied());
    println!("- Problem 2 solved in {:#?}: {output}", now.elapsed());
}

#[cfg(test)]
mod test {
    use crate::{parse, solve_1, solve_2};

    const INPUT: &str = include_str!("../../input/XX_test.txt");
    const RESULT_1: u32 = 0;
    const RESULT_2: u32 = 0;

    #[test]
    fn test_1() {
        let input = parse(INPUT);
        let output = solve_1(input.iter().copied());
        assert_eq!(output, RESULT_1);
    }

    #[test]
    fn test_2() {
        let input = parse(INPUT);
        let output = solve_2(input.iter().copied());
        assert_eq!(output, RESULT_2);
    }
}
