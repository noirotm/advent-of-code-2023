use crate::solutions::{exec_all_days, exec_day};
use std::env;

mod grid;
mod solutions;
mod solver;

fn main() {
    if let Some(day) = env::args().nth(1) {
        exec_day(day.parse().unwrap_or(1));
    } else {
        exec_all_days();
    }
}
