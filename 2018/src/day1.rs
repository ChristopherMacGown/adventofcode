use request::Error;
use std::collections::HashMap;
use std::str::Lines;

pub fn run(input: Lines) -> Result<(), Error> {
    let mut seen = HashMap::new();
    let mut running = 0;

    for f in input.clone().cycle() {
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
        .map(|f| isize::from_str_radix(f, 10).unwrap())
        .sum();

    println!("{}", frequency);
    println!("Merry Christmas!");
    Ok(())
}
