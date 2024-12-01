use std::env;

use reqwest::{self, header::COOKIE};

pub(crate) fn get(day: u32) -> String {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://adventofcode.com/2024/day/{}/input", day);

    client
        .get(url)
        .header(COOKIE, env::var("AOC_ID").unwrap())
        .send()
        .unwrap()
        .text()
        .unwrap()
}
