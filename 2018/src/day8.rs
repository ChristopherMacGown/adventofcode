use request::Error;
use std::str::Lines;
use std::iter::{Iterator, Peekable};


type Metadata = usize;

#[derive(Debug)]
struct Header(usize, usize);

impl Header {
    fn parse<I: Iterator<Item=usize>>(iter: &mut Peekable<I>) -> Header {
        let child_count = iter.next().expect("Expected a child count");
        let meta_count = iter.next().expect("Expected a meta count");

        Header(child_count, meta_count)
    }
}

#[derive(Debug)]
struct Node {
    header: Header,
    children: Vec<Node>,
    metadata: Vec<Metadata>,
}

impl Node {
    fn parse<I: Iterator<Item=usize>>(iter: &mut Peekable<I>) -> Self {
        let header = Header::parse(iter);
        let children = (0..header.0).map(|_| Node::parse(iter)).collect();
        let metadata = (0..header.1).map(|_| iter.next().unwrap()).collect();

        Node {
            header,
            children,
            metadata,
        }
    }

    fn walk(&self) -> Vec<&Node> {
        let mut root = vec![self];
        match self.children.as_slice() {
            [] => root,
            _ => {
                let children: Vec<&Node> = self.children.iter().flat_map(|c| c.walk()).collect();
                root.extend(children);
                root
            }
        }
    }

    fn value(&self) -> usize {
        let mut sum: usize = 0;

        match self.children.as_slice() {
            [] => sum += self.metadata.iter().map(|&m| m).sum::<usize>(),
            _ => {
                for idx in self.metadata.iter() {
                    if let Some(child) = self.children.get(*idx - 1) {
                        sum += child.value();
                    }
                }
            }
        }

        sum
    }
}

pub fn run(input: Lines) -> Result<(), Error> {
//    let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".lines();
    let input: Vec<usize> = input
        .flat_map(|l| l.split(" "))
        .filter_map(|c| c.parse::<usize>().ok())
        .collect();

    let tree = Node::parse(&mut input.into_iter().peekable());
    let part_one: Vec<usize> = tree.walk().iter().flat_map(|n| n.metadata.clone()).collect();
    let part_two: usize = tree.value();

    println!("I: {:?}", part_one.into_iter().sum::<usize>());
    println!("II: {:?}", part_two);

    Ok(())
}
