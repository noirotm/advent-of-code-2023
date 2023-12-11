use crate::grid::Grid;
use crate::solver::Solver;
use anyhow::anyhow;
use itertools::Itertools;
use std::io::Read;

pub struct Problem;

impl Solver for Problem {
    type Input = Grid<Pixel>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        Grid::from_reader(r).unwrap()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        sum_of_distances(input, 1)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        sum_of_distances(input, 999999)
    }
}

fn find_empty_columns(g: &Grid<Pixel>) -> Vec<usize> {
    (0..g.w)
        .filter(|&col| g.iter_col(col).all(|v| *v == Pixel::Space))
        .collect()
}

fn find_empty_rows(g: &Grid<Pixel>) -> Vec<usize> {
    (0..g.h)
        .filter(|&row| g.iter_row(row).all(|v| *v == Pixel::Space))
        .collect()
}

fn find_galaxies(g: &Grid<Pixel>) -> Vec<(usize, usize)> {
    g.iter_with_coords()
        .filter_map(|(c, p)| (*p == Pixel::Galaxy).then_some(c))
        .collect()
}

fn expanding_manhattan_distance(
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
    empty_cols: &[usize],
    empty_rows: &[usize],
    expand_by: usize,
) -> usize {
    let distance = x2.abs_diff(x1) + y2.abs_diff(y1);

    let range_x = if x1 < x2 { x1..=x2 } else { x2..=x1 };
    let range_y = if y1 < y2 { y1..=y2 } else { y2..=y1 };

    let expand_x = empty_cols
        .iter()
        .filter(|&col| range_x.contains(col))
        .count();
    let expand_y = empty_rows
        .iter()
        .filter(|&row| range_y.contains(row))
        .count();

    distance + expand_x * expand_by + expand_y * expand_by
}

fn sum_of_distances(g: &Grid<Pixel>, expand_by: usize) -> u64 {
    let empty_cols = find_empty_columns(g);
    let empty_rows = find_empty_rows(g);
    let galaxies = find_galaxies(g);

    galaxies
        .iter()
        .combinations(2)
        .map(|v| {
            expanding_manhattan_distance(
                **v.get(0).unwrap(),
                **v.get(1).unwrap(),
                &empty_cols,
                &empty_rows,
                expand_by,
            ) as u64
        })
        .sum()
}

#[derive(Eq, PartialEq)]
pub enum Pixel {
    Space,
    Galaxy,
}

impl TryFrom<u8> for Pixel {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' => Ok(Self::Space),
            b'#' => Ok(Self::Galaxy),
            _ => Err(anyhow!("invalid pixel")),
        }
    }
}
