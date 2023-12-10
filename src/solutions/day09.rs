use crate::parsing::WhitespaceSeparatedList;
use crate::solver::Solver;
use itertools::Itertools;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Read};
use std::iter::successors;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Vec<i64>>;
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .flat_map(|l| WhitespaceSeparatedList::from_str(&l))
            .map(|l| l.into())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input.iter().flat_map(|v| extrapolate(v).pop()).sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut reversed = input.clone();
        for v in &mut reversed {
            v.reverse();
        }

        reversed.iter().flat_map(|v| extrapolate(v).pop()).sum()
    }
}

fn extrapolate(v: &[i64]) -> Vec<i64> {
    let mut derivative_stack = successors(Some(v.iter().cloned().collect_vec()), |v| {
        v.iter().any(|&e| e != 0).then_some(derivative(v))
    })
    .collect::<VecDeque<_>>();

    let mut expected_diff = 0;
    for v in derivative_stack.iter_mut().rev() {
        if let Some(&n) = v.last() {
            let extrapolated = n + expected_diff;
            v.push(extrapolated);
            expected_diff = extrapolated;
        }
    }

    derivative_stack.pop_front().expect("not empty")
}

fn derivative(nums: &[i64]) -> Vec<i64> {
    nums.windows(2).map(|n| n[1] - n[0]).collect()
}
