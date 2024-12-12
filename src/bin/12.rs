use std::time::Instant;

const INPUT: &str = include_str!("../../input/12.txt");

fn solve_1(mut input: Box<[Box<[u8]>]>) -> u32 {
    (0..input.len())
        .map(|y| {
            (0..input[y].len())
                .map(|x| {
                    if input[y][x] < b'A' {
                        return 0;
                    }

                    let plant = input[y][x];
                    let (area, perimeter) = area_perimeter(&mut input, (x, y), plant);
                    area * perimeter
                })
                .sum::<u32>()
        })
        .sum()
}

fn area_perimeter(input: &mut [Box<[u8]>], (x, y): (usize, usize), plant: u8) -> (u32, u32) {
    if input[y][x] < b'A' {
        return (0, 0);
    }

    input[y][x] -= b'A';

    let mut area = 1;
    let mut perimeter = 0;

    let edge_bits = edge_bits(input, (x, y), plant);

    if edge_bits & 0x01 == 0 {
        if x > 0 {
            let (a, p) = area_perimeter(input, (x - 1, y), plant);
            area += a;
            perimeter += p;
        }
    } else {
        perimeter += 1;
    }

    if edge_bits & 0x02 == 0 {
        if x < input[y].len() - 1 {
            let (a, p) = area_perimeter(input, (x + 1, y), plant);
            area += a;
            perimeter += p;
        }
    } else {
        perimeter += 1;
    }

    if edge_bits & 0x04 == 0 {
        if y > 0 {
            let (a, p) = area_perimeter(input, (x, y - 1), plant);
            area += a;
            perimeter += p;
        }
    } else {
        perimeter += 1;
    }

    if edge_bits & 0x08 == 0 {
        if y < input.len() - 1 {
            let (a, p) = area_perimeter(input, (x, y + 1), plant);
            area += a;
            perimeter += p;
        }
    } else {
        perimeter += 1;
    }

    (area, perimeter)
}

fn solve_2(mut input: Box<[Box<[u8]>]>) -> u32 {
    (0..input.len())
        .map(|y| {
            (0..input[y].len())
                .map(|x| {
                    if input[y][x] < b'A' {
                        return 0;
                    }

                    let plant = input[y][x];
                    let (area, edges) = area_edges(&mut input, (x, y), plant, 0);
                    let edges = edges & !0x01;

                    area * edges
                })
                .sum::<u32>()
        })
        .sum()
}

fn area_edges(
    input: &mut [Box<[u8]>],
    (x, y): (usize, usize),
    plant: u8,
    previous_edge_bits: u8,
) -> (u32, u32) {
    fn up_down(
        input: &mut [Box<[u8]>],
        (x, y): (usize, usize),
        plant: u8,
        edge_bits: u8,
        edge_bits_left_right: u8,
        area: &mut u32,
        edges: &mut u32,
    ) {
        if edge_bits & 0x01 == 0 && x > 0 {
            let (a, e) = area_edges(input, (x - 1, y), plant, edge_bits_left_right);
            *area += a;
            *edges += e;
        }

        if edge_bits & 0x02 == 0 && x < input[y].len() - 1 {
            let (a, p) = area_edges(input, (x + 1, y), plant, edge_bits_left_right);
            *area += a;
            *edges += p;
        }
    }

    fn left_right(
        input: &mut [Box<[u8]>],
        (x, y): (usize, usize),
        plant: u8,
        edge_bits: u8,
        edge_bits_up_down: u8,
        area: &mut u32,
        edges: &mut u32,
    ) {
        if edge_bits & 0x04 == 0 && y > 0 {
            let (a, p) = area_edges(input, (x, y - 1), plant, edge_bits_up_down);
            *area += a;
            *edges += p;
        }

        if edge_bits & 0x08 == 0 && y < input.len() - 1 {
            let (a, p) = area_edges(input, (x, y + 1), plant, edge_bits_up_down);
            *area += a;
            *edges += p;
        }
    }

    if input[y][x] < b'A' {
        return (0, 0);
    }

    input[y][x] -= b'A';

    let mut area = 1;

    let edge_bits = edge_bits(input, (x, y), plant);
    let mut edges = (edge_bits & !previous_edge_bits).count_ones();

    let edge_bits_left_right = edge_bits & 0x0c;
    if edge_bits_left_right != 0 {
        up_down(
            input,
            (x, y),
            plant,
            edge_bits,
            edge_bits_left_right,
            &mut area,
            &mut edges,
        );
    }

    let edge_bits_up_down = edge_bits & 0x03;
    if edge_bits_up_down != 0 {
        left_right(
            input,
            (x, y),
            plant,
            edge_bits,
            edge_bits_up_down,
            &mut area,
            &mut edges,
        );
    }

    if edge_bits_left_right == 0 {
        up_down(
            input,
            (x, y),
            plant,
            edge_bits,
            edge_bits_left_right,
            &mut area,
            &mut edges,
        );
    }

    if edge_bits_up_down == 0 {
        left_right(
            input,
            (x, y),
            plant,
            edge_bits,
            edge_bits_up_down,
            &mut area,
            &mut edges,
        );
    }

    (area, edges)
}

fn edge_bits(input: &[Box<[u8]>], (x, y): (usize, usize), plant: u8) -> u8 {
    let mut edge_bits = 0;

    if x > 0 {
        if input[y][x - 1] != plant && input[y][x - 1] != plant - b'A' {
            edge_bits |= 0x01;
        }
    } else {
        edge_bits |= 0x01;
    }

    if x < input[y].len() - 1 {
        if input[y][x + 1] != plant && input[y][x + 1] != plant - b'A' {
            edge_bits |= 0x02;
        }
    } else {
        edge_bits |= 0x02;
    }

    if y > 0 {
        if input[y - 1][x] != plant && input[y - 1][x] != plant - b'A' {
            edge_bits |= 0x04;
        }
    } else {
        edge_bits |= 0x04;
    }

    if y < input.len() - 1 {
        if input[y + 1][x] != plant && input[y + 1][x] != plant - b'A' {
            edge_bits |= 0x08;
        }
    } else {
        edge_bits |= 0x08;
    }

    edge_bits
}

fn parse(input: &str) -> Box<[Box<[u8]>]> {
    input.lines().map(|line| line.as_bytes().into()).collect()
}

fn main() {
    println!("Advent of Code 2024 - Day 12:");

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

    const INPUT: &str = include_str!("../../input/12_test.txt");
    const RESULT_1: u32 = 1930;
    const RESULT_2: u32 = 1206;

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
