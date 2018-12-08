use request::Error;
use itertools::Itertools;

use std::cmp::Ordering;
use std::str::Lines;
use std::collections::{BTreeSet, BinaryHeap, HashMap};

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

fn part_one(graph: &Graph) -> String {
    walk_graph(graph, 1, 0).0.iter().collect::<String>()
}

fn part_two(graph: &Graph, workers: usize, step: isize) -> usize {
    walk_graph(graph, workers, step).1
}

fn walk_graph(graph: &Graph, workers: usize, step: isize) -> (Vec<char>, usize) {
    let mut tick = 0;
    let mut completed: Vec<char> = Vec::new();
    let mut seen: BTreeSet<char> = BTreeSet::new();
    let mut queue = BinaryHeap::new();
    let mut ready: BTreeSet<char> = graph
        .iter()
        .filter_map(|(&k, v)| if v.is_empty() { Some(k) } else { None })
        .collect();

    while !(ready.is_empty() && queue.is_empty()) {
        while queue.len() < workers && !ready.is_empty() {
            let job = *ready.iter().next().unwrap();
            let cost = step + (job as isize - ADJUST) + 1;

            seen.insert(job);
            ready.remove(&job);
            queue.push(Job(job, cost));
        }

        let Job(finished, running) = queue.pop().unwrap();
        completed.push(finished);
        tick += running;

        queue = queue.iter()
                     .filter_map(|Job(job, cost)| {
                         let cost = cost - running;
                         if cost > 0 {
                             Some(Job(*job, cost))
                         } else {
                             completed.push(*job);
                             None
                         }
                     })
                    .collect();

        for key in graph.keys().sorted() {
            if seen.contains(key) {
                continue;
            }

            if let Some(deps) = graph.get(key) {
                if deps.iter().all(|d| completed.contains(d)) {
                    ready.insert(*key);
                }
            }
        }
    }

    (completed, tick as usize)
}

pub fn run(input: Lines) -> Result<(), Error> {
    let graph = input
        .map(to_edge)
        .fold(HashMap::new(), |mut g, (parent, child)| {
            edge_to_graph(&mut g, parent, child);
            g
        });

    println!("I: {}", part_one(&graph));
    println!("II: {:?}", part_two(&graph, 5, 60));

    Ok(())
}
