use std::{collections::VecDeque, fs};



fn main() {
    let read = fs::read_to_string("input.txt").unwrap();

    let mut vec_deck = read.split("\r\n\r\n")
        .map(|f| f.split("\r\n").skip(1).map(|score| score.parse::<u64>().unwrap()).collect::<VecDeque<_>>()).collect::<Vec<_>>();

    play_the_game(&mut vec_deck);
}

fn play_the_game(players: &mut Vec<VecDeque<u64>>) {
    while !players[0].is_empty() && !players[1].is_empty() {
        let (p1, p2) = (*players[0].front().unwrap(), *players[1].front().unwrap());
        match p1 > p2  {
            true => {
                players[0].pop_front();
                players[1].pop_front();
                players[0].push_back(p1);
                players[0].push_back(p2); 
            },
            _ => {
                players[1].pop_front();
                players[0].pop_front();
                players[1].push_back(p2);
                players[1].push_back(p1);
            }
        }
            
        
    }
    println!("{} {}", players[0].iter().rev().enumerate().fold(0, |acc , (i, cur)| (i+1) as u64 * cur + acc ), players[1].iter().rev().enumerate().fold(0, |acc , (i, cur)| (i+1) as u64 * cur + acc ))
}