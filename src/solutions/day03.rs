use crate::grid::Grid;
use crate::solver::Solver;
use std::collections::{HashMap, HashSet};
use std::io::Read;

pub struct Problem;

impl Solver for Problem {
    type Input = Grid<u8>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        Grid::from_reader(r).expect("valid grid")
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        find_correct_numbers(input).iter().sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        find_gear_coords_for_numbers(input)
            .iter()
            .filter(|(_, v)| v.len() == 2)
            .map(|(_, v)| v[0] * v[1])
            .sum()
    }
}

fn find_correct_numbers(g: &Grid<u8>) -> Vec<u64> {
    let mut out = vec![];

    for y in 0..g.h {
        let mut accum = 0;
        let mut has_symbol = false;

        for x in 0..g.w {
            let b = g.get((x, y)).expect("valid point");

            if b.is_ascii_digit() {
                // build number
                accum = accum * 10 + (b - b'0') as u64;
                // search symbol
                has_symbol |= g
                    .neighbours8((x, y))
                    .iter()
                    .any(|&b| !b.eq(&b'.') && !b.is_ascii_digit());
            } else if accum > 0 {
                if has_symbol {
                    out.push(accum);
                    has_symbol = false;
                }
                accum = 0;
            }
        }

        // end of line cleanup
        if accum > 0 && has_symbol {
            out.push(accum);
        }
    }

    out
}

fn find_gear_coords_for_numbers(g: &Grid<u8>) -> HashMap<(usize, usize), Vec<u64>> {
    let mut output: HashMap<(usize, usize), Vec<u64>> = HashMap::new();

    for y in 0..g.h {
        let mut accum = 0;
        let mut current_number_gears = HashSet::new();

        for x in 0..g.w {
            let b = g.get((x, y)).expect("valid point");

            if b.is_ascii_digit() {
                accum = accum * 10 + (b - b'0') as u64;

                let gears = g
                    .neighbours_coords8((x, y))
                    .into_iter()
                    .map(|c| (c, g.get(c).expect("valid point")))
                    .filter(|(_, &b)| b.eq(&b'*'))
                    .map(|(c, _)| c);
                current_number_gears.extend(gears);
            } else if accum > 0 {
                for &gear in &current_number_gears {
                    output
                        .entry(gear)
                        .and_modify(|numbers| (*numbers).push(accum))
                        .or_insert(vec![accum]);
                }
                current_number_gears.clear();
                accum = 0;
            }
        }

        if accum > 0 {
            for &gear in &current_number_gears {
                output
                    .entry(gear)
                    .and_modify(|numbers| (*numbers).push(accum))
                    .or_insert(vec![accum]);
            }
            current_number_gears.clear();
        }
    }

    output
}
