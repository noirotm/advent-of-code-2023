// DO NOT EDIT THIS FILE
use crate::solver::Solver;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;


pub fn exec_day(day: u32) {
    match day {
        1 => day01::Problem {}.solve(day),
        2 => day02::Problem {}.solve(day),
        3 => day03::Problem {}.solve(day),
        4 => day04::Problem {}.solve(day),
        5 => day05::Problem {}.solve(day),
        6 => day06::Problem {}.solve(day),
        7 => day07::Problem {}.solve(day),
        8 => day08::Problem {}.solve(day),
        9 => day09::Problem {}.solve(day),

        d => println!("Day {d} hasn't been solved yet :("),
    }
}

pub fn exec_all_days() {
    println!("Day 1:");
    day01::Problem {}.solve(1);
    println!("Day 2:");
    day02::Problem {}.solve(2);
    println!("Day 3:");
    day03::Problem {}.solve(3);
    println!("Day 4:");
    day04::Problem {}.solve(4);
    println!("Day 5:");
    day05::Problem {}.solve(5);
    println!("Day 6:");
    day06::Problem {}.solve(6);
    println!("Day 7:");
    day07::Problem {}.solve(7);
    println!("Day 8:");
    day08::Problem {}.solve(8);
    println!("Day 9:");
    day09::Problem {}.solve(9);
}
