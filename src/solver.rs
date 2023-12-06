use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;

fn input_file(day: u32) -> String {
    format!("input/{:02}.txt", day)
}

pub trait Solver {
    type Input;
    type Output1: Display;
    type Output2: Display;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input;
    fn solve_first(&self, input: &Self::Input) -> Self::Output1;
    fn solve_second(&self, input: &Self::Input) -> Self::Output2;

    fn load_input<P: AsRef<Path>>(&self, p: P) -> io::Result<Self::Input> {
        let f = File::open(p)?;
        Ok(self.parse_input(f))
    }

    fn solve(&self, day: u32) {
        let input_file = input_file(day);
        let input = self
            .load_input(input_file)
            .expect("unable to open input file");

        let start = Instant::now();
        let s1 = self.solve_first(&input);
        let time = start.elapsed();
        println!("Solution 1: {:<20} ({}ms)", s1, time.as_secs_f64() * 1000.0);

        let start = Instant::now();
        let s2 = self.solve_second(&input);
        let time = start.elapsed();
        println!("Solution 2: {:<20} ({}ms)", s2, time.as_secs_f64() * 1000.0);
    }
}

pub trait ReadExt<T> {
    fn split_by(self, separator: u8) -> Vec<T>;
    fn split_commas(self) -> Vec<T>;
    fn split_lines(self) -> Vec<T>;
    fn split_groups(self) -> Vec<T>;
}

impl<R, T> ReadExt<T> for R
where
    R: Read,
    T: FromStr,
{
    fn split_by(self, separator: u8) -> Vec<T> {
        BufReader::new(self)
            .split(separator)
            .flatten()
            .flat_map(String::from_utf8)
            .flat_map(|s| s.parse())
            .collect()
    }

    fn split_commas(self) -> Vec<T> {
        self.split_by(b',')
    }

    fn split_lines(self) -> Vec<T> {
        BufReader::new(self)
            .lines()
            .flatten()
            .flat_map(|l| l.parse())
            .collect()
    }

    fn split_groups(self) -> Vec<T> {
        BufReader::new(self)
            .lines()
            .flatten()
            .collect::<Vec<_>>()
            .split(|l| l.is_empty())
            .flat_map(|e| e.join("\n").parse())
            .collect::<Vec<T>>()
    }
}
