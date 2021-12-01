extern crate itermore;

mod day1;

fn main() {
    let day1_iterator = day1::prepare();

    println!("DAY1 - P1: {}", day1::part1(day1_iterator.clone()));
    println!("DAY1 - P2: {}", day1::part2(day1_iterator));
}
