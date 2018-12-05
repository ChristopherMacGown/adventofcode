use request::{get, Error};
use std::iter::Peekable;

const INPUT: &'static str = "https://adventofcode.com/2018/day/5/input";

fn char_matcher(ch: char) -> char {
    if ch.is_uppercase() {
        ch.to_lowercase().next()
    } else {
        ch.to_uppercase().next()
    }.unwrap()
}

fn parse_polymer<I: Into<String>>(input: I) -> String {
    fn remove_adjacent<I: Iterator<Item = char>>(
        ch: char,
        input: &mut Peekable<I>,
    ) -> Option<char> {
        {
            let matcher = char_matcher(ch);
            if let Some(next) = input.peek() {
                if matcher != *next {
                    return Some(ch);
                }
            } else {
                return Some(ch);
            }
        }

        input.next();
        None
    }

    let input = input.into();
    let mut iter = input.chars().peekable();
    let mut output = Vec::new();
    while let Some(&ch) = iter.peek() {
        iter.next();
        if let Some(ch) = remove_adjacent(ch, &mut iter) {
            output.push(ch);
        }
    }

    output.into_iter().collect()
}

fn process<I: Into<String>>(input: I) -> String {
    let mut input = input.into();
    let mut previous = input.clone();

    loop {
        input = parse_polymer(input);
        if input == previous {
            break;
        }
        previous = input.clone();
    }

    input
}

fn clean_and_process<I: Into<String>>(input: I) -> usize {
    let input = input.into();
    let mut unique_units: Vec<char> = input.clone().to_lowercase().chars().collect();

    unique_units.sort();
    unique_units.dedup();

    unique_units
        .into_iter()
        .map(|c| (c, char_matcher(c)))
        .map(|(l, u)| {
            let i = input
                .clone()
                .chars()
                .filter(|&x| x != l && x != u)
                .collect::<String>();
            process(i).len()
        }).min()
        .unwrap()
}

pub fn run() -> Result<(), Error> {
    let input = get(INPUT)?.text()?;

    println!("{}", process("aA"));
    println!("{}", process("abBA"));
    println!("{}", process("abAB"));
    println!("{}", process("aabAAB"));
    println!("{}", process("dabAcCaCBAcCcaDA"));
    println!("I: {}", process(input.trim_end()).len());
    println!("II: {}", clean_and_process(input.trim_end()));

    Ok(())
}
