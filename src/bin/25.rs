use std::{array, time::Instant};

const INPUT: &str = include_str!("../../input/25.txt");

struct Schematics {
    locks: Box<[[u8; 5]]>,
    keys: Box<[[u8; 5]]>,
    height: u8,
}

fn solve_1(
    Schematics {
        locks,
        keys,
        height,
    }: Schematics,
) -> u32 {
    let mut count = 0;

    for key in &keys {
        for lock in &locks {
            if key.iter().zip(lock).all(|(k, l)| k + l <= height) {
                count += 1;
            }
        }
    }

    count
}

fn parse(input: &str) -> Schematics {
    let mut locks = vec![];
    let mut keys = vec![];

    let n = input.split_once("\n\n").unwrap().0.len() / 6;

    input.split("\n\n").for_each(|sch| {
        if sch.starts_with("#####") {
            locks.push(array::from_fn(|y| {
                (1..=n)
                    .map(|x| sch.as_bytes()[6 * x + y])
                    .position(|c| c == b'.')
                    .unwrap_or(n)
                    .try_into()
                    .unwrap()
            }));
        } else {
            keys.push(array::from_fn(|y| {
                (0..n)
                    .rev()
                    .map(|x| sch.as_bytes()[6 * x + y])
                    .position(|c| c == b'.')
                    .unwrap_or(n)
                    .try_into()
                    .unwrap()
            }));
        }
    });

    Schematics {
        keys: keys.into_boxed_slice(),
        locks: locks.into_boxed_slice(),
        height: input.lines().next().unwrap().len().try_into().unwrap(),
    }
}

fn main() {
    println!("Advent of Code 2024 - Day 25:");

    let now = Instant::now();
    let input = parse(INPUT);
    println!("- Parsed in {:#?}", now.elapsed());

    let now = Instant::now();
    let output = solve_1(input);
    println!("- Problem 1 solved in {:#?}: {output}", now.elapsed());
}

#[cfg(test)]
mod test {
    use crate::{parse, solve_1};

    const INPUT: &str = include_str!("../../input/25_test.txt");
    const RESULT_1: u32 = 3;

    #[test]
    fn test_1() {
        let input = parse(INPUT);
        let output = solve_1(input);
        assert_eq!(output, RESULT_1);
    }
}
