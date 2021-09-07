mod gameplay;
use std::collections::{HashSet, VecDeque};
use std::fs;

use gameplay::play_the_long_game;
fn main() {
    let read = fs::read_to_string("input.txt").unwrap();

    let mut vec_deck = read
        .split("\r\n\r\n")
        .map(|f| {
            f.split("\r\n")
                .skip(1)
                .map(|score| score.parse::<u64>().unwrap())
                .collect::<VecDeque<_>>()
        })
        .collect::<Vec<_>>();

    

    println!(
        "{:?}",
        play_the_long_game(&mut vec_deck, &mut HashSet::new())
    );
}
