use crate::solver::{ReadExt, Solver};
use std::io::Read;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<String>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_lines()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .filter_map(|s| first_last_number_in_string(s))
            .map(|(a, b)| digits_to_number(a, b))
            .sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input
            .iter()
            .map(|s| numbers_from_str(s))
            .map(|v| (v.first().cloned(), v.last().cloned()))
            .filter_map(|t| match t {
                (Some(a), Some(b)) => Some((a, b)),
                _ => None,
            })
            .map(|(a, b)| digits_to_number(a, b))
            .sum()
    }
}

fn first_last_number_in_string(s: &str) -> Option<(u8, u8)> {
    let mut n = (None, None);
    for b in s.bytes().filter(u8::is_ascii_digit) {
        let num = Some(b - b'0');
        if n.0.is_none() {
            n.0 = num;
        }
        n.1 = num;
    }
    match n {
        (Some(a), Some(b)) => Some((a, b)),
        _ => None,
    }
}

fn numbers_from_str(s: &str) -> Vec<u8> {
    let subs = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let mut out = vec![];
    let mut it = s;

    while !it.is_empty() {
        for (s, r) in subs {
            if it.starts_with(s) || it.starts_with(&r.to_string()) {
                out.push(r);
            }
        }
        it = &it[1..];
    }

    out
}

fn digits_to_number(first: u8, second: u8) -> u64 {
    first as u64 * 10 + second as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_last_number_in_string() {
        assert_eq!(first_last_number_in_string("123"), Some((1, 3)));
        assert_eq!(first_last_number_in_string("abc"), None);
        assert_eq!(first_last_number_in_string("1bc"), Some((1, 1)));
        assert_eq!(first_last_number_in_string(""), None);
    }

    #[test]
    fn test_number_strings_to_digits() {
        assert_eq!(numbers_from_str("onetwothree"), vec![1, 2, 3]);
        assert_eq!(numbers_from_str("eightwo"), vec![8, 2]);
        assert_eq!(numbers_from_str("zoneight234"), vec![1, 8, 2, 3, 4]);
        assert_eq!(
            numbers_from_str("four2tszbgmxpbvninebxns6nineqbqzgjpmpqr"),
            vec![4, 2, 9, 6, 9]
        );
        assert_eq!(numbers_from_str("onetwone"), vec![1, 2, 1])
    }

    #[test]
    fn test_digits_to_number() {
        assert_eq!(digits_to_number(1, 2), 12);
    }

    #[test]
    fn test_solve_first() {
        let i = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            .as_bytes();
        let p = Problem;
        let v = p.solve_first(&p.parse_input(i));
        assert_eq!(v, 142);
    }

    #[test]
    fn test_solve_second() {
        let i = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            .as_bytes();
        let p = Problem;
        let v = p.solve_second(&p.parse_input(i));
        assert_eq!(v, 281);
    }
}
