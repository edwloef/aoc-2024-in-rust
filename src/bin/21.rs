use ahash::{HashMap, HashMapExt as _};
use arrayvec::ArrayVec;
use itertools::Itertools as _;
use memoize::memoize;
use std::{
    iter::{once, repeat_n},
    time::Instant,
};

const INPUT: &str = include_str!("../../input/21.txt");

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Keypad(i8, i8);

impl Keypad {
    const A: Self = Self(0, 0);

    const ZERO: Self = Self(1, 0);
    const ONE: Self = Self(2, 1);
    const TWO: Self = Self(1, 1);
    const THREE: Self = Self(0, 1);
    const FOUR: Self = Self(2, 2);
    const FIVE: Self = Self(1, 2);
    const SIX: Self = Self(0, 2);
    const SEVEN: Self = Self(2, 3);
    const EIGHT: Self = Self(1, 3);
    const NINE: Self = Self(0, 3);

    const ARROW_UP: Self = Self(1, 0);
    const ARROW_RIGHT: Self = Self(0, -1);
    const ARROW_DOWN: Self = Self(1, -1);
    const ARROW_LEFT: Self = Self(2, -1);

    fn from_byte(c: u8) -> Self {
        match c {
            b'A' => Self::A,
            b'0' => Self::ZERO,
            b'1' => Self::ONE,
            b'2' => Self::TWO,
            b'3' => Self::THREE,
            b'4' => Self::FOUR,
            b'5' => Self::FIVE,
            b'6' => Self::SIX,
            b'7' => Self::SEVEN,
            b'8' => Self::EIGHT,
            b'9' => Self::NINE,
            _ => unreachable!(),
        }
    }

    fn to_digit(self) -> u64 {
        match self {
            Self::ZERO => 0,
            Self::ONE => 1,
            Self::TWO => 2,
            Self::THREE => 3,
            Self::FOUR => 4,
            Self::FIVE => 5,
            Self::SIX => 6,
            Self::SEVEN => 7,
            Self::EIGHT => 8,
            Self::NINE => 9,
            _ => unreachable!(),
        }
    }
}

fn solve_1(input: &[Box<[Keypad]>]) -> u64 {
    solve(input, 3)
}

fn solve_2(input: &[Box<[Keypad]>]) -> u64 {
    solve(input, 26)
}

fn solve(input: &[Box<[Keypad]>], depth: u8) -> u64 {
    let mut complexity = 0;
    for input in input.iter().cloned() {
        let mut numeric = 0;
        input.iter().filter(|&&c| c != Keypad::A).for_each(|c| {
            numeric *= 10;
            numeric += c.to_digit();
        });

        let length = once(Keypad::A)
            .chain(input)
            .tuple_windows()
            .map(|(last, cur)| recursive(last, cur, depth))
            .sum::<u64>();

        complexity += numeric * length;
    }

    complexity
}

#[memoize(CustomHasher: HashMap)]
fn recursive(last: Keypad, cur: Keypad, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut expand = ArrayVec::<Keypad, 6>::new();
    expand.push(Keypad::A);

    if last.0 > cur.0 {
        expand.extend(repeat_n(Keypad::ARROW_RIGHT, last.0.abs_diff(cur.0).into()));
    }

    if last.1 < cur.1 {
        expand.extend(repeat_n(Keypad::ARROW_UP, last.1.abs_diff(cur.1).into()));
    }

    if last.1 > cur.1 {
        expand.extend(repeat_n(Keypad::ARROW_DOWN, last.1.abs_diff(cur.1).into()));
    }

    if last.0 < cur.0 {
        expand.extend(repeat_n(Keypad::ARROW_LEFT, last.0.abs_diff(cur.0).into()));
    }

    expand.push(Keypad::A);

    let mut count = expand
        .iter()
        .tuple_windows()
        .map(|(&last, &cur)| recursive(last, cur, depth - 1))
        .sum::<u64>();

    if !(last.1 == 0 && cur.0 == 2 || cur.1 == 0 && last.0 == 2) {
        count = count.min(
            expand
                .iter()
                .rev()
                .tuple_windows()
                .map(|(&last, &cur)| recursive(last, cur, depth - 1))
                .sum(),
        );
    }

    count
}

fn parse(input: &str) -> Box<[Box<[Keypad]>]> {
    input
        .lines()
        .map(|line| line.bytes().map(Keypad::from_byte).collect())
        .collect()
}

fn main() {
    println!("Advent of Code 2024 - Day 21:");

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

    const INPUT: &str = include_str!("../../input/21_test.txt");
    const RESULT_1: u64 = 126_384;
    const RESULT_2: u64 = 154_115_708_116_294;

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
