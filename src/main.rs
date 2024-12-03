#![feature(array_windows)]
mod args;
mod day1;
mod day2;
mod day3;
mod fetcher;

use aoc2024::Day;
use args::Args;
use clap::Parser;
use std::time::Instant;

fn run(runner: Box<dyn Day>, challenge: u8) {
    let res = match challenge {
        1 => runner.first(),
        2 => runner.second(),
        _ => panic!(),
    };
    println!("{}", res);
}

fn get_day(day: u32, data: String) -> Box<dyn Day> {
    match day {
        1 => Box::new(day1::Day1::new(data)),
        2 => Box::new(day2::Day2::new(data)),
        3 => Box::new(day3::Day3::new(data)),
        _ => panic!(),
    }
}

fn main() {
    let args = Args::parse();

    let data = fetcher::get(args.day);

    let now = Instant::now();

    run(get_day(args.day, data), args.challenge);

    println!("Time taken = {}", now.elapsed().as_secs_f64());
}
