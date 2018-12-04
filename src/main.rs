#[macro_use]
extern crate bindata;
#[macro_use]
extern crate bindata_impl;
extern crate chrono;
extern crate counter;
extern crate levenshtein;
extern crate nalgebra;
extern crate reqwest;
extern crate spade;

mod assets {
    bindata!("session.cookie");
}

// mod day1;
// mod day2;
// mod day3;
mod day4;
mod request;

fn main() {
    // day1::run();
    // day2::run();
    day4::run();
}
