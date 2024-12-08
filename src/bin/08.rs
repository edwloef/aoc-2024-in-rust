use ahash::{HashMap, HashMapExt as _, HashSet, HashSetExt as _};
use derive_more::derive::{Add, AddAssign, Sub};
use std::time::Instant;

const INPUT: &str = include_str!("../../input/08.txt");

#[derive(Add, AddAssign, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd, Sub)]
struct Point {
    x: i8,
    y: i8,
}

fn solve_1((antennas, bounds): &(Box<[Box<[Point]>]>, Point)) -> usize {
    let mut antinodes = HashSet::new();

    antennas.iter().for_each(|t| {
        t.iter().for_each(|&a| {
            t.iter().filter(|&&b| a != b).for_each(|&b| {
                let diff = a - b;

                antinodes.insert(a + diff);
                antinodes.insert(b - diff);
            });
        });
    });

    antinodes
        .iter()
        .filter(|&a| a < bounds && a.x >= 0 && a.y >= 0)
        .count()
}

fn solve_2((antennas, bounds): &(Box<[Box<[Point]>]>, Point)) -> usize {
    let mut antinodes = HashSet::new();

    antennas.iter().for_each(|t| {
        t.iter().for_each(|&a| {
            t.iter().filter(|&&b| a != b).for_each(|&b| {
                let diff = a - b;

                let mut a = a;
                while a.x >= 0 && a.x < bounds.x && a.y >= 0 && a.y < bounds.y {
                    antinodes.insert(a);
                    a += diff;
                }

                let mut b = b;
                while b.x >= 0 && b.x < bounds.x && b.y >= 0 && b.y < bounds.y {
                    antinodes.insert(b);
                    b += diff;
                }
            });
        });
    });

    antinodes.len()
}

fn parse(input: &str) -> (Box<[Box<[Point]>]>, Point) {
    let mut antennas = HashMap::new();

    input.lines().zip(0..).for_each(|(line, y)| {
        line.bytes()
            .zip(0..)
            .filter(|&(c, _)| c != b'.')
            .for_each(|(c, x)| {
                antennas
                    .entry(c)
                    .and_modify(|e: &mut Vec<Point>| {
                        e.push(Point { x, y });
                    })
                    .or_insert_with(|| vec![Point { x, y }]);
            });
    });

    (
        antennas
            .drain()
            .map(|(_, v)| v.into_boxed_slice())
            .collect(),
        Point {
            x: input.lines().next().unwrap().len().try_into().unwrap(),
            y: input.lines().count().try_into().unwrap(),
        },
    )
}

fn main() {
    println!("Advent of Code 2024 - Day 08:");

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

    const INPUT: &str = include_str!("../../input/08_test.txt");
    const RESULT_1: usize = 14;
    const RESULT_2: usize = 34;

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
