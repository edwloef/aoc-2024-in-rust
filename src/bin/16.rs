use arrayvec::ArrayVec;
use std::{cmp::Reverse, collections::BinaryHeap, time::Instant};

const INPUT: &str = include_str!("../../input/16.txt");

const HORIZONTAL: usize = 0;
const VERTICAL: usize = 1;

fn solve_1(map: &[Box<[u8]>]) -> u32 {
    let start = (1, map.len() - 2);
    let end = (map[0].len() - 2, 1);

    let visited = dijkstra(map, start, end, &[HORIZONTAL]);

    *visited[end.1][end.0].iter().min().unwrap()
}

fn solve_2(map: &[Box<[u8]>]) -> usize {
    let start = (1, map.len() - 2);
    let end = (map[0].len() - 2, 1);

    let visited_from_start = dijkstra(map, start, end, &[HORIZONTAL]);
    let visited_from_end = dijkstra(map, end, start, &[HORIZONTAL, VERTICAL]);

    let goal = visited_from_end[start.1][start.0][HORIZONTAL];

    visited_from_start
        .iter()
        .zip(visited_from_end)
        .map(|(start_line, end_line)| {
            start_line
                .iter()
                .zip(end_line)
                .filter(|(a, b)| {
                    a[HORIZONTAL] + b[HORIZONTAL] == goal || a[VERTICAL] + b[VERTICAL] == goal
                })
                .count()
        })
        .sum()
}

fn dijkstra(
    map: &[Box<[u8]>],
    start: (usize, usize),
    end: (usize, usize),
    start_directions: &[usize],
) -> Box<[Box<[[u32; 2]]>]> {
    let mut queue = BinaryHeap::new();

    let mut visited = vec![vec![[u32::MAX >> 1; 2]; map[0].len()].into_boxed_slice(); map.len()]
        .into_boxed_slice();

    for &direction in start_directions {
        queue.push(Reverse((0, start, direction)));
        visited[start.1][start.0][direction] = 0;
    }

    while let Some(Reverse((cost, pos, direction))) = queue.pop() {
        if pos == end {
            continue;
        }

        for (new_pos, new_direction) in neighbors(map, pos) {
            let new_cost = cost + if direction == new_direction { 1 } else { 1001 };

            if visited[new_pos.1][new_pos.0][new_direction] > new_cost {
                visited[new_pos.1][new_pos.0][new_direction] = new_cost;
                queue.push(Reverse((new_cost, new_pos, new_direction)));
            }

            let new_cost = new_cost + 1000;
            let new_direction = (new_direction + 1) % 2;

            if visited[new_pos.1][new_pos.0][new_direction] > new_cost {
                visited[new_pos.1][new_pos.0][new_direction] = new_cost;
            }
        }
    }

    visited
}

fn neighbors(map: &[Box<[u8]>], (x, y): (usize, usize)) -> ArrayVec<((usize, usize), usize), 4> {
    let mut neighbors = ArrayVec::new();

    if map[y - 1][x] != b'#' {
        neighbors.push(((x, y - 1), VERTICAL));
    }

    if map[y][x + 1] != b'#' {
        neighbors.push(((x + 1, y), HORIZONTAL));
    }

    if map[y + 1][x] != b'#' {
        neighbors.push(((x, y + 1), VERTICAL));
    }

    if map[y][x - 1] != b'#' {
        neighbors.push(((x - 1, y), HORIZONTAL));
    }

    neighbors
}

fn parse(input: &str) -> Box<[Box<[u8]>]> {
    input.lines().map(|line| line.as_bytes().into()).collect()
}

fn main() {
    println!("Advent of Code 2024 - Day 16:");

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

    const INPUT: &str = include_str!("../../input/16_test.txt");
    const RESULT_1: u32 = 7036;
    const RESULT_2: usize = 45;

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
