use counter::Counter;
use itertools::Itertools;
use request::{get, Error};
use std::collections::HashSet;
use std::iter::Peekable;

const INPUT: &'static str = "https://adventofcode.com/2018/day/6/input";

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point((isize, isize));
impl Point {
    fn from_str<I: Into<String>>(s: I) -> Self {
        let s = s.into();
        let mut split = s.split(", ");

        Point((
            split.next().unwrap().parse::<isize>().unwrap(),
            split.next().unwrap().parse::<isize>().unwrap(),
        ))
    }

    fn x(&self) -> isize {
        let Point(ref lhs) = self;

        lhs.0
    }

    fn y(&self) -> isize {
        let Point(ref lhs) = self;

        lhs.1
    }

    fn distance(&self, rhs: &Point) -> isize {
        let Point(ref lhs) = self;
        let Point(ref rhs) = rhs;

        (lhs.0 - rhs.0).abs() + (lhs.1 - rhs.1).abs() // Manhattan Distance.
    }
}

pub fn run() -> Result<(), Error> {
    let input = get(INPUT)?.text()?;

    let points = input.lines().map(Point::from_str).collect::<Vec<Point>>();
    let points_iter = points.iter();

    let min_x = points_iter
        .clone()
        .min_by(|a, b| a.x().cmp(&b.x()))
        .unwrap()
        .x();
    let max_x = points_iter
        .clone()
        .max_by(|a, b| a.x().cmp(&b.x()))
        .unwrap()
        .x();

    let min_y = points_iter
        .clone()
        .min_by(|a, b| a.y().cmp(&b.y()))
        .unwrap()
        .y();
    let max_y = points_iter
        .clone()
        .max_by(|a, b| a.y().cmp(&b.y()))
        .unwrap()
        .y();

    let mut counter: Counter<Point> = Counter::new();
    let mut counter_fnal: Counter<Point> = Counter::new();
    let mut edges = HashSet::new();
    let mut area = 0;

    fn nearest_by_distance(reference: &Point, point: &Point) -> (isize, Point) {
        (reference.distance(point), *reference)
    }

    for x in (min_x - 10)..(max_x + 10) {
        for y in (min_y - 10)..(max_y + 10) {
            let (_, nearest_fnal) = points_iter
                .clone()
                .map(|reference| nearest_by_distance(reference, &Point((x, y))))
                .unique_by(|&(distance, _)| distance)
                .min_by_key(|&(distance, _)| distance)
                .unwrap();

            let mut minimum_distance = ::std::isize::MAX;
            let mut nearest = None;
            let mut sum = 0;

            for point in points_iter.clone() {
                let distance = point.distance(&Point((x, y)));
                sum += distance;

                if distance < minimum_distance {
                    minimum_distance = distance;
                    nearest = Some(point);
                } else if distance == minimum_distance {
                    nearest = None;
                }
            }

            if sum < 10_000 {
                area += 1;
            }

            *counter_fnal.entry(nearest_fnal).or_insert(0) += 1;

            if nearest.is_some() {
                let nearest = nearest.unwrap();
                *counter.entry(*nearest).or_insert(0) += 1;

                if x <= min_x || x >= max_x || y <= min_y || y >= max_y {
                    edges.insert(nearest);
                }
            }
        }
    }

    println!(
        "I: {:?}",
        counter
            .iter()
            .filter(|(k, _)| !edges.contains(k))
            .max_by_key(|&(_, v)| v)
    );

    println!("II: {:?}", area);

    Ok(())
}
