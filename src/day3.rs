use request::{get, Error};
use std::collections::HashSet;
use std::iter::Peekable;

const INPUT: &'static str = "https://adventofcode.com/2018/day/3/input";

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Hash)]
enum ClaimID {
    Id(usize),
    Overlap,
}

#[derive(Debug)]
struct Claim {
    id: ClaimID,
    location: (usize, usize),
    dimensions: (usize, usize),
}

impl Claim {
    fn corners(&self) -> ((usize, usize), (usize, usize)) {
        (
            self.location,
            (
                self.location.0 + self.dimensions.0,
                self.location.1 + self.dimensions.1,
            ),
        )
    }

    fn from_str<'c>(from: &'c str) -> Result<Self, &'static str> {
        fn parse_number<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> usize {
            let mut number = 0;
            while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<usize>()) {
                number *= 10;
                number += digit;
                iter.next();
            }

            number
        }

        fn parse_claim_id<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> ClaimID {
            ClaimID::Id(parse_number(iter))
        }

        fn parse_claim_location<I: Iterator<Item = char>>(
            iter: &mut Peekable<I>,
        ) -> (usize, usize) {
            let x = parse_number(iter);

            if let Some(',') = iter.peek() {
                iter.next();
            }

            let y = parse_number(iter);

            (x, y)
        }

        fn parse_claim_dimensions<I: Iterator<Item = char>>(
            iter: &mut Peekable<I>,
        ) -> (usize, usize) {
            let x = parse_number(iter);

            if let Some('x') = iter.peek() {
                iter.next();
            }

            let y = parse_number(iter);

            (x, y)
        }

        let mut id: Option<ClaimID> = None;
        let mut location: Option<(usize, usize)> = None;
        let mut dimensions: Option<(usize, usize)> = None;

        let mut iterator = from.chars().peekable();
        while let Some(&ch) = iterator.peek() {
            match ch {
                '#' => {
                    iterator.next();
                    id = Some(parse_claim_id(&mut iterator));
                }
                ' ' => {
                    iterator.next();
                }
                '@' => {
                    iterator.next();
                    iterator.next(); // WHITESPACE
                    location = Some(parse_claim_location(&mut iterator));
                }
                ':' => {
                    iterator.next();
                    iterator.next(); // WHITESPACE
                    dimensions = Some(parse_claim_dimensions(&mut iterator));
                }

                _ => {
                    println!("{:?}", iterator.peek());
                    unreachable!()
                }
            }
        }

        if let Some(id) = id {
            if let Some(location) = location {
                if let Some(dimensions) = dimensions {
                    return Ok(Claim {
                        id,
                        location,
                        dimensions,
                    });
                }
            }
        }

        Err("unable to parse claim")
    }
}

const WIDTH: usize = 1000;
struct Fabric {
    claims: HashSet<ClaimID>,
    intersecting: HashSet<ClaimID>,
    fabric: Box<[Option<ClaimID>]>,
}
impl Fabric {
    fn new() -> Self {
        Fabric {
            claims: HashSet::new(),
            intersecting: HashSet::new(),
            fabric: vec![None; 1000 * 1000].into_boxed_slice(),
        }
    }

    fn insert(&mut self, claim: &Claim) {
        let (top_left, bottom_right) = claim.corners();

        self.claims.insert(claim.id);
        for x in (top_left.0)..(bottom_right.0) {
            for y in (top_left.1)..(bottom_right.1) {
                if let Some(elem) = self.fabric.get_mut(x * WIDTH + y) {
                    match elem {
                        None => *elem = Some(claim.id),
                        Some(mut conflict) => {
                            self.intersecting.insert(conflict);
                            self.intersecting.insert(claim.id);

                            *elem = Some(ClaimID::Overlap);
                        }
                    }
                }
            }
        }
    }

    fn difference(&self) -> Option<&ClaimID> {
        self.claims.difference(&self.intersecting).next()
    }

    fn area(&self, claim_id: ClaimID) -> usize {
        self.fabric
            .iter()
            .filter_map(|x| *x)
            .filter(|c| *c == claim_id)
            .count()
    }
}

pub fn run() -> Result<(), Error> {
    let mut fabric = Fabric::new();
    let claims = get(INPUT)?.text()?;
    claims
        .lines()
        .map(Claim::from_str)
        .filter_map(|c| c.ok())
        .for_each(|c| fabric.insert(&c));

    println!("I: {:?}", fabric.area(ClaimID::Overlap));
    println!("II: {:?}", fabric.difference());

    Ok(())
}
