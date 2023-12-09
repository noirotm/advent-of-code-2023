use crate::parsing::ReadExt;
use crate::solver::Solver;
use std::collections::BTreeSet;
use std::io::{read_to_string, Read};

pub struct Problem;

impl Solver for Problem {
    type Input = (BTreeSet<part1::HandBid>, BTreeSet<part2::HandBid>);
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        let s = read_to_string(r).unwrap();
        (s.as_bytes().split_lines(), s.as_bytes().split_lines())
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input
            .0
            .iter()
            .enumerate()
            .map(|(rank, h)| (rank as u64 + 1) * h.bid)
            .sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        //input.1.iter().for_each(|c| println!("{:?}", c));
        input
            .1
            .iter()
            .enumerate()
            .map(|(rank, h)| (rank as u64 + 1) * h.bid)
            .sum()
    }
}

mod part1 {
    use anyhow::anyhow;
    use itertools::Itertools;
    use scan_fmt::scan_fmt;
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::str::FromStr;

    #[derive(Debug)]
    pub struct Hand {
        cards: [Card; 5],
        combo: Combination,
    }

    impl Eq for Hand {}

    impl PartialEq<Self> for Hand {
        fn eq(&self, other: &Self) -> bool {
            self.cards.eq(&other.cards)
        }
    }

    impl PartialOrd<Self> for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            if self.combo != other.combo {
                return self.combo.cmp(&other.combo);
            }
            self.cards.cmp(&other.cards)
        }
    }

    impl Hand {
        fn new(cards: [Card; 5]) -> Self {
            let combo = Self::find_combo(&cards);
            Self { cards, combo }
        }

        fn find_combo(cards: &[Card; 5]) -> Combination {
            // count card occurences
            let mut occurences = HashMap::with_capacity(5);
            for c in cards {
                occurences
                    .entry(c)
                    .and_modify(|e| *e += 1)
                    .or_insert(1usize);
            }

            // sort occurences, desc
            // iterate
            let mut has_three = false;
            let mut pairs = 0;

            for &occ in occurences.values().sorted().rev() {
                if occ == 5 {
                    return Combination::FiveOfAKind;
                }
                if occ == 4 {
                    return Combination::FourOfAKind;
                }
                if occ == 3 {
                    has_three = true;
                }
                if occ == 2 && has_three {
                    return Combination::FullHouse;
                }
                if occ == 2 {
                    pairs += 1;
                }
            }

            if has_three {
                return Combination::ThreeOfAKind;
            }
            if pairs == 2 {
                return Combination::TwoPair;
            }
            if pairs == 1 {
                return Combination::OnePair;
            }

            Combination::HighCard
        }
    }

    #[derive(Debug)]
    pub struct HandBid {
        pub hand: Hand,
        pub bid: u64,
    }

    impl Eq for HandBid {}

    impl PartialEq<Self> for HandBid {
        fn eq(&self, other: &Self) -> bool {
            self.hand.eq(&other.hand)
        }
    }

    impl PartialOrd<Self> for HandBid {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for HandBid {
        fn cmp(&self, other: &Self) -> Ordering {
            self.hand.cmp(&other.hand)
        }
    }

    impl FromStr for HandBid {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (hand, bid) = scan_fmt!(s, "{} {d}", String, u64)?;
            Ok(Self {
                hand: Hand::new(
                    hand.split("")
                        .flat_map(Card::from_str)
                        .collect_vec()
                        .try_into()
                        .map_err(|_| anyhow!("invalid cards"))?,
                ),
                bid,
            })
        }
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    enum Card {
        C2,
        C3,
        C4,
        C5,
        C6,
        C7,
        C8,
        C9,
        CT,
        CJ,
        CQ,
        CK,
        CA,
    }

    impl FromStr for Card {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "2" => Ok(Self::C2),
                "3" => Ok(Self::C3),
                "4" => Ok(Self::C4),
                "5" => Ok(Self::C5),
                "6" => Ok(Self::C6),
                "7" => Ok(Self::C7),
                "8" => Ok(Self::C8),
                "9" => Ok(Self::C9),
                "T" => Ok(Self::CT),
                "J" => Ok(Self::CJ),
                "Q" => Ok(Self::CQ),
                "K" => Ok(Self::CK),
                "A" => Ok(Self::CA),
                v => Err(anyhow!("invalid string {v}")),
            }
        }
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    enum Combination {
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::solutions::day07::part1::Card::*;
        use crate::solutions::day07::part1::Combination::*;

        #[test]
        fn from_str() {
            let hb = HandBid::from_str("32T3K 765").unwrap();
            assert_eq!(
                hb,
                HandBid {
                    hand: Hand::new([C3, C2, CT, C3, CK]),
                    bid: 765
                }
            );
        }

        #[test]
        fn combo() {
            let h = Hand::new([C2, C3, C4, C5, C6]);
            assert_eq!(h.combo, HighCard);

            let h = Hand::new([C2, C2, C4, C5, C6]);
            assert_eq!(h.combo, OnePair);

            let h = Hand::new([C2, C2, C4, C4, C6]);
            assert_eq!(h.combo, TwoPair);

            let h = Hand::new([C2, C2, C2, C5, C6]);
            assert_eq!(h.combo, ThreeOfAKind);

            let h = Hand::new([C2, C2, C2, C5, C5]);
            assert_eq!(h.combo, FullHouse);

            let h = Hand::new([C2, C2, C2, C2, C6]);
            assert_eq!(h.combo, FourOfAKind);

            let h = Hand::new([C2, C2, C2, C2, C2]);
            assert_eq!(h.combo, FiveOfAKind);
        }

        #[test]
        fn ord() {
            let h1 = Hand::new([C2, C3, C4, C5, C6]);
            let h2 = Hand::new([C2, C3, C4, C5, C7]);
            assert!(h1 < h2);

            let h1 = Hand::new([C2, C3, C4, C5, C6]);
            let h2 = Hand::new([C6, C3, C4, C5, C2]);
            assert!(h1 < h2);

            let h1 = Hand::new([C2, C3, C4, C5, C6]);
            let h2 = Hand::new([C2, C2, C4, C5, C6]);
            assert!(h1 < h2);

            let h1 = Hand::new([C2, C2, C4, C5, C6]);
            let h2 = Hand::new([C2, C2, C4, C4, C6]);
            assert!(h1 < h2);
        }
    }
}

mod part2 {
    use anyhow::anyhow;
    use itertools::Itertools;
    use scan_fmt::scan_fmt;
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::str::FromStr;

    #[derive(Debug)]
    pub struct Hand {
        cards: [Card; 5],
        combo: Combination,
    }

    impl Eq for Hand {}

    impl PartialEq<Self> for Hand {
        fn eq(&self, other: &Self) -> bool {
            self.cards.eq(&other.cards)
        }
    }

    impl PartialOrd<Self> for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            if self.combo != other.combo {
                return self.combo.cmp(&other.combo);
            }
            self.cards.cmp(&other.cards)
        }
    }

    impl Hand {
        fn new(cards: [Card; 5]) -> Self {
            let combo = Self::find_combo(&cards);
            Self { cards, combo }
        }

        fn find_combo(cards: &[Card; 5]) -> Combination {
            // count card occurences
            let mut occurences = HashMap::with_capacity(5);
            for c in cards {
                occurences
                    .entry(c)
                    .and_modify(|e| *e += 1)
                    .or_insert(1usize);
            }

            // sort occurences, desc
            // iterate
            let jokers = occurences.get(&Card::CJ).cloned().unwrap_or_default();
            let mut unused_jokers = jokers;

            let mut has_three = false;
            let mut pairs = 0usize;

            for occ in occurences
                .iter()
                .filter(|&(&c, _)| !c.eq(&Card::CJ))
                .map(|(_, n)| *n)
                .sorted()
                .rev()
            {
                // jokers can replace any card, so add the occurrences if not joker
                let occ = occ + unused_jokers;
                unused_jokers = 0;

                if occ == 5 {
                    return Combination::FiveOfAKind;
                }
                if occ == 4 {
                    return Combination::FourOfAKind;
                }
                if occ == 3 {
                    has_three = true;
                }
                if occ == 2 && has_three {
                    return Combination::FullHouse;
                }
                if occ == 2 {
                    pairs += 1;
                }
            }

            if has_three {
                return Combination::ThreeOfAKind;
            }
            if pairs == 2 {
                return Combination::TwoPair;
            }
            if pairs == 1 {
                return Combination::OnePair;
            }

            match jokers {
                5 => Combination::FiveOfAKind,
                4 => Combination::FourOfAKind,
                3 => Combination::ThreeOfAKind,
                2 => Combination::OnePair,
                _ => Combination::HighCard,
            }
        }
    }

    #[derive(Debug)]
    pub struct HandBid {
        pub hand: Hand,
        pub bid: u64,
    }

    impl Eq for HandBid {}

    impl PartialEq<Self> for HandBid {
        fn eq(&self, other: &Self) -> bool {
            self.hand.eq(&other.hand)
        }
    }

    impl PartialOrd<Self> for HandBid {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for HandBid {
        fn cmp(&self, other: &Self) -> Ordering {
            self.hand.cmp(&other.hand)
        }
    }

    impl FromStr for HandBid {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (hand, bid) = scan_fmt!(s, "{} {d}", String, u64)?;
            Ok(Self {
                hand: Hand::new(
                    hand.split("")
                        .flat_map(Card::from_str)
                        .collect_vec()
                        .try_into()
                        .map_err(|_| anyhow!("invalid cards"))?,
                ),
                bid,
            })
        }
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    enum Card {
        CJ,
        C2,
        C3,
        C4,
        C5,
        C6,
        C7,
        C8,
        C9,
        CT,
        CQ,
        CK,
        CA,
    }

    impl FromStr for Card {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "J" => Ok(Self::CJ),
                "2" => Ok(Self::C2),
                "3" => Ok(Self::C3),
                "4" => Ok(Self::C4),
                "5" => Ok(Self::C5),
                "6" => Ok(Self::C6),
                "7" => Ok(Self::C7),
                "8" => Ok(Self::C8),
                "9" => Ok(Self::C9),
                "T" => Ok(Self::CT),
                "Q" => Ok(Self::CQ),
                "K" => Ok(Self::CK),
                "A" => Ok(Self::CA),
                v => Err(anyhow!("invalid string {v}")),
            }
        }
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    enum Combination {
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::solutions::day07::part2::Card::*;
        use crate::solutions::day07::part2::Combination::*;

        #[test]
        fn from_str() {
            let hb = HandBid::from_str("32T3K 765").unwrap();
            assert_eq!(
                hb,
                HandBid {
                    hand: Hand::new([C3, C2, CT, C3, CK]),
                    bid: 765
                }
            );
        }

        #[test]
        fn combo() {
            let h = Hand::new([C2, C3, C4, C5, C6]);
            assert_eq!(h.combo, HighCard);

            let h = Hand::new([C2, C2, C4, C5, C6]);
            assert_eq!(h.combo, OnePair);

            let h = Hand::new([C2, C2, C4, C4, C6]);
            assert_eq!(h.combo, TwoPair);

            let h = Hand::new([C2, C2, C2, C5, C6]);
            assert_eq!(h.combo, ThreeOfAKind);

            let h = Hand::new([C2, C2, C2, C5, C5]);
            assert_eq!(h.combo, FullHouse);

            let h = Hand::new([C2, C2, C2, C2, C6]);
            assert_eq!(h.combo, FourOfAKind);

            let h = Hand::new([C2, C2, C2, C2, C2]);
            assert_eq!(h.combo, FiveOfAKind);
        }

        #[test]
        fn ord() {
            let h1 = Hand::new([C2, C3, C4, C5, C6]);
            let h2 = Hand::new([C2, C3, C4, C5, C7]);
            assert!(h1 < h2);

            let h1 = Hand::new([C2, C3, C4, C5, C6]);
            let h2 = Hand::new([C6, C3, C4, C5, C2]);
            assert!(h1 < h2);

            let h1 = Hand::new([C2, C3, C4, C5, C6]);
            let h2 = Hand::new([C2, C2, C4, C5, C6]);
            assert!(h1 < h2);

            let h1 = Hand::new([C2, C2, C4, C5, C6]);
            let h2 = Hand::new([C2, C2, C4, C4, C6]);
            assert!(h1 < h2);
        }

        #[test]
        fn joker() {
            let h = Hand::new([CJ, CJ, CJ, CJ, C2]);
            assert_eq!(h.combo, FiveOfAKind);

            let h = Hand::new([C2, CJ, CJ, CJ, CJ]);
            assert_eq!(h.combo, FiveOfAKind);
        }
    }
}
