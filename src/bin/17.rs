use itertools::Itertools as _;
use std::time::Instant;

const INPUT: &str = include_str!("../../input/17.txt");

struct Computer {
    a: u64,
    b: u64,
    c: u64,
    program: Box<[u8]>,
}

fn solve_1(computer: &Computer) -> String {
    run_program(computer)
        .iter()
        .map(ToString::to_string)
        .join(",")
}

fn solve_2(mut computer: Computer) -> u64 {
    let idx = computer.program.len() - 1;
    recursive(&mut computer, idx, 0).unwrap()
}

fn recursive(computer: &mut Computer, idx: usize, a: u64) -> Option<u64> {
    for i in a << 3..(a << 3) + 8 {
        computer.a = i;

        if run_program(computer)[..] == computer.program[idx..] {
            if idx == 0 {
                return Some(computer.a);
            }

            let result = recursive(computer, idx - 1, i);
            if result.is_some() {
                return result;
            }
        }
    }

    None
}

fn run_program(
    Computer {
        mut a,
        mut b,
        mut c,
        ref program,
    }: &Computer,
) -> Box<[u8]> {
    let mut ip = 0;
    let mut output = vec![];

    let combo = |x, a, b, c| match x {
        4 => a,
        5 => b,
        6 => c,
        x if x < 4 => x.into(),
        _ => unreachable!(),
    };

    while ip < program.len() - 1 {
        match program[ip] {
            0 => a >>= combo(program[ip + 1], a, b, c),
            1 => b ^= u64::from(program[ip + 1]),
            2 => b = combo(program[ip + 1], a, b, c) & 0b111,
            3 if a != 0 => ip = usize::from(program[ip + 1]).wrapping_sub(2),
            3 => {}
            4 => b ^= c,
            5 => output.push(
                (combo(program[ip + 1], a, b, c) & 0b111)
                    .try_into()
                    .unwrap(),
            ),
            6 => b = a >> combo(program[ip + 1], a, b, c),
            7 => c = a >> combo(program[ip + 1], a, b, c),
            _ => unreachable!(),
        }

        ip = ip.wrapping_add(2);
    }

    output.into_boxed_slice()
}

fn parse(input: &str) -> Computer {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let mut registers = registers.lines();

    Computer {
        a: registers.next().unwrap()[12..].parse().unwrap(),
        b: registers.next().unwrap()[12..].parse().unwrap(),
        c: registers.next().unwrap()[12..].parse().unwrap(),
        program: program[9..]
            .split(',')
            .map(|c| c.trim().parse().unwrap())
            .collect(),
    }
}

fn main() {
    println!("Advent of Code 2024 - Day 17:");

    let now = Instant::now();
    let input = parse(INPUT);
    println!("- Parsed in {:#?}", now.elapsed());

    let now = Instant::now();
    let output = solve_1(&input);
    println!("- Problem 1 solved in {:#?}: {output}", now.elapsed());

    let now = Instant::now();
    let output = solve_2(input);
    println!("- Problem 2 solved in {:#?}: {output}", now.elapsed());
}

#[cfg(test)]
mod test {
    use crate::{parse, solve_1, solve_2};

    const INPUT: &str = include_str!("../../input/17_test.txt");
    const RESULT_1: &str = "5,7,3,0";
    const RESULT_2: u64 = 117_440;

    #[test]
    fn test_1() {
        let input = parse(INPUT);
        let output = solve_1(&input);
        assert_eq!(output, RESULT_1);
    }

    #[test]
    fn test_2() {
        let input = parse(INPUT);
        let output = solve_2(input);
        assert_eq!(output, RESULT_2);
    }
}
