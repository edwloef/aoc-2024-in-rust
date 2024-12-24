use ahash::HashMap;
use arrayvec::ArrayVec;
use itertools::Itertools as _;
use std::time::Instant;

const INPUT: &str = include_str!("../../input/24.txt");

#[derive(Clone)]
struct Gate<'a> {
    lhs: &'a str,
    rel: &'a str,
    rhs: &'a str,
    res: &'a str,
}

fn solve_1((mut values, mut gates): (HashMap<&str, bool>, Vec<Gate<'_>>)) -> u64 {
    while !gates.is_empty() {
        gates.retain(|gate| {
            if !values.contains_key(gate.lhs) || !values.contains_key(gate.rhs) {
                return true;
            }

            values.insert(
                gate.res,
                match gate.rel {
                    "AND" => values[gate.lhs] & values[gate.rhs],
                    "OR" => values[gate.lhs] | values[gate.rhs],
                    "XOR" => values[gate.lhs] ^ values[gate.rhs],
                    _ => unreachable!(),
                },
            );

            false
        });
    }

    let mut res = 0;

    values
        .keys()
        .filter(|key| key.starts_with('z'))
        .sorted_unstable()
        .rev()
        .for_each(|k| {
            res <<= 1;
            res += u64::from(values[k]);
        });

    res
}

fn solve_2((_, mut gates): (HashMap<&str, bool>, Vec<Gate<'_>>)) -> String {
    let mut swapped = ArrayVec::<&str, 8>::new();
    let mut last_carry = "";

    for i in 0.. {
        let x = &*format!("x{i:02}");
        let y = &*format!("y{i:02}");

        let mut half_adder_1_sum = find(x, y, "XOR", &mut gates);
        if half_adder_1_sum.is_empty() {
            break;
        };

        let mut half_adder_1_carry = find(x, y, "AND", &mut gates);
        let mut full_adder_carry = "";

        if !last_carry.is_empty() {
            let mut half_adder_2_carry = find(last_carry, half_adder_1_sum, "AND", &mut gates);

            if half_adder_2_carry.is_empty() {
                std::mem::swap(&mut half_adder_1_sum, &mut half_adder_1_carry);
                swapped.extend([half_adder_1_sum, half_adder_1_carry]);

                half_adder_2_carry = find(last_carry, half_adder_1_sum, "AND", &mut gates);
            }

            let mut half_adder_2_sum = find(last_carry, half_adder_1_sum, "XOR", &mut gates);

            if half_adder_1_sum.starts_with('z') {
                std::mem::swap(&mut half_adder_1_sum, &mut half_adder_2_sum);
                swapped.extend([half_adder_1_sum, half_adder_2_sum]);
            }

            if half_adder_1_carry.starts_with('z') {
                std::mem::swap(&mut half_adder_1_carry, &mut half_adder_2_sum);
                swapped.extend([half_adder_1_carry, half_adder_2_sum]);
            }

            if half_adder_2_carry.starts_with('z') {
                std::mem::swap(&mut half_adder_2_carry, &mut half_adder_2_sum);
                swapped.extend([half_adder_2_carry, half_adder_2_sum]);
            }

            full_adder_carry = find(half_adder_2_carry, half_adder_1_carry, "OR", &mut gates);

            if full_adder_carry.starts_with('z') && full_adder_carry != "z45" {
                std::mem::swap(&mut full_adder_carry, &mut half_adder_2_sum);
                swapped.extend([full_adder_carry, half_adder_2_sum]);
            }
        }

        last_carry = if last_carry.is_empty() {
            half_adder_1_carry
        } else {
            full_adder_carry
        };
    }

    swapped.sort_unstable();

    swapped.iter().join(",")
}

fn find<'a>(lhs: &str, rhs: &str, rel: &str, gates: &mut Vec<Gate<'a>>) -> &'a str {
    let Some(position) = gates.iter().position(|gate| {
        gate.rel == rel
            && ((gate.lhs == lhs && gate.rhs == rhs) || (gate.lhs == rhs && gate.rhs == lhs))
    }) else {
        return "";
    };
    gates.remove(position).res
}

fn parse(input: &str) -> (HashMap<&str, bool>, Vec<Gate<'_>>) {
    let (values, gates) = input.split_once("\n\n").unwrap();

    let values = values
        .lines()
        .map(|line| (&line[..3], line.as_bytes()[5] == b'1'))
        .collect();

    let gates = gates
        .lines()
        .map(|line| Gate {
            lhs: &line[..3],
            rel: &line[4..line.len() - 11],
            rhs: &line[line.len() - 10..line.len() - 7],
            res: &line[line.len() - 3..],
        })
        .collect();

    (values, gates)
}

fn main() {
    println!("Advent of Code 2024 - Day 24:");

    let now = Instant::now();
    let input = parse(INPUT);
    println!("- Parsed in {:#?}", now.elapsed());

    let now = Instant::now();
    let output = solve_1(input.clone());
    println!("- Problem 1 solved in {:#?}: {output}", now.elapsed());

    let now = Instant::now();
    let output = solve_2(input);
    println!("- Problem 2 solved in {:#?}: {output}", now.elapsed());
}

#[cfg(test)]
mod test {
    use crate::{parse, solve_1};

    const INPUT: &str = include_str!("../../input/24_test.txt");
    const RESULT_1: u64 = 2024;

    #[test]
    fn test_1() {
        let input = parse(INPUT);
        let output = solve_1(input);
        assert_eq!(output, RESULT_1);
    }
}
