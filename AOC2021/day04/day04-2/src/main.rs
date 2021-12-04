use std::{collections::HashSet, fs};
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();

    let lines = lines.split("\r\n\r\n").collect::<Vec<_>>();

    let draws = lines[0]
        .split(',')
        .map(|f| f.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let boards = lines[1..]
        .iter()
        .map(|f| {
            f.split("\r\n")
                .map(|l| {
                    l.split_ascii_whitespace()
                        .map(|n| n.trim().parse::<i64>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{}", process_the_game(&draws, &boards));
}

const MARKED: i64 = 1000;
fn process_the_game(draws: &[i64], boards: &Vec<Vec<Vec<i64>>>) -> i64 {
    let mut boardz = boards.clone();
    let mut last_winner_score = 0;
    let mut winners = HashSet::new();
    for draw in draws {
        for (i, board) in boards.iter().enumerate() {
            for (j, r) in board.iter().enumerate() {
                for (x, n) in r.iter().enumerate() {
                    if *n == *draw {
                        boardz[i][j][x] = MARKED;
                        if let Some(sum) = is_winner(&boardz[i]) {
                            if !winners.contains(&i) {
                                winners.insert(i);
                                last_winner_score = sum * *n;
                            }
                        }
                    }
                }
            }
        }
    }
    last_winner_score
}

fn is_winner(board: &[Vec<i64>]) -> Option<i64> {
    let mut valid = false;

    for i in 0..5 {
        let mut count_marked = 0;
        for j in 0..5 {
            count_marked += (board[j][i] == MARKED) as i64;
        }
        valid |= count_marked == 5;
    }

    valid |= board.iter().any(|r| r.iter().all(|f| *f == MARKED));

    if valid {
        Some(
            board
                .iter()
                .map(|r| r.iter().filter(|f| **f != MARKED).collect::<Vec<_>>())
                .fold(0, |acc, cur| acc + cur.into_iter().sum::<i64>()),
        )
    } else {
        None
    }
}
