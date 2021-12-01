use request::Error;
use std::collections::VecDeque;
use std::str::Lines;

fn play_game(players: usize, max: usize) -> usize {
    let mut board = VecDeque::with_capacity(max);
    let mut scores = vec![0; players];

    board.push_back(0);
    for marble in 1..=max {
        if marble % 23 == 0 {
            for _ in 0..7 {
                let back = board.pop_back().unwrap();

                board.push_front(back);
            }

            scores[marble % players] += marble + board.pop_front().unwrap();
        } else {
            for _ in 0..2 {
                let front = board.pop_front().unwrap();
                board.push_back(front);
            }

            board.push_front(marble);
        }
    }

    *scores.iter().max().unwrap()
}

pub fn run(input: Lines) -> Result<(), Error> {
    let (players, max) = input
        .map(|l| l.split(" ").collect::<Vec<&str>>())
        .map(|l| {
            (
                l[0].parse::<usize>().unwrap(),
                l[6].parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<(usize, usize)>>()[0];

    println!("I: {:?}", play_game(players, max));
    println!("II: {:?}", play_game(players, max * 100));

    Ok(())
}
