use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator as _};
use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Instant,
};

const INPUT: &str = include_str!("../../input/22.txt");

fn solve_1(input: &[u64]) -> u64 {
    input
        .par_iter()
        .copied()
        .map(|mut num| {
            for _ in 0..2000 {
                num = next_secret_number(num);
            }
            num
        })
        .sum()
}

fn solve_2(input: &[u64]) -> u64 {
    let slice: Arc<_> = (0..19usize.pow(4)).map(|_| AtomicU64::new(0)).collect();

    input
        .par_iter()
        .copied()
        .map(|num| (num, slice.clone()))
        .for_each(|(mut num, slice)| {
            let mut sequence = [0u64; (19usize.pow(4) >> 6) + 1];
            let mut last_4 = 0;
            let mut old_price = num % 10;

            for i in 0..2000 {
                let new = next_secret_number(num);
                let new_price = new % 10;

                let diff: usize = ((old_price + 9) - new_price).try_into().unwrap();
                last_4 = (last_4 * 19 + diff) % 19usize.pow(4);

                if i >= 3 {
                    let div = last_4 >> 6;
                    let rem = last_4 & 63;
                    let flag = 1 << rem;

                    if sequence[div] & flag == 0 {
                        slice[last_4].fetch_add(new_price, Ordering::Relaxed);
                        sequence[div] |= flag;
                    }
                }

                num = new;
                old_price = new_price;
            }
        });

    slice
        .iter()
        .map(|s| s.load(Ordering::Relaxed))
        .max()
        .unwrap()
}

fn next_secret_number(mut n: u64) -> u64 {
    n = prune(mix(n, n << 6));
    n = prune(mix(n >> 5, n));
    n = prune(mix(n << 11, n));
    n
}

fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

fn prune(n: u64) -> u64 {
    n & 16_777_215
}

fn parse(input: &str) -> Box<[u64]> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn main() {
    println!("Advent of Code 2024 - Day 22:");

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

    const INPUT: &str = include_str!("../../input/22_test.txt");
    const RESULT_1: u64 = 37_990_510;
    const RESULT_2: u64 = 23;

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
