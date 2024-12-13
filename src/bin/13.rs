use std::time::Instant;

const INPUT: &str = include_str!("../../input/13.txt");

#[derive(Clone, Copy)]
struct Machine {
    ax: f64,
    ay: f64,
    bx: f64,
    by: f64,
    px: f64,
    py: f64,
}

impl Machine {
    pub fn solve(self) -> (f64, f64) {
        let b = self.py.mul_add(self.ax, -self.ay * self.px)
            / self.ax.mul_add(self.by, -self.ay * self.bx);
        let a = self.bx.mul_add(-b, self.px) / self.ax;

        (a, b)
    }
}

fn solve_1(input: impl Iterator<Item = Machine>) -> u64 {
    input
        .map(Machine::solve)
        .filter(|(a, b)| a.fract() == 0.0 && b.fract() == 0.0)
        .map(|(a, b)| 3.0f64.mul_add(a, b) as u64)
        .sum()
}

fn solve_2(input: &[Machine]) -> u64 {
    solve_1(input.iter().copied().map(|mut eq| {
        eq.px += 10_000_000_000_000.0;
        eq.py += 10_000_000_000_000.0;
        eq
    }))
}

fn parse(input: &str) -> Box<[Machine]> {
    input
        .split("\n\n")
        .map(|machine| machine.lines())
        .map(|mut machine| {
            (
                &machine.next().unwrap()[12..],
                &machine.next().unwrap()[12..],
                machine.next().unwrap()[9..].split_once(", Y=").unwrap(),
            )
        })
        .map(|(a, b, p)| Machine {
            ax: a[..2].parse().unwrap(),
            ay: a[6..].parse().unwrap(),
            bx: b[..2].parse().unwrap(),
            by: b[6..].parse().unwrap(),
            px: p.0.parse().unwrap(),
            py: p.1.parse().unwrap(),
        })
        .collect()
}

fn main() {
    println!("Advent of Code 2024 - Day 13:");

    let now = Instant::now();
    let input = parse(INPUT);
    println!("- Parsed in {:#?}", now.elapsed());

    let now = Instant::now();
    let output = solve_1(input.iter().copied());
    println!("- Problem 1 solved in {:#?}: {output}", now.elapsed());

    let now = Instant::now();
    let output = solve_2(&input);
    println!("- Problem 2 solved in {:#?}: {output}", now.elapsed());
}

#[cfg(test)]
mod test {
    use crate::{parse, solve_1, solve_2};

    const INPUT: &str = include_str!("../../input/13_test.txt");
    const RESULT_1: u64 = 480;
    const RESULT_2: u64 = 875_318_608_908;

    #[test]
    fn test_1() {
        let input = parse(INPUT);
        let output = solve_1(input.iter().copied());
        assert_eq!(output, RESULT_1);
    }

    #[test]
    fn test_2() {
        let input = parse(INPUT);
        let output = solve_2(&input);
        assert_eq!(output, RESULT_2);
    }
}
