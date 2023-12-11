use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{Display, Error, Formatter};
use std::io::{BufRead, BufReader, Read};
use std::iter::{repeat, FromIterator};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Grid<T> {
    cells: Vec<T>,
    pub w: usize,
    pub h: usize,
}

#[allow(unused)]
impl<T> Grid<T> {
    pub fn new(w: usize, h: usize) -> Self
    where
        T: Clone + Default,
    {
        Self::new_with(w, h, Default::default())
    }

    pub fn new_with(w: usize, h: usize, val: T) -> Self
    where
        T: Clone,
    {
        Self {
            cells: Vec::from_iter(repeat(val).take(w * h)),
            w,
            h,
        }
    }

    pub fn from_reader_callback<R, F, E>(r: R, f: F) -> Result<Self, E>
    where
        R: Read,
        F: FnMut(u8) -> Result<T, E> + Copy,
    {
        let cells = BufReader::new(r)
            .lines()
            .map_while(Result::ok)
            .map(|l| l.bytes().map(f).collect::<Result<Vec<_>, _>>())
            .collect::<Result<Vec<_>, _>>()?;
        let h = cells.len();
        let w = cells.first().map_or(0, |c| c.len());

        Ok(Self {
            cells: cells.into_iter().flatten().collect(),
            w,
            h,
        })
    }

    pub fn from_reader<R: Read>(r: R) -> Result<Self, T::Error>
    where
        T: TryFrom<u8>,
    {
        Self::from_reader_callback(r, T::try_from)
    }

    pub fn from_split_whitespace_reader<R>(r: R) -> Result<Self, T::Err>
    where
        T: FromStr,
        R: Read,
    {
        let cells = BufReader::new(r)
            .lines()
            .map_while(Result::ok)
            .map(|l| {
                l.split_whitespace()
                    .map(T::from_str)
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        let h = cells.len();
        let w = cells.first().map_or(0, |c| c.len());

        Ok(Self {
            cells: cells.into_iter().flatten().collect(),
            w,
            h,
        })
    }

    pub fn from_map(points: HashMap<Point, T>) -> Self
    where
        T: Clone + Default,
    {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        for pt in points.keys() {
            if pt.x < min_x {
                min_x = pt.x;
            }
            if pt.x > max_x {
                max_x = pt.x;
            }
            if pt.y < min_y {
                min_y = pt.y;
            }
            if pt.y > max_y {
                max_y = pt.y;
            }
        }
        let w = (max_x - min_x + 1) as usize;
        let h = (max_y - min_y + 1) as usize;
        let x_offset = -min_x;
        let y_offset = -min_y;

        let mut grid = Self::new(w, h);

        for (pt, cell) in points {
            let x = (pt.x + x_offset) as usize;
            let y = (pt.y + y_offset) as usize;
            if let Some(v) = grid.get_mut((x, y)) {
                *v = cell;
            }
        }

        grid
    }

    pub fn get(&self, c: impl Coord) -> Option<&T> {
        if self.contains_coord(&c) {
            self.cells.get(c.x() + c.y() * self.w)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, c: impl Coord) -> Option<&mut T> {
        if self.contains_coord(&c) {
            self.cells.get_mut(c.x() + c.y() * self.w)
        } else {
            None
        }
    }

    pub fn contains_coord(&self, c: &impl Coord) -> bool {
        c.x() < self.w && c.y() < self.h
    }

    pub fn neighbours4(&self, c: impl Coord) -> Vec<&T> {
        self.neighbours_coords4(c)
            .iter()
            .flat_map(|&c| self.get(c))
            .collect()
    }

    pub fn neighbours_coords4(&self, c: impl Coord) -> Vec<(usize, usize)> {
        [(-1, 0), (0, -1), (0, 1), (1, 0)]
            .iter()
            .flat_map(|&(dx, dy)| self.neighbour_coords(&c, dx, dy))
            .collect()
    }

    pub fn neighbours8(&self, c: impl Coord) -> Vec<&T> {
        self.neighbours_coords8(c)
            .iter()
            .flat_map(|&c| self.get(c))
            .collect()
    }

    pub fn neighbours_coords8(&self, c: impl Coord) -> Vec<(usize, usize)> {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .flat_map(|&(dx, dy)| self.neighbour_coords(&c, dx, dy))
        .collect()
    }

    fn neighbour_coords(&self, c: &impl Coord, dx: isize, dy: isize) -> Option<(usize, usize)> {
        if (c.x() == 0 && dx == -1)
            || (c.y() == 0 && dy == -1)
            || (c.x() == self.w - 1 && dx == 1)
            || (c.y() == self.h - 1 && dy == 1)
        {
            None
        } else {
            Some((
                ((c.x() as isize) + dx) as usize,
                ((c.y() as isize) + dy) as usize,
            ))
        }
    }

    pub fn iter_row(&self, row: usize) -> RowIter<'_, T> {
        RowIter {
            grid: self,
            row,
            pos: 0,
        }
    }

    pub fn iter_col(&self, col: usize) -> ColIter<'_, T> {
        ColIter {
            grid: self,
            col,
            pos: 0,
        }
    }

    pub fn iter_with_coords(&self) -> IterWithCoords<'_, T> {
        IterWithCoords {
            grid: self,
            x: 0,
            y: 0,
        }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.cells
    }

    pub fn as_slice_mut(&mut self) -> &mut [T] {
        &mut self.cells
    }
}

impl<T> FromStr for Grid<T>
where
    T: TryFrom<u8>,
{
    type Err = T::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Grid::from_reader(s.as_bytes())
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for row in self.cells.chunks(self.w) {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub trait Coord {
    fn x(&self) -> usize;
    fn y(&self) -> usize;
    fn coords(&self) -> (usize, usize) {
        (self.x(), self.y())
    }
}

#[derive(Debug, Default, Eq, Hash, PartialEq)]
pub struct GridPoint {
    pub x: usize,
    pub y: usize,
}

impl Coord for GridPoint {
    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
    }
}

impl Coord for &GridPoint {
    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
    }
}

impl Coord for (usize, usize) {
    fn x(&self) -> usize {
        self.0
    }

    fn y(&self) -> usize {
        self.1
    }

    fn coords(&self) -> (usize, usize) {
        *self
    }
}

impl Coord for &(usize, usize) {
    fn x(&self) -> usize {
        self.0
    }

    fn y(&self) -> usize {
        self.1
    }

    fn coords(&self) -> (usize, usize) {
        **self
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

pub struct ColIter<'a, T> {
    grid: &'a Grid<T>,
    col: usize,
    pos: usize,
}

impl<'a, T> Iterator for ColIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.grid.get((self.col, self.pos))?;
        self.pos += 1;
        Some(val)
    }
}

pub struct RowIter<'a, T> {
    grid: &'a Grid<T>,
    row: usize,
    pos: usize,
}

impl<'a, T> Iterator for RowIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.grid.get((self.pos, self.row))?;
        self.pos += 1;
        Some(val)
    }
}

pub struct IterWithCoords<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<'a, T> Iterator for IterWithCoords<'a, T> {
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let item = ((self.x, self.y), self.grid.get((self.x, self.y))?);

        if self.x == self.grid.w - 1 {
            self.x = 0;
            self.y += 1;
        } else {
            self.x += 1;
        }

        if self.y == self.grid.h {
            return None;
        }

        Some(item)
    }
}

impl<'a, T> ExactSizeIterator for IterWithCoords<'a, T> {
    fn len(&self) -> usize {
        self.grid.w * self.grid.h
    }
}
