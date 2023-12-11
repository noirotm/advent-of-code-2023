use crate::grid::Grid;
use crate::solver::Solver;
use anyhow::anyhow;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::Read;

pub struct Problem;

impl Solver for Problem {
    type Input = Grid<Tile>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        Grid::from_reader(r).unwrap()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        //println!("{input}");

        let start = find_start(input).unwrap();
        let loop_starts = find_loop_starts(input, start);
        let mut previous_coord = start;
        let mut current_coord = *loop_starts.first().unwrap();
        let mut len = 1usize;
        let end = *loop_starts.last().unwrap();

        while current_coord != end {
            let new_current_coord = find_next_step(input, current_coord, previous_coord).unwrap();
            previous_coord = current_coord;
            current_coord = new_current_coord;
            len += 1;
        }

        (len + 1) / 2
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut path = HashMap::new();

        // find all coords representing the path
        let start = find_start(input).unwrap();
        let loop_starts = find_loop_starts(input, start);
        let mut previous_coord = start;
        let mut current_coord = *loop_starts.first().unwrap();
        let end = *loop_starts.last().unwrap();

        // insert start as pipe
        let p1 = Pos::new(start, current_coord).unwrap();
        let p2 = Pos::new(start, end).unwrap();
        let pipe = Pipe::from_pos(p1, p2).unwrap();
        path.insert(start, pipe);

        while current_coord != end {
            if let Some(p) = input.get(current_coord).unwrap().as_pipe() {
                path.insert(current_coord, *p);
            }

            let new_current_coord = find_next_step(input, current_coord, previous_coord).unwrap();
            previous_coord = current_coord;
            current_coord = new_current_coord;
        }

        // insert end as pipe
        path.insert(end, *input.get(end).unwrap().as_pipe().unwrap());

        let mut output = Grid::new_with(input.w, input.h, '.');

        // for each point not in the path, trace a ray right to count intersections with the path
        // even means outside
        // odd means inside
        let count = input
            .iter_with_coords()
            .filter_map(|(c, _)| (!path.contains_key(&c)).then_some(c))
            .map(|c| (c, trace_ray_count_intersections(c, input, &path)))
            .filter(|(_, n)| n % 2 != 0)
            .inspect(|(c, _)| {
                if let Some(e) = output.get_mut(c) {
                    *e = '1';
                }
            })
            .count();

        //println!("{output}");

        count
    }
}

fn trace_ray_count_intersections(
    (from_x, from_y): (usize, usize),
    grid: &Grid<Tile>,
    path: &HashMap<(usize, usize), Pipe>,
) -> usize {
    let mut intersections = 0usize;
    let mut has_n = false;
    let mut has_s = false;

    for x in from_x + 1..grid.w {
        let c = (x, from_y);

        // don't care about non-pipe tiles
        if let Some(p) = path.get(&c) {
            if *p == Pipe::NS {
                has_n = false;
                has_s = false;
                intersections += 1;
            } else {
                if p.has_pos(Pos::N) {
                    has_n = true;
                }
                if p.has_pos(Pos::S) {
                    has_s = true;
                }

                if !p.has_pos(Pos::E) {
                    if has_n && has_s {
                        intersections += 1;
                    }
                    has_n = false;
                    has_s = false;
                }
            }
        }
    }

    intersections
}

fn find_start(g: &Grid<Tile>) -> Option<(usize, usize)> {
    g.iter_with_coords()
        .find_map(|(c, t)| (t == &Tile::Start).then_some(c))
}

fn find_loop_starts(g: &Grid<Tile>, s: (usize, usize)) -> Vec<(usize, usize)> {
    g.neighbours_coords4(s)
        .iter()
        .flat_map(|c| g.get(c).map(|t| (*c, t)))
        .filter_map(|(c, t)| {
            let pos = Pos::new(s, c).unwrap().opposite();
            match t {
                Tile::Pipe(pipe) => pipe.has_pos(pos).then_some(c),
                _ => None,
            }
        })
        .collect()
}

fn find_next_step(
    g: &Grid<Tile>,
    current: (usize, usize),
    previous: (usize, usize),
) -> Option<(usize, usize)> {
    let current_pipe = g.get(current)?.as_pipe()?;
    g.neighbours_coords4(current)
        .iter()
        .filter(|&&c| c != previous)
        .flat_map(|&c| g.get(c).map(|t| (c, t)))
        .find(|(c, t)| {
            let pos = Pos::new(current, *c);
            match (pos, t) {
                (Some(pos), Tile::Pipe(pipe)) => current_pipe.is_compatible_with(&pipe, &pos),
                _ => false,
            }
        })
        .map(|(c, _)| c)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Pos {
    N,
    S,
    E,
    W,
}

impl Pos {
    fn new((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> Option<Self> {
        if x1 < x2 && y1 == y2 {
            return Some(Self::E);
        }
        if x1 > x2 && y1 == y2 {
            return Some(Self::W);
        }
        if x1 == x2 && y1 < y2 {
            return Some(Self::S);
        }
        if x1 == x2 && y1 > y2 {
            return Some(Self::N);
        }

        None
    }

    fn opposite(&self) -> Self {
        match self {
            Self::N => Self::S,
            Self::S => Self::N,
            Self::E => Self::W,
            Self::W => Self::E,
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum Tile {
    Ground,
    Start,
    Pipe(Pipe),
}

impl Tile {
    fn as_pipe(&self) -> Option<&Pipe> {
        if let Self::Pipe(p) = self {
            Some(p)
        } else {
            None
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Ground => ".".to_string(),
                Tile::Start => "S".to_string(),
                Tile::Pipe(p) => p.to_string(),
            }
        )
    }
}

impl TryFrom<u8> for Tile {
    type Error = anyhow::Error;

    fn try_from(b: u8) -> Result<Self, Self::Error> {
        Ok(match b {
            b'.' => Self::Ground,
            b'S' => Self::Start,
            v => Self::Pipe(Pipe::try_from(v)?),
        })
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Pipe {
    EW,
    NE,
    NS,
    NW,
    SE,
    SW,
}

impl Pipe {
    fn from_pos(p1: Pos, p2: Pos) -> Option<Self> {
        Some(match (p1, p2) {
            (Pos::E, Pos::W) => Self::EW,
            (Pos::N, Pos::E) => Self::NE,
            (Pos::N, Pos::S) => Self::NS,
            (Pos::N, Pos::W) => Self::NW,
            (Pos::S, Pos::E) => Self::SE,
            (Pos::S, Pos::W) => Self::SW,
            (Pos::W, Pos::E) => Self::EW,
            (Pos::E, Pos::N) => Self::NE,
            (Pos::S, Pos::N) => Self::NS,
            (Pos::W, Pos::N) => Self::NW,
            (Pos::E, Pos::S) => Self::SE,
            (Pos::W, Pos::S) => Self::SW,
            _ => None?,
        })
    }

    fn openings(&self) -> (Pos, Pos) {
        match self {
            Pipe::EW => (Pos::E, Pos::W),
            Pipe::NE => (Pos::N, Pos::E),
            Pipe::NS => (Pos::N, Pos::S),
            Pipe::NW => (Pos::N, Pos::W),
            Pipe::SE => (Pos::S, Pos::E),
            Pipe::SW => (Pos::S, Pos::W),
        }
    }

    fn has_pos(&self, p: Pos) -> bool {
        match self.openings() {
            (pos, _) if pos == p => true,
            (_, pos) if pos == p => true,
            _ => false,
        }
    }

    fn is_compatible_with(&self, other: &Pipe, pos: &Pos) -> bool {
        match pos {
            Pos::N => self.has_pos(Pos::N) && other.has_pos(Pos::S),
            Pos::S => self.has_pos(Pos::S) && other.has_pos(Pos::N),
            Pos::E => self.has_pos(Pos::E) && other.has_pos(Pos::W),
            Pos::W => self.has_pos(Pos::W) && other.has_pos(Pos::E),
        }
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pipe::EW => "═",
                Pipe::NE => "╚",
                Pipe::NS => "║",
                Pipe::NW => "╝",
                Pipe::SE => "╔",
                Pipe::SW => "╗",
            }
        )
    }
}

impl TryFrom<u8> for Pipe {
    type Error = anyhow::Error;

    fn try_from(b: u8) -> Result<Self, Self::Error> {
        Ok(match b {
            b'-' => Self::EW,
            b'L' => Self::NE,
            b'|' => Self::NS,
            b'J' => Self::NW,
            b'F' => Self::SE,
            b'7' => Self::SW,
            _ => Err(anyhow!("unknown character"))?,
        })
    }
}
