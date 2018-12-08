use std::cmp::Ordering;
use itertools::Itertools;
use request::{get, Error};
use std::collections::{BTreeSet, BinaryHeap, HashMap};

const INPUT: &'static str = "https://adventofcode.com/2018/day/7/input";

type Graph = HashMap<char, Vec<char>>;

fn to_edge(line: &str) -> (char, char) {
    let mut chars = line.chars();

    (
        chars.nth(5).expect("parent not a char"),
        chars.nth(30).expect("child not a char"),
    )
}

fn edge_to_graph(graph: &mut Graph, parent: char, child: char) {
    {
        {
            graph.entry(parent).or_insert(Vec::new());
        }
        let ancestors = graph.entry(child).or_insert(Vec::new());
        ancestors.push(parent);
    }
}

fn part_one(graph: &Graph) -> String {
    let mut keys: BTreeSet<char> = graph.keys().cloned().collect();
    let mut satisfied: BTreeSet<char> = BTreeSet::new();
    let mut out: Vec<char> = Vec::new();

    while !keys.is_empty() {
        for key in keys.iter().cloned() {
            let mut ok = true;
            if let Some(deps) = graph.get(&key) {
                ok = deps.iter().all(|d| satisfied.contains(d));
            }

            if ok {
                out.push(key);
                satisfied.insert(key);
                break;
            }
        }

        keys = keys.difference(&satisfied).map(|&c| c).collect();
    }

    out.iter().collect::<String>()
}

const ADJUST: isize = 65; // ASCII 'A';

#[derive(Debug, Eq, PartialEq)]
struct Job(char, isize);
impl Ord for Job {
    fn cmp(&self, other: &Job) -> Ordering {
        other.1.cmp(&self.1)
             .then_with(|| self.0.cmp(&other.0))
    }
}
impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Job) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part_two(graph: &Graph, workers: usize, step: isize) -> usize {
    let mut completed: BTreeSet<char> = BTreeSet::new();
    let mut satisfied: BTreeSet<char> = BTreeSet::new();
    let mut ready: BTreeSet<char> = graph
        .iter()
        .filter_map(|(&k, v)| if v.is_empty() { Some(k) } else { None })
        .collect();
    let mut queue = BinaryHeap::new();

    let mut tick = 0;
    while !(ready.is_empty() && queue.is_empty()) {
        while queue.len() < workers && !ready.is_empty() {
            let job = *ready.iter().next().unwrap();
            let cost = step + (job as isize - ADJUST) + 1;

            satisfied.insert(job);
            ready.remove(&job);
            queue.push(Job(job, cost));
        }

        let Job(finished, running) = queue.pop().unwrap();
        completed.insert(finished);
        tick += running;

        queue = queue.iter()
                     .filter_map(|Job(job, cost)| {
                         let cost = cost - running;
                         if cost > 0 {
                             Some(Job(*job, cost))
                         } else {
                             completed.insert(*job);
                             None
                         }
                     })
                    .collect();

        for key in graph.keys().sorted() {
            if satisfied.contains(key) {
                continue;
            }

            if let Some(deps) = graph.get(key) {
                if deps.iter().all(|d| completed.contains(d)) {
                    ready.insert(*key);
                }
            }
        }
    }

    tick as usize
}

pub fn run() -> Result<(), Error> {
    let input = get(INPUT)?.text()?;
    let graph = input
        .lines()
        .map(to_edge)
        .fold(HashMap::new(), |mut g, (parent, child)| {
            edge_to_graph(&mut g, parent, child);
            g
        });

    println!("I: {}", part_one(&graph));
    println!("II: {}", part_two(&graph, 5, 60));

    Ok(())
}
