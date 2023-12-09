use crate::parsing::ReadExt;
use crate::solver::Solver;
use scan_fmt::scan_fmt;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::Read;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Card>;
    type Output1 = u64;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_lines()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input.iter().map(|c| c.points()).sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let card_points = input
            .iter()
            .cloned()
            .map(|c| (c.id, c.matching_numbers()))
            .collect::<HashMap<_, _>>();

        let mut result = input.iter().map(|c| c.id).collect::<Vec<_>>();
        let mut cards_to_process = VecDeque::from_iter(result.iter().cloned());

        while let Some(id) = cards_to_process.pop_front() {
            if let Some(&score) = card_points.get(&id) {
                if score > 0 {
                    for c in id + 1..=id + score {
                        result.push(c);
                        cards_to_process.push_back(c);
                    }
                }
            }
        }

        result.len()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Card {
    id: usize,
    winning_numbers: HashSet<u8>,
    numbers: HashSet<u8>,
}

impl Card {
    fn matching_numbers(&self) -> usize {
        self.winning_numbers.intersection(&self.numbers).count()
    }

    fn points(&self) -> u64 {
        let wins = self.matching_numbers();
        if wins > 0 {
            2u64.pow(wins as u32 - 1)
        } else {
            0
        }
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // split parts
        let (id, winning_nums, nums) = scan_fmt!(
            s,
            "Card {d}: {/[0-9 ]+/} | {/[0-9 ]+/}",
            usize,
            String,
            String
        )?;

        Ok(Self {
            id,
            winning_numbers: winning_nums
                .split_ascii_whitespace()
                .flat_map(|s| s.parse())
                .collect(),
            numbers: nums
                .split_ascii_whitespace()
                .flat_map(|s| s.parse())
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_card() {
        let c = Card::from_str("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").unwrap();
        assert_eq!(
            c,
            Card {
                id: 1,
                winning_numbers: [41, 48, 83, 86, 17].into_iter().collect(),
                numbers: [83, 86, 6, 31, 17, 9, 48, 53].into_iter().collect(),
            }
        )
    }

    #[test]
    fn points() {
        let c = Card::from_str("Card 1: 1 2 3 | 1 2 3").unwrap();
        assert_eq!(c.points(), 4);

        let c = Card::from_str("Card 1: 1 2 3 | 4 5 6").unwrap();
        dbg!(&c);
        assert_eq!(c.points(), 0);

        let c = Card::from_str("Card 1: 1 2 3 | 1 2 5").unwrap();
        assert_eq!(c.points(), 2);

        let c = Card::from_str("Card 1: 1 2 3 | 5 3 9").unwrap();
        assert_eq!(c.points(), 1);
    }
}
