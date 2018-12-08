use request::{get, Error};
use std::collections::HashMap;

pub fn run(input: &str) -> Result<(), Error> {
    let input = get(input)?.text()?;
    let mut seen = HashMap::new();
    let mut running = 0;

    for f in input.lines().cycle() {
        let x = isize::from_str_radix(f, 10).unwrap();
        running += x;

        *seen.entry(running).or_insert(0) += 1;
        // println!("{} {:?}", running, seen.get(&running));
        if *(seen.get(&running).unwrap()) == 2 {
            println!("FRIST {}", running);
            break;
        }
    }

    let frequency: isize = input
        .lines()
        .map(|f| isize::from_str_radix(f, 10).unwrap())
        .sum();

    println!("{}", frequency);
    println!("Merry Christmas!");
    Ok(())
}
