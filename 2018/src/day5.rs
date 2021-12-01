use request::Error;
use std::iter::Peekable;
use std::str::Lines;

fn char_matcher(ch: char) -> char {
    if ch.is_uppercase() {
        ch.to_ascii_lowercase()
    } else {
        ch.to_ascii_uppercase()
    }
}

fn parse_polymer<I: Iterator<Item = char>>(input: I) -> String {
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

    let mut iter = input.peekable();
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
    let mut prev_len = input.len();

    loop {
        input = parse_polymer(input.chars());
        let len = input.len();
        if len == prev_len {
            break;
        }
        prev_len = len;
    }

    input
}

const UNITS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn clean_and_process(input: String) -> usize {
    UNITS
        .iter()
        .map(|&u| {
            let i = input
                .chars()
                .filter(|&x| x.to_ascii_lowercase() != u)
                .collect::<String>();
            process(i).len()
        }).min()
        .unwrap()
}


pub fn run(input: Lines) -> Result<(), Error> {
    let mut input = input;
    let input = input.next().unwrap();

    println!("{}", process("aA"));
    println!("{}", process("abBA"));
    println!("{}", process("abAB"));
    println!("{}", process("aabAAB"));
    println!("{}", process("dabAcCaCBAcCcaDA"));
    println!("I: {}", process(input).len());
    println!("II: {}", clean_and_process(process(input)));

    Ok(())
}
