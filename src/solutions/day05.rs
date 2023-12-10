use crate::parsing::ReadExt;
use crate::solver::Solver;
use anyhow::anyhow;
use rayon::prelude::*;
use scan_fmt::scan_fmt;
use std::io::Read;
use std::ops::{Range, RangeInclusive};
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Almanac;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        Self::Input::from_reader(r).expect("valid input")
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input
            .seeds
            .iter()
            .cloned()
            .map(|seed| input.maps.iter().fold(seed, |id, map| map.get(id)))
            .min()
            .unwrap_or(0)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input
            .seed_pairs()
            .into_par_iter()
            .flat_map(|range| {
                range
                    .into_par_iter()
                    .map(|seed| input.maps.iter().fold(seed, |id, map| map.get(id)))
            })
            .min()
            .unwrap_or(0)
    }
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

impl Almanac {
    fn from_reader<R: Read>(r: R) -> anyhow::Result<Self> {
        let groups: Vec<String> = r.split_groups();
        let mut group_iter = groups.iter();

        let seeds = scan_fmt!(
            group_iter.next().ok_or(anyhow!("missing seeds"))?,
            "seeds: {/[0-9 ]+/}",
            String
        )?
        .split_ascii_whitespace()
        .flat_map(usize::from_str)
        .collect();

        let maps = group_iter.flat_map(|s| s.parse()).collect();

        Ok(Self { seeds, maps })
    }

    fn seed_pairs(&self) -> Vec<Range<usize>> {
        self.seeds.chunks(2).map(|c| c[0]..(c[0] + c[1])).collect()
    }
}

#[derive(Debug)]
struct Map {
    entries: Vec<MapEntry>,
}

impl Map {
    fn get(&self, id: usize) -> usize {
        self.entries
            .iter()
            .filter_map(|e| e.get(id))
            .next()
            .unwrap_or(id)
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            entries: s.lines().skip(1).flat_map(|l| l.parse()).collect(),
        })
    }
}

#[derive(Debug)]
struct MapEntry {
    range: RangeInclusive<usize>,
    delta: usize,
}

impl MapEntry {
    fn new(n: usize, source: usize, dest: usize) -> Self {
        Self {
            range: source..=source + n,
            delta: dest - source,
        }
    }

    fn get(&self, id: usize) -> Option<usize> {
        self.range.contains(&id).then_some(self.delta + id)
    }
}

impl FromStr for MapEntry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dest, source, n) = scan_fmt!(s, "{} {} {}", usize, usize, usize)?;
        Ok(Self::new(n, source, dest))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_entry() {
        let map_entry = MapEntry::new(5, 1, 21);
        assert_eq!(map_entry.get(1), Some(21));
        assert_eq!(map_entry.get(6), Some(26));
        assert_eq!(map_entry.get(7), None);
    }

    #[test]
    fn map_entry2() {
        let map_entry = MapEntry::new(2, 50, 98);
        assert_eq!(map_entry.get(79), None);
        assert_eq!(map_entry.get(14), None);
    }
}
