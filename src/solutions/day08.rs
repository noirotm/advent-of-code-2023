use crate::solver::Solver;
use anyhow::anyhow;
use itertools::Itertools;
use rayon::prelude::*;
use scan_fmt::scan_fmt;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Map;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        let mut r = BufReader::new(r).lines();
        let directions = r
            .next()
            .unwrap()
            .unwrap()
            .split("")
            .flat_map(Dir::from_str)
            .collect();

        let graph = r
            .skip(1)
            .flatten()
            .flat_map(|s| scan_fmt!(&s, "{} = ({}, {})", String, String, String))
            .map(|(a, b, c)| (node_to_u16(&a), (node_to_u16(&b), node_to_u16(&c))))
            .collect();

        Map { directions, graph }
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut current_node = AAA;
        for (step, dir) in input.directions.iter().cycle().enumerate() {
            current_node = input
                .graph
                .get(&current_node)
                .map(|&(l, r)| match dir {
                    Dir::Left => l,
                    Dir::Right => r,
                })
                .unwrap();
            if current_node == ZZZ {
                return step + 1;
            }
        }

        unreachable!()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut current_nodes = input
            .graph
            .keys()
            .cloned()
            .filter(|&k| is_starter_node(k))
            .collect_vec();

        let mut first_endings = vec![HashMap::new(); current_nodes.len()];

        for (step, dir) in input.directions.iter().cycle().enumerate() {
            current_nodes = current_nodes
                .into_par_iter()
                .flat_map(|k| input.graph.get(&k))
                .map(|&(l, r)| match dir {
                    Dir::Left => l,
                    Dir::Right => r,
                })
                .collect();

            // try to identify cycles ( (node, Dir, step % N) )
            for (i, &n) in current_nodes.iter().enumerate() {
                if is_ending_node(n) {
                    let h = first_endings.get_mut(i).unwrap();
                    if let Entry::Vacant(e) = h.entry(n) {
                        e.insert(step + 1);
                    }
                }
            }

            if first_endings.iter().all(|h| !h.is_empty()) {
                if let Some(lcm) = first_endings
                    .iter()
                    .flat_map(|h| h.values())
                    .cloned()
                    .reduce(lcm)
                {
                    return lcm;
                }
            }

            // easy way out
            if current_nodes.iter().all(|&e| is_ending_node(e)) {
                return step + 1;
            }
        }

        unreachable!()
    }
}

const AAA: u16 = 0;
const ZZZ: u16 = 26 * 26 * 26 - 1;

#[derive(Debug)]
pub struct Map {
    directions: Vec<Dir>,
    graph: HashMap<u16, (u16, u16)>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Dir {
    Left,
    Right,
}

impl FromStr for Dir {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(anyhow!("unknown direction")),
        }
    }
}

fn node_to_u16(n: &str) -> u16 {
    n.bytes().fold(0, |acc, v| acc * 26 + (v - b'A') as u16)
}

fn is_starter_node(v: u16) -> bool {
    v % 26 == 0
}

fn is_ending_node(v: u16) -> bool {
    (v + 1) % 26 == 0
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_to_u16() {
        assert_eq!(node_to_u16("AAA"), 0);
        assert_eq!(node_to_u16("ZZZ"), 26 * 26 * 26 - 1);
    }

    #[test]
    fn test_is_starter_node() {
        assert!(is_starter_node(node_to_u16("AAA")));
        assert!(is_starter_node(node_to_u16("ZZA")));
        assert!(is_starter_node(node_to_u16("ABA")));
        assert!(!is_starter_node(node_to_u16("ABB")));
        assert!(!is_starter_node(node_to_u16("ABC")));
        assert!(!is_starter_node(node_to_u16("ABZ")));
    }

    #[test]
    fn test_is_ending_node() {
        assert!(!is_ending_node(node_to_u16("AAA")));
        assert!(!is_ending_node(node_to_u16("ZZA")));
        assert!(!is_ending_node(node_to_u16("ABA")));
        assert!(!is_ending_node(node_to_u16("ABB")));
        assert!(!is_ending_node(node_to_u16("ABC")));
        assert!(is_ending_node(node_to_u16("ABZ")));
        assert!(is_ending_node(node_to_u16("ZBZ")));
        assert!(is_ending_node(node_to_u16("ACZ")));
        assert!(is_ending_node(node_to_u16("ZZZ")));
    }
}
