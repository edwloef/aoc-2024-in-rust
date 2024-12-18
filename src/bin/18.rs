use arrayvec::ArrayVec;
use std::{collections::VecDeque, time::Instant};

const INPUT: &str = include_str!("../../input/18.txt");

fn solve_1(input: &[Box<[u8]>], size: usize) -> u32 {
    bfs(input, (0, 0), (size - 1, size - 1)).unwrap()
}

fn solve_2(input: &str, size: usize) -> &str {
    let mut lower = 0;
    let mut upper = input.lines().count();

    let mut too_high = false;
    while lower != upper {
        let new = (lower + upper) / 2;

        too_high = bfs(&parse(input, size, new), (0, 0), (size - 1, size - 1)).is_none();

        if too_high {
            upper = new;
        } else {
            lower = if lower % upper > 0 { new + 1 } else { new };
        }
    }

    input
        .lines()
        .nth(
            if too_high || bfs(&parse(input, size, lower), (0, 0), (size - 1, size - 1)).is_none() {
                lower - 1
            } else {
                lower
            },
        )
        .unwrap()
}

fn bfs(map: &[Box<[u8]>], start: (usize, usize), end: (usize, usize)) -> Option<u32> {
    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut visited =
        vec![vec![None; map[0].len()].into_boxed_slice(); map.len()].into_boxed_slice();
    visited[start.1][start.0] = Some(0);

    while let Some(pos) = queue.pop_front() {
        if pos == end {
            return visited[pos.1][pos.0];
        }

        let new_cost = visited[pos.1][pos.0].unwrap() + 1;

        for new_pos in neighbors(map, pos) {
            if visited[new_pos.1][new_pos.0].is_none_or(|cost| cost > new_cost) {
                visited[new_pos.1][new_pos.0] = Some(new_cost);
                queue.push_back(new_pos);
            }
        }
    }

    None
}

fn neighbors(map: &[Box<[u8]>], (x, y): (usize, usize)) -> ArrayVec<(usize, usize), 4> {
    let mut neighbors = ArrayVec::new();

    if y != 0 && map[y - 1][x] != b'#' {
        neighbors.push((x, y - 1));
    }

    if x != map[y].len() - 1 && map[y][x + 1] != b'#' {
        neighbors.push((x + 1, y));
    }

    if y != map.len() - 1 && map[y + 1][x] != b'#' {
        neighbors.push((x, y + 1));
    }

    if x != 0 && map[y][x - 1] != b'#' {
        neighbors.push((x - 1, y));
    }

    neighbors
}

fn parse(input: &str, size: usize, bytes: usize) -> Box<[Box<[u8]>]> {
    let mut output = vec![vec![b'.'; size].into_boxed_slice(); size].into_boxed_slice();

    input
        .lines()
        .take(bytes)
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .for_each(|(x, y)| {
            output[y][x] = b'#';
        });

    output
}

fn main() {
    println!("Advent of Code 2024 - Day 18:");

    let now = Instant::now();
    let input = parse(INPUT, 71, 1024);
    println!("- Parsed in {:#?}", now.elapsed());

    let now = Instant::now();
    let output = solve_1(&input, 71);
    println!("- Problem 1 solved in {:#?}: {output}", now.elapsed());

    let now = Instant::now();
    let output = solve_2(INPUT, 71);
    println!("- Problem 2 solved in {:#?}: {output}", now.elapsed());
}

#[cfg(test)]
mod test {
    use crate::{parse, solve_1, solve_2};

    const INPUT: &str = include_str!("../../input/18_test.txt");
    const RESULT_1: u32 = 22;
    const RESULT_2: &str = "6,1";

    #[test]
    fn test_1() {
        let input = parse(INPUT, 7, 12);
        let output = solve_1(&input, 7);
        assert_eq!(output, RESULT_1);
    }

    #[test]
    fn test_2() {
        let output = solve_2(INPUT, 7);
        assert_eq!(output, RESULT_2);
    }
}
