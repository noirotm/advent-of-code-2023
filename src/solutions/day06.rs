use crate::solver::Solver;
use anyhow::anyhow;
use itertools::Itertools;
use rayon::prelude::*;
use scan_fmt::scan_fmt;
use std::io::{read_to_string, Read};
use std::iter::zip;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Races;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        Races::from_str(&read_to_string(r).unwrap()).unwrap()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input
            .0
            .iter()
            .map(|r| r.find_times_above_record().len())
            .product()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input.as_single_race().find_times_above_record_par().len()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn find_times_above_record(&self) -> Vec<u64> {
        (0..=self.time)
            .map(|t| (t, self.compute_distance(t)))
            .filter(|&(_, d)| d > self.distance)
            .map(|(t, _)| t)
            .collect()
    }

    fn find_times_above_record_par(&self) -> Vec<u64> {
        (0..=self.time)
            .into_par_iter()
            .map(|t| (t, self.compute_distance(t)))
            .filter(|&(_, d)| d > self.distance)
            .map(|(t, _)| t)
            .collect()
    }

    fn compute_distance(&self, loading_time: u64) -> u64 {
        let speed = loading_time;
        speed * (self.time - loading_time)
    }
}

#[derive(Debug)]
pub struct Races(Vec<Race>);

impl Races {
    fn as_single_race(&self) -> Race {
        let time = self
            .0
            .iter()
            .map(|r| r.time.to_string())
            .join("")
            .parse()
            .unwrap();
        let distance = self
            .0
            .iter()
            .map(|r| r.distance.to_string())
            .join("")
            .parse()
            .unwrap();
        Race { time, distance }
    }
}

impl FromStr for Races {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.lines();
        let times = scan_fmt!(
            it.next().ok_or(anyhow!("missing line"))?,
            "Time:      {/[0-9 ]+/}",
            String
        )?
        .split_ascii_whitespace()
        .flat_map(u64::from_str)
        .collect::<Vec<_>>();

        let distances = scan_fmt!(
            it.next().ok_or(anyhow!("missing line"))?,
            "Distance:  {/[0-9 ]+/}",
            String
        )?
        .split_ascii_whitespace()
        .flat_map(u64::from_str)
        .collect::<Vec<_>>();

        let races = zip(times, distances)
            .map(|(time, distance)| Race { time, distance })
            .collect();

        Ok(Self(races))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_distance() {
        let r = Race {
            time: 7,
            distance: 9,
        };
        assert_eq!(r.compute_distance(1), 6);
    }

    #[test]
    fn find_times() {
        let r = Race {
            time: 7,
            distance: 9,
        };
        assert_eq!(r.find_times_above_record(), vec![2, 3, 4, 5]);
    }

    #[test]
    fn as_single_race() {
        let races = Races(vec![
            Race {
                time: 7,
                distance: 9,
            },
            Race {
                time: 15,
                distance: 40,
            },
            Race {
                time: 30,
                distance: 200,
            },
        ]);
        assert_eq!(
            races.as_single_race(),
            Race {
                time: 71530,
                distance: 940200
            }
        );
    }
}
