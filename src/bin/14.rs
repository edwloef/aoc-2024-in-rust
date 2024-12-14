use std::time::Instant;

const INPUT: &str = include_str!("../../input/14.txt");

#[derive(Clone, Copy)]
struct Robot {
    pub x_pos: i16,
    pub y_pos: i16,
    pub x_speed: i16,
    pub y_speed: i16,
}

fn solve_1(input: &mut [Robot], (x, y): (i16, i16)) -> u32 {
    for _ in 0..100 {
        for robot in input.iter_mut() {
            robot.x_pos += robot.x_speed;
            robot.x_pos = robot.x_pos.rem_euclid(x);
            robot.y_pos += robot.y_speed;
            robot.y_pos = robot.y_pos.rem_euclid(y);
        }
    }

    let mut quadrant = [0; 4];

    let x_m = x / 2;
    let y_m = y / 2;

    input
        .iter()
        .filter(|robot| robot.x_pos != x_m && robot.y_pos != y_m)
        .for_each({
            |robot| {
                if robot.x_pos < x_m {
                    if robot.y_pos < y_m {
                        quadrant[0] += 1;
                    } else {
                        quadrant[1] += 1;
                    }
                } else if robot.y_pos < y_m {
                    quadrant[2] += 1;
                } else {
                    quadrant[3] += 1;
                }
            }
        });

    quadrant[0] * quadrant[1] * quadrant[2] * quadrant[3]
}

fn solve_2(input: &mut [Robot], (x, y): (i16, i16)) -> u32 {
    let mut c = 1;

    while isnt_tree(input, (x, y)) {
        for robot in input.iter_mut() {
            robot.x_pos += robot.x_speed;
            robot.x_pos = robot.x_pos.rem_euclid(x);
            robot.y_pos += robot.y_speed;
            robot.y_pos = robot.y_pos.rem_euclid(y);
        }

        c += 1;
    }

    c
}

fn isnt_tree(input: &[Robot], (x, y): (i16, i16)) -> bool {
    for i in 0..input.len() {
        let some = input[i];
        for other in &input[i + 1..] {
            if some.x_pos == other.x_pos && some.y_pos == other.y_pos {
                return true;
            }
        }
    }

    for y in 0..y {
        let mut counter = 0;
        for x in 0..x {
            if input
                .iter()
                .any(|robot| robot.x_pos == x && robot.y_pos == y)
            {
                if counter == 30 {
                    return false;
                }

                counter += 1;
            } else {
                counter = 0;
            }
        }
    }

    true
}

fn parse(input: &str) -> Box<[Robot]> {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(pos, speed)| {
            let (pos, speed) = (&pos[2..], &speed[2..]);

            let (x_pos, y_pos) = pos.split_once(',').unwrap();
            let (x_speed, y_speed) = speed.split_once(',').unwrap();

            Robot {
                x_pos: x_pos.parse().unwrap(),
                y_pos: y_pos.parse().unwrap(),
                x_speed: x_speed.parse().unwrap(),
                y_speed: y_speed.parse().unwrap(),
            }
        })
        .collect()
}

fn main() {
    println!("Advent of Code 2024 - Day 14:");

    let now = Instant::now();
    let mut input = parse(INPUT);
    println!("- Parsed in {:#?}", now.elapsed());

    let now = Instant::now();
    let output = solve_1(&mut input.clone(), (101, 103));
    println!("- Problem 1 solved in {:#?}: {output}", now.elapsed());

    let now = Instant::now();
    let output = solve_2(&mut input, (101, 103));
    println!("- Problem 2 solved in {:#?}: {output}", now.elapsed());
}

#[cfg(test)]
mod test {
    use crate::{parse, solve_1};

    const INPUT: &str = include_str!("../../input/14_test.txt");
    const RESULT_1: u32 = 12;

    #[test]
    fn test_1() {
        let mut input = parse(INPUT);
        let output = solve_1(&mut input, (11, 7));
        assert_eq!(output, RESULT_1);
    }
}
