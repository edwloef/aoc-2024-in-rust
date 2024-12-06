use std::time::Instant;

const INPUT: &str = include_str!("../../input/06.txt");

#[derive(Clone)]
struct Guard {
    map: Box<[Box<[u8]>]>,
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
#[repr(u8)]
enum Direction {
    Up = 0x01,
    Right = 0x02,
    Down = 0x04,
    Left = 0x08,
}

impl Direction {
    fn rotate(self) -> Self {
        [Self::Right, Self::Down, Self::Left, Self::Up]
            [usize::try_from((self as u8).trailing_zeros()).unwrap()]
    }

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
            Self::Up => (x, y.wrapping_sub(1)),
            Self::Right => (x + 1, y),
            Self::Down => (x, y + 1),
            Self::Left => (x.wrapping_sub(1), y),
        }
    }
}

fn solve_1(
    Guard {
        mut map,
        mut x,
        mut y,
    }: Guard,
) -> u32 {
    let mut counter = 0;
    let mut direction = Direction::from_arrow(map[y][x]);
    let max_x = map[0].len();
    let max_y = map.len();

    while x < max_x && y < max_y {
        if map[y][x] == b'#' {
            direction = direction.rotate();
            (x, y) = direction.rotate().step(x, y);
        }

        if map[y][x] != b'X' {
            counter += 1;
            map[y][x] = b'X';
        }

        (x, y) = direction.step(x, y);
    }

    counter
}

fn solve_2(
    Guard {
        mut map,
        mut x,
        mut y,
    }: Guard,
) -> u32 {
    let mut counter = 0;
    let mut direction = Direction::from_arrow(map[y][x]);
    let max_x = map[0].len();
    let max_y = map.len();
    map[y][x] = 0x10;

    let mut clone_guard = Guard {
        map: map.clone(),
        x,
        y,
    };

    while x < max_x && y < max_y {
        if map[y][x] == b'#' {
            direction = direction.rotate();
            (x, y) = direction.rotate().step(x, y);
        }

        if map[y][x] >= 0x20 {
            clone_guard.map.clone_from(&map);
            clone_guard.map[y][x] = b'#';
            let new_direction = direction.rotate();
            (clone_guard.x, clone_guard.y) = new_direction.rotate().step(x, y);

            if with_new_obstruction(&mut clone_guard, new_direction) {
                counter += 1;
            }

            map[y][x] = direction as u8 | 0x10;
        } else {
            map[y][x] |= direction as u8;
        }

        (x, y) = direction.step(x, y);
    }

    counter
}

fn with_new_obstruction(Guard { map, x, y }: &mut Guard, mut direction: Direction) -> bool {
    let mut x = *x;
    let mut y = *y;

    let max_x = map[0].len();
    let max_y = map.len();

    while x < max_x && y < max_y {
        if map[y][x] == b'#' {
            direction = direction.rotate();
            (x, y) = direction.rotate().step(x, y);
        }

        if map[y][x] >= 0x20 {
            map[y][x] = direction as u8;
        } else if map[y][x] & direction as u8 != 0 {
            return true;
        } else {
            map[y][x] |= direction as u8;
        }

        (x, y) = direction.step(x, y);
    }

    false
}

fn parse(input: &str) -> Guard {
    let (mut px, mut py) = (0, 0);

    Guard {
        map: input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.bytes()
                    .enumerate()
                    .map(|(x, byte)| {
                        if byte == b'^' || byte == b'>' || byte == b'v' || byte == b'<' {
                            px = x;
                            py = y;
                        }
                        byte
                    })
                    .collect()
            })
            .collect(),
        x: px,
        y: py,
    }
}

fn main() {
    println!("Advent of Code 2024 - Day 06:");

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
    use crate::{parse, solve_1, solve_2};

    const INPUT: &str = include_str!("../../input/06_test.txt");
    const RESULT_1: u32 = 41;
    const RESULT_2: u32 = 6;

    #[test]
    fn test_1() {
        let input = parse(INPUT);
        let output = solve_1(input);
        assert_eq!(output, RESULT_1);
    }

    #[test]
    fn test_2() {
        let input = parse(INPUT);
        let output = solve_2(input);
        assert_eq!(output, RESULT_2);
    }
}
