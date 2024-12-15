use std::time::Instant;

const INPUT: &str = include_str!("../../input/15.txt");

#[derive(Clone)]
struct Warehouse {
    map: Box<[Box<[u8]>]>,
    movements: Box<[Direction]>,
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn from_arrow(char: u8) -> Self {
        match char {
            b'^' => Self::Up,
            b'>' => Self::Right,
            b'v' => Self::Down,
            b'<' => Self::Left,
            _ => unreachable!(),
        }
    }

    fn step(self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Self::Up => (x, y - 1),
            Self::Right => (x + 1, y),
            Self::Down => (x, y + 1),
            Self::Left => (x - 1, y),
        }
    }
}

fn solve_1(
    &Warehouse {
        ref map,
        ref movements,
        mut x,
        mut y,
    }: &Warehouse,
) -> usize {
    let mut map = map.clone();

    movements.iter().for_each(|d| {
        let (mut tx, mut ty) = d.step(x, y);

        while map[ty][tx] == b'O' {
            (tx, ty) = d.step(tx, ty);
        }

        if map[ty][tx] == b'#' {
            return;
        }

        map[ty][tx] = b'O';

        map[y][x] = b'.';
        (x, y) = d.step(x, y);
        map[y][x] = b'@';
    });

    map.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|&(_, &c)| c == b'O')
                .map(|(x, _)| x + 100 * y)
                .sum::<usize>()
        })
        .sum()
}

fn solve_2(
    &Warehouse {
        ref map,
        ref movements,
        mut x,
        mut y,
    }: &Warehouse,
) -> usize {
    let mut map: Box<[Box<[_]>]> = map
        .iter()
        .map(|line| {
            line.iter()
                .flat_map(|c| match c {
                    b'#' => [b'#', b'#'],
                    b'O' => [b'[', b']'],
                    b'.' => [b'.', b'.'],
                    b'@' => [b'@', b'.'],
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    x *= 2;

    movements.iter().for_each(|&d| {
        let (tx, ty) = d.step(x, y);

        if map[ty][tx] == b'#' {
            return;
        }

        if map[ty][tx] != b'.' {
            if !can_push_box(&map, d, (tx, ty)) {
                return;
            }

            push_box(&mut map, d, (tx, ty));
        }

        map[y][x] = b'.';
        (x, y) = d.step(x, y);
        map[y][x] = b'@';
    });

    map.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|&(_, &c)| c == b'[')
                .map(|(x, _)| x + 100 * y)
                .sum::<usize>()
        })
        .sum()
}

fn can_push_box(map: &[Box<[u8]>], d: Direction, (mut x, mut y): (usize, usize)) -> bool {
    match d {
        Direction::Left | Direction::Right => {
            let lf = map[y][x];

            while map[y][x] == lf {
                (x, y) = d.step(x, y);
                (x, y) = d.step(x, y);
            }

            map[y][x] != b'#'
        }
        Direction::Up | Direction::Down => {
            if map[y][x] == b']' {
                x -= 1;
            }

            let (x, y) = d.step(x, y);

            if map[y][x] == b'#' || map[y][x + 1] == b'#' {
                false
            } else if map[y][x] == b']' {
                if map[y][x + 1] == b'[' {
                    return can_push_box(map, d, (x, y)) && can_push_box(map, d, (x + 1, y));
                }
                can_push_box(map, d, (x, y))
            } else if map[y][x] == b'[' {
                can_push_box(map, d, (x, y))
            } else if map[y][x + 1] == b'[' {
                can_push_box(map, d, (x + 1, y))
            } else {
                true
            }
        }
    }
}

fn push_box(map: &mut Box<[Box<[u8]>]>, d: Direction, (mut x, mut y): (usize, usize)) {
    match d {
        Direction::Left | Direction::Right => {
            let mut last = b'.';
            while map[y][x] != b'.' {
                last = if map[y][x] == b'[' { b']' } else { b'[' };
                map[y][x] = last;
                (x, y) = d.step(x, y);
            }

            map[y][x] = if last == b'[' { b']' } else { b'[' };
        }
        Direction::Up | Direction::Down => {
            if map[y][x] == b']' {
                x -= 1;
            }

            let (nx, ny) = d.step(x, y);

            if map[ny][nx] != b'.' || map[ny][nx + 1] != b'.' {
                if map[ny][nx] == b']' || map[ny][nx] == b'[' {
                    push_box(map, d, (nx, ny));
                }

                if map[ny][nx + 1] == b'[' {
                    push_box(map, d, (nx + 1, ny));
                }
            }

            map[ny][nx] = b'[';
            map[ny][nx + 1] = b']';
            map[y][x] = b'.';
            map[y][x + 1] = b'.';
        }
    }
}

fn parse(input: &str) -> Warehouse {
    let (map, movements) = input.split_once("\n\n").unwrap();

    let map: Box<[Box<[_]>]> = map.lines().map(|line| line.as_bytes().into()).collect();
    let movements = movements
        .lines()
        .flat_map(|line| line.bytes().map(Direction::from_arrow))
        .collect();
    let (x, y) = map
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find(|&(_, &c)| c == b'@')
                .map(|(x, _)| (x, y))
        })
        .unwrap();

    Warehouse {
        map,
        movements,
        x,
        y,
    }
}

fn main() {
    println!("Advent of Code 2024 - Day 15:");

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

    const INPUT: &str = include_str!("../../input/15_test.txt");
    const RESULT_1: usize = 10092;
    const RESULT_2: usize = 9021;

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
