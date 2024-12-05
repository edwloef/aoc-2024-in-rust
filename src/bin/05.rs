use ahash::{HashMap, HashMapExt as _};
use std::{cmp::Ordering, time::Instant};

const INPUT: &str = include_str!("../../input/05.txt");

struct Manual {
    rules: HashMap<u32, Vec<u32>>,
    updates: Box<[Box<[u32]>]>,
}

fn solve_1(input: &Manual) -> u32 {
    input
        .updates
        .iter()
        .filter(|update| {
            update.iter().enumerate().all(|(i, page)| {
                input
                    .rules
                    .get(page)
                    .is_none_or(|after| update[..i].iter().all(|page| !after.contains(page)))
            })
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

fn solve_2(input: &Manual) -> u32 {
    input
        .updates
        .iter()
        .filter_map(|update| {
            let mut sorted_update = update.clone();
            sorted_update.sort_by(|a, b| {
                if input.rules.get(a).is_some_and(|after| after.contains(b)) {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            });

            if **update == *sorted_update {
                None
            } else {
                Some(sorted_update)
            }
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

fn parse(input: &str) -> Manual {
    let split = input.split_once("\n\n").unwrap();

    let mut rules = HashMap::new();
    split
        .0
        .lines()
        .map(|line| (&line[..2], &line[3..]))
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .for_each(|(a, b)| {
            rules
                .entry(a)
                .and_modify(|a: &mut Vec<u32>| a.push(b))
                .or_insert_with(|| vec![b]);
        });

    let updates = split
        .1
        .lines()
        .map(|line| line.split(',').map(|page| page.parse().unwrap()).collect())
        .collect();

    Manual { rules, updates }
}

fn main() {
    println!("Advent of Code 2024 - Day 05:");

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

    const INPUT: &str = include_str!("../../input/05_test.txt");
    const RESULT_1: u32 = 143;
    const RESULT_2: u32 = 123;

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
