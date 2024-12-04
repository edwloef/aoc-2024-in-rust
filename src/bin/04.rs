use std::time::Instant;

const INPUT: &str = include_str!("../../input/04.txt");

fn solve_1(input: &[Box<[u8]>]) -> u32 {
    let max_x = input[0].len();
    let max_y = input.len();

    input
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|&(_, &c)| c == b'X')
                .map(|(x, _)| {
                    let mut count = 0;

                    let left = x >= 3;
                    let right = x + 3 < max_x;
                    let up = y >= 3;
                    let down = y + 3 < max_y;

                    if up {
                        if (input[y - 1][x], input[y - 2][x], input[y - 3][x]) == (b'M', b'A', b'S')
                        {
                            count += 1;
                        }

                        if left
                            && (
                                input[y - 1][x - 1],
                                input[y - 2][x - 2],
                                input[y - 3][x - 3],
                            ) == (b'M', b'A', b'S')
                        {
                            count += 1;
                        }

                        if right
                            && (
                                input[y - 1][x + 1],
                                input[y - 2][x + 2],
                                input[y - 3][x + 3],
                            ) == (b'M', b'A', b'S')
                        {
                            count += 1;
                        }
                    }

                    if down {
                        if (input[y + 1][x], input[y + 2][x], input[y + 3][x]) == (b'M', b'A', b'S')
                        {
                            count += 1;
                        }

                        if left
                            && (
                                input[y + 1][x - 1],
                                input[y + 2][x - 2],
                                input[y + 3][x - 3],
                            ) == (b'M', b'A', b'S')
                        {
                            count += 1;
                        }

                        if right
                            && (
                                input[y + 1][x + 1],
                                input[y + 2][x + 2],
                                input[y + 3][x + 3],
                            ) == (b'M', b'A', b'S')
                        {
                            count += 1;
                        }
                    }

                    if left
                        && (input[y][x - 1], input[y][x - 2], input[y][x - 3]) == (b'M', b'A', b'S')
                    {
                        count += 1;
                    }

                    if right
                        && (input[y][x + 1], input[y][x + 2], input[y][x + 3]) == (b'M', b'A', b'S')
                    {
                        count += 1;
                    }

                    count
                })
                .sum::<u32>()
        })
        .sum()
}

fn solve_2(input: &[Box<[u8]>]) -> usize {
    let max_x = input[0].len();
    let max_y = input.len();

    input[..max_y - 1]
        .iter()
        .enumerate()
        .skip(1)
        .map(|(y, line)| {
            line[..max_x - 1]
                .iter()
                .enumerate()
                .skip(1)
                .filter(|&(_, &c)| c == b'A')
                .filter(|(x, _)| {
                    ((input[y - 1][x - 1], input[y + 1][x + 1]) == (b'M', b'S')
                        || (input[y + 1][x + 1], input[y - 1][x - 1]) == (b'M', b'S'))
                        && ((input[y + 1][x - 1], input[y - 1][x + 1]) == (b'M', b'S')
                            || (input[y - 1][x + 1], input[y + 1][x - 1]) == (b'M', b'S'))
                })
                .count()
        })
        .sum()
}

fn parse(input: &str) -> Box<[Box<[u8]>]> {
    input.lines().map(|a| a.as_bytes().into()).collect()
}

fn main() {
    println!("Advent of Code 2024 - Day 04:");

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

    const INPUT: &str = include_str!("../../input/04_test.txt");
    const RESULT_1: u32 = 18;
    const RESULT_2: usize = 9;

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
