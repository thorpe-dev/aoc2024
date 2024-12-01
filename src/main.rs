mod args;
mod day1;
mod fetcher;

use aoc2024::Day;
use args::Args;
use clap::Parser;
use day1::Day1;

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
        1 => Box::new(Day1::new(data)),
        _ => panic!(),
    }
}

fn main() {
    let args = Args::parse();

    let data = fetcher::get(args.day);

    run(get_day(args.day, data), args.challenge);
}
