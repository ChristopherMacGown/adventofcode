use itertools::{join, Itertools};
use request::Error;
use std::fmt::Result as FmtResult;
use std::fmt::{Display, Formatter};
#[allow(unused)]
use std::iter::Peekable;
use std::str::Lines;

#[derive(Debug)]
struct Sky(Vec<Mote>);

impl Sky {
    fn tick(&mut self) {
        self.0.iter_mut().for_each(|m| m.tick())
    }

    fn x_extremes(&self) -> (isize, isize) {
        let x_positions = self.0.iter().map(|m| m.position.0).sorted();
        (x_positions[0], x_positions[x_positions.len() - 1])
    }

    fn y_extremes(&self) -> (isize, isize) {
        let y_positions = self.0.iter().map(|m| m.position.1).sorted();
        (y_positions[0], y_positions[y_positions.len() - 1])
    }

    fn is_coherent(&self) -> bool {
        let (min_y, max_y) = self.y_extremes();

        (max_y - min_y + 1) <= 11
    }
}

impl Display for Sky {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let (x_min, x_max) = self.x_extremes();
        let (y_min, _y_max) = self.y_extremes();

        let positions = self.0.iter().map(|m| m.position).unique();
        let positions = positions.sorted();
        let mut positions = positions.iter().peekable();
        let mut display: Vec<String> = Vec::new();

        // We rotate it so it's easier to display;
        for x in x_min..x_max {
            let mut line = Vec::new();

            for y in 0..10 {
                let mut next = ".";

                if let Some(&&position) = positions.peek() {
                    let comp = Point(x, y + y_min);
                    if position == comp {
                        positions.next();
                        next = "*";
                    }
                }
                line.push(next);
            }

            line.reverse();

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

impl ::std::ops::Mul<isize> for Point {
    type Output = Point;

    fn mul(self, rhs: isize) -> Point {
        Point(self.0 * rhs, self.1 * rhs)
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
