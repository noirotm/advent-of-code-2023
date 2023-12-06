use crate::solver::{ReadExt, Solver};
use anyhow::anyhow;
use scan_fmt::scan_fmt;
use std::io::Read;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Game>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_lines()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        const CUBES: Cubes = Cubes {
            r: 12,
            g: 13,
            b: 14,
        };

        input
            .iter()
            .filter(|g| g.is_possible(&CUBES))
            .map(|g| g.id)
            .sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input.iter().map(|g| g.min_cubes().power()).sum()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Game {
    id: usize,
    turns: Vec<Cubes>,
}

impl Game {
    fn is_possible(&self, c: &Cubes) -> bool {
        self.turns
            .iter()
            .all(|t| t.r <= c.r && t.g <= c.g && t.b <= c.b)
    }

    fn min_cubes(&self) -> Cubes {
        self.turns
            .iter()
            .fold(Cubes::default(), |a, b| Self::max_cubes(&a, b))
    }

    fn max_cubes(a: &Cubes, b: &Cubes) -> Cubes {
        Cubes {
            r: a.r.max(b.r),
            g: a.g.max(b.g),
            b: a.b.max(b.b),
        }
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(':');
        let id = scan_fmt!(it.next().ok_or(anyhow!("missing game"))?, "Game {}", usize)?;
        let turns = it
            .next()
            .ok_or(anyhow!("missing turns"))?
            .split(';')
            .flat_map(Cubes::from_str)
            .collect();

        Ok(Game { id, turns })
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
struct Cubes {
    r: usize,
    g: usize,
    b: usize,
}

impl Cubes {
    fn power(&self) -> usize {
        self.r * self.g * self.b
    }
}

impl FromStr for Cubes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut turn = Cubes::default();
        for s in s.split(',') {
            let (n, color) = scan_fmt!(s, " {} {}", usize, String)?;
            match color.as_str() {
                "red" => turn.r = n,
                "green" => turn.g = n,
                "blue" => turn.b = n,
                _ => {}
            }
        }

        Ok(turn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cubes() {
        let c = " 1 red, 2 green, 6 blue";
        let c = Cubes::from_str(c).unwrap();
        assert_eq!(c, Cubes { r: 1, g: 2, b: 6 });
    }

    #[test]
    fn test_game() {
        let g = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let g = Game::from_str(g).unwrap();
        assert_eq!(
            g,
            Game {
                id: 1,
                turns: vec![
                    Cubes { r: 4, g: 0, b: 3 },
                    Cubes { r: 1, g: 2, b: 6 },
                    Cubes { r: 0, g: 2, b: 0 }
                ]
            }
        );
    }
}
