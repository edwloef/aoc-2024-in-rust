use std::{
    array,
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    time::Instant,
};

const INPUT: &str = include_str!("../../input/09.txt");

#[derive(Clone, Copy)]
struct FileBlock {
    pos: usize,
    len: usize,
    id: usize,
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct FreeBlock {
    pos: usize,
    len: usize,
}

fn solve_1((mut file_blocks, mut free_blocks): (Box<[FileBlock]>, Box<[FreeBlock]>)) -> usize {
    let mut idx = 0;

    file_blocks
        .iter_mut()
        .rev()
        .map(|file| {
            let mut ret = 0;
            loop {
                let free = &mut free_blocks[idx];
                if file.pos > free.pos {
                    match file.len.cmp(&free.len) {
                        Ordering::Less => {
                            ret += sum(free.pos, free.pos + file.len) * file.id;
                            free.pos += file.len;
                            free.len -= file.len;
                            break;
                        }
                        Ordering::Equal => {
                            ret += sum(free.pos, free.pos + free.len) * file.id;
                            idx += 1;
                            break;
                        }
                        Ordering::Greater => {
                            ret += sum(free.pos, free.pos + free.len) * file.id;
                            file.len -= free.len;
                            idx += 1;
                        }
                    }
                } else {
                    ret += sum(file.pos, file.pos + file.len) * file.id;
                    break;
                }
            }
            ret
        })
        .sum()
}

fn solve_2((file_blocks, free_blocks): (Box<[FileBlock]>, Box<[FreeBlock]>)) -> usize {
    let mut free_blocks_heaps: [_; 10] = array::from_fn(|_| BinaryHeap::new());
    free_blocks
        .iter()
        .for_each(|&free| free_blocks_heaps[free.len].push(Reverse(free)));

    file_blocks
        .iter()
        .rev()
        .map(|file| {
            let (min_pos, len) = (file.len..10)
                .filter(|&i| !free_blocks_heaps[i].is_empty())
                .map(|i| (free_blocks_heaps[i].peek().unwrap().0.pos, i))
                .min()
                .unwrap_or((usize::MAX, usize::MAX));

            if min_pos > file.pos {
                return sum(file.pos, file.pos + file.len) * file.id;
            }

            free_blocks_heaps[len].pop();
            if len > file.len {
                let new_len = len - file.len;
                free_blocks_heaps[new_len].push(Reverse(FreeBlock {
                    pos: min_pos + file.len,
                    len: new_len,
                }));
            }

            sum(min_pos, min_pos + file.len) * file.id
        })
        .sum()
}

fn sum(x: usize, y: usize) -> usize {
    ((y - x) * (x + y - 1)) / 2
}

fn parse(input: &str) -> (Box<[FileBlock]>, Box<[FreeBlock]>) {
    let mut file_blocks = Vec::with_capacity(input.len() / 2);
    let mut free_blocks = Vec::with_capacity(input.len() / 2);

    let mut pos = 0;
    input
        .trim()
        .bytes()
        .map(|c| (c - b'0').into())
        .enumerate()
        .filter(|&(_, len)| len > 0)
        .for_each(|(i, len)| {
            if i % 2 == 0 {
                file_blocks.push(FileBlock {
                    pos,
                    len,
                    id: i / 2,
                });
            } else {
                free_blocks.push(FreeBlock { pos, len });
            }
            pos += len;
        });

    (
        file_blocks.into_boxed_slice(),
        free_blocks.into_boxed_slice(),
    )
}

fn main() {
    println!("Advent of Code 2024 - Day 09:");

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

    const INPUT: &str = include_str!("../../input/09_test.txt");
    const RESULT_1: usize = 1928;
    const RESULT_2: usize = 2858;

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
