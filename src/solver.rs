use humantime::format_duration;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
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

        let start = Instant::now();
        let input = self
            .load_input(input_file)
            .expect("unable to open input file");
        let time = start.elapsed();
        println!("Parsing: ({})", format_duration(time));

        let start = Instant::now();
        let s1 = self.solve_first(&input);
        let time = start.elapsed();
        println!("Solution 1: {:<20} ({})", s1, format_duration(time));

        let start = Instant::now();
        let s2 = self.solve_second(&input);
        let time = start.elapsed();
        println!("Solution 2: {:<20} ({})", s2, format_duration(time));
    }
}
