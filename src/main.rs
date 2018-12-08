#[macro_use]
extern crate bindata;
#[macro_use]
extern crate bindata_impl;
extern crate chrono;
extern crate counter;
extern crate failure;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate levenshtein;
extern crate nalgebra;
extern crate reqwest;
extern crate spade;
extern crate voronoi;

use itertools::Itertools;

use std::env;
use std::str::Lines;
use std::collections::HashMap;


mod assets {
    bindata!("session.cookie");
}

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod request;

type Callback = fn(Lines) -> Result<(), failure::Error>;

lazy_static! {
    static ref MODMAP: HashMap<&'static str, Callback> = {
        let mut m = HashMap::new();
        m.insert("1", day1::run as Callback);
        m.insert("2", day2::run as Callback);
        m.insert("3", day3::run as Callback);
        m.insert("4", day4::run as Callback);
        m.insert("5", day5::run as Callback);
        m.insert("6", day6::run as Callback);
        m.insert("7", day7::run as Callback);
        m
    };
}

fn get_runner_and_input_url(day: Option<&String>) -> Result<(String, Callback), failure::Error> {
    fn advent_of_code_input_url(day: String) -> String {
        format!("https://adventofcode.com/2018/day/{}/input", day)
    }

    let map = &MODMAP;
    let sorted = map.keys().sorted();
    let latest = sorted.last().unwrap();
    let latest_fn = map.get(*latest).unwrap();

    Ok(match day {
        None => (advent_of_code_input_url(latest.to_string()), *latest_fn),
        Some(day) => {
            let day = &**day;
            
            (advent_of_code_input_url(day.to_string()), *map.get(day).unwrap_or(latest_fn))
        }
    })
}


fn main() -> Result<(), failure::Error> {
    let args: Vec<String> = env::args().collect();
    let (input, func) = get_runner_and_input_url(args.get(1))?;

    func(request::get(&input)?.text()?.lines())?;


    Ok(())
}
