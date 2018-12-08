use levenshtein::levenshtein;
use request::{get, Error};
use std::collections::HashMap;
use std::str;

fn count_seen_characters(barcode: &str) -> HashMap<char, usize> {
    let mut seen = HashMap::new();

    barcode.chars().for_each(|c| {
        *seen.entry(c).or_insert(0) += 1;
    });

    seen
}

fn checksum(list: &str) -> usize {
    let checksum = list
        .lines()
        .map(|l| {
            let mut two = 0;
            let mut three = 0;
            let seen = count_seen_characters(l);
            if seen.values().any(|&x| x == 2) {
                two = 1;
            }
            if seen.values().any(|&x| x == 3) {
                three = 1;
            }

            (two, three)
        }).fold((0, 0), |acc, next| (acc.0 + next.0, acc.1 + next.1));

    checksum.0 * checksum.1
}

fn find_levenshtein(list: &str) -> String {
    let mut first: &str = "";
    let mut second: &str = "";
    let lines = list.lines();

    'outer: for line in lines {
        let mut offset = list.lines();
        offset.next();

        'inner: for comparator in offset {
            let distance = levenshtein(line, comparator);
            if distance == 0 {
                break 'inner;
            }

            if distance == 1 {
                first = line;
                second = comparator;
                break 'outer;
            }
        }
    }

    // TODO - actually calculate that.
    println!("{} {}", first, second);

    "FOO".to_string()
}

pub fn run(input: &str) -> Result<(), Error> {
    let boxes = get(input)?.text()?;

    println!("CHECKSUM: {}", checksum(&boxes));
    println!("LEVENSHTEIN: {}", find_levenshtein(&boxes));

    Ok(())
}
