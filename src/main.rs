#[macro_use]
extern crate bindata;
#[macro_use]
extern crate bindata_impl;
extern crate chrono;
extern crate counter;
extern crate itertools;
extern crate levenshtein;
extern crate nalgebra;
extern crate reqwest;
extern crate spade;
extern crate voronoi;

mod assets {
    bindata!("session.cookie");
}

// mod day1;
// mod day2;
// mod day3;
// mod day4;
// mod day5;
// mod day6;
mod day7;
mod request;

fn main() {
    // day1::run();
    // day2::run();
    // day3::run();
    // day4::run();
    // day5::run();
    // day6::run();
    day7::run();
}
