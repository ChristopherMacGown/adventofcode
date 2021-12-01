use chrono::{NaiveDate, NaiveDateTime, Timelike};
use counter::Counter;
use request::Error;

use std::str::Lines;
use std::collections::HashMap;

const DATETIME_FORMAT: &'static str = "[%Y-%m-%d %H:%M";

#[derive(Debug, Hash)]
enum Message {
    GuardID(usize),
    FallsAsleep,
    WakesUp,
}

#[derive(Debug)]
struct Record {
    timestamp: NaiveDateTime,
    message: Message,
}

impl Record {
    fn from_str<'c>(from: &'c str) -> Self {
        let split: Vec<&str> = from.split("] ").collect();
        let timestamp = NaiveDateTime::parse_from_str(split[0], DATETIME_FORMAT).unwrap();

        let message = match split[1].get(0..5) {
            Some("Guard") => {
                let split: Vec<&str> = split[1].split("#").collect();
                let split: Vec<&str> = split[1].split(" begins").collect();
                let id = split[0].parse::<usize>().unwrap();

                Message::GuardID(id)
            }
            Some("falls") => Message::FallsAsleep,
            Some("wakes") => Message::WakesUp,
            _ => unreachable!(),
        };

        Record { timestamp, message }
    }
}

pub fn run(input: Lines) -> Result<(), Error> {
    let mut schedule: Vec<Record> = input.map(Record::from_str).collect();
    let mut sleeping: HashMap<usize, Counter<i64>> = HashMap::new();

    schedule.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    let mut guard: usize = 0;
    let mut start: NaiveDateTime = NaiveDate::from_ymd(800, 1, 1).and_hms(0, 0, 0);
    for record in schedule {
        match record.message {
            Message::GuardID(id) => guard = id,
            Message::FallsAsleep => start = record.timestamp,
            Message::WakesUp => {
                let s = start.time().minute() as i64;
                let e = record.timestamp.signed_duration_since(start).num_minutes();

                sleeping
                    .entry(guard)
                    .or_insert(Counter::new())
                    .update((s..(s + e)).map(|x| x).collect::<Vec<i64>>());
            }
        }
    }

    let strategy1 = sleeping
        .iter()
        .map(|(id, counter)| (id, counter.values().sum::<usize>()))
        .max_by_key(|(_, counter)| *counter)
        .unwrap()
        .0;
    let strategy2 = sleeping
        .iter()
        .map(|(id, counter)| (id, counter.most_common_ordered()[0]))
        .max_by_key(|(_, counter)| counter.1)
        .unwrap();

    let s1_hour = sleeping.get(strategy1).unwrap().most_common_ordered()[0].0;

    println!(
        "I:\t{} * {} = {}",
        strategy1,
        s1_hour,
        *strategy1 as i64 * s1_hour
    );
    println!(
        "I::\t{} * {} = {}",
        strategy2.0,
        (strategy2.1).0,
        *strategy2.0 as i64 * (strategy2.1).0
    );

    Ok(())
}
