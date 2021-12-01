use itertools::{join, Itertools};
use request::Error;
use std::collections::HashSet;
use std::fmt::Result as FmtResult;
use std::fmt::{Display, Formatter};
use std::str::Lines;

enum Axis {
    X,
    Y,
}

#[derive(Debug)]
struct Sky(Vec<Mote>);

impl Sky {
    fn tick(&mut self) {
        self.0.iter_mut().for_each(|m| m.tick())
    }

    fn extremes(&self, axis: Axis) -> (isize, isize) {
        let positions = self
            .0
            .iter()
            .map(|m| match axis {
                Axis::X => m.position.0,
                Axis::Y => m.position.1,
            })
            .sorted();

        (positions[0], positions[positions.len() - 1])
    }

    fn is_coherent(&self) -> bool {
        let (min_y, max_y) = self.extremes(Axis::Y);

        // Discovered during development that when all of the motes are coherent
        // with one another all of the characters in the message are 10 motes high.
        (max_y - min_y + 1) <= 11
    }
}

impl Display for Sky {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let positions: HashSet<Point> = self.0.iter().map(|m| m.position).collect();
        let (x_min, x_max) = self.extremes(Axis::X);
        let (y_min, _y_max) = self.extremes(Axis::Y);

        let mut display: Vec<String> = Vec::new();

        for y in 0..10 {
            let mut line = Vec::new();
            for x in x_min..x_max + 2 {
                line.push(match positions.contains(&Point(x, y + y_min)) {
                    false => ".",
                    true => "*",
                });
            }

            display.push(line.concat());
        }

        write!(f, "{}", join(display, "\n"))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point(isize, isize);
impl ::std::ops::AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[derive(Debug)]
struct Mote {
    position: Point,
    velocity: Point,
}

impl Mote {
    fn parse(input: &str) -> Self {
        fn parse_point(input: &str) -> Point {
            let coords: Vec<isize> = input
                .split(", ")
                .map(|v| v.trim())
                .map(|v| v.parse::<isize>().unwrap())
                .collect();

            Point(coords[0], coords[1])
        }
        let pos = &input[10..24];
        let vel = &input[36..input.len() - 1];

        Mote {
            position: parse_point(pos),
            velocity: parse_point(vel),
        }
    }

    fn tick(&mut self) {
        self.position += self.velocity;
    }
}

pub fn run(input: Lines) -> Result<(), Error> {
    let mut motes: Sky = Sky(input.map(|l| Mote::parse(l)).collect());
    let mut tick = 0;

    while !motes.is_coherent() {
        motes.tick();
        tick += 1;
    }

    println!("ticks: {}", tick);
    println!("message:\n{}", motes);

    Ok(())
}
