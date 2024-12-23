use ahash::{HashMap, HashMapExt as _, HashSet, HashSetExt as _};
use itertools::Itertools as _;
use std::{iter::once, time::Instant};

const INPUT: &str = include_str!("../../input/23.txt");

fn solve_1(input: &HashMap<&str, HashSet<&str>>) -> u32 {
    let mut count = 0;

    for node1 in input.keys() {
        let t1 = node1.starts_with('t');
        for node2 in &input[node1] {
            let t2 = node2.starts_with('t');
            for node3 in &input[node2] {
                let t3 = node3.starts_with('t');
                if (t1 || t2 || t3) && input[node3].contains(node1) {
                    count += 1;
                }
            }
        }
    }

    count / 6
}

fn solve_2(input: &HashMap<&str, HashSet<&str>>) -> String {
    bron_kerbosch(
        input,
        &mut HashSet::new(),
        &mut input.keys().copied().collect(),
        &mut HashSet::new(),
    )
    .into_iter()
    .sorted_unstable()
    .join(",")
}

fn bron_kerbosch<'a>(
    graph: &HashMap<&str, HashSet<&'a str>>,
    r: &mut HashSet<&'a str>,
    p: &mut HashSet<&'a str>,
    x: &mut HashSet<&'a str>,
) -> HashSet<&'a str> {
    if p.is_empty() && x.is_empty() {
        return r.clone();
    }

    let u = p.union(x).next().unwrap();

    let mut p_intersection = HashSet::new();
    let mut x_intersection = HashSet::new();

    p.clone()
        .difference(&graph[u])
        .map(|&v| {
            r.insert(v);

            p_intersection.clear();
            x_intersection.clear();

            p_intersection.extend(p.intersection(&graph[v]).copied());
            x_intersection.extend(x.intersection(&graph[v]).copied());

            let out = bron_kerbosch(graph, r, &mut p_intersection, &mut x_intersection);

            r.remove(v);
            p.remove(v);
            x.insert(v);

            out
        })
        .max_by(|lhs, rhs| lhs.len().cmp(&rhs.len()))
        .unwrap_or_else(HashSet::new)
}

fn parse(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut out = HashMap::new();

    input
        .lines()
        .map(|line| (&line[..2], &line[3..]))
        .for_each(|(from, to)| {
            out.entry(from)
                .and_modify(|x: &mut HashSet<_>| {
                    x.insert(to);
                })
                .or_insert_with(|| once(to).collect());

            out.entry(to)
                .and_modify(|x: &mut HashSet<_>| {
                    x.insert(from);
                })
                .or_insert_with(|| once(from).collect());
        });

    out
}

fn main() {
    println!("Advent of Code 2024 - Day 23:");

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

    const INPUT: &str = include_str!("../../input/23_test.txt");
    const RESULT_1: u32 = 7;
    const RESULT_2: &str = "co,de,ka,ta";

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
