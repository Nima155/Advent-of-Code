use std::{collections::{VecDeque, HashMap, HashSet}, fs, mem::swap, ops::RangeBounds};
fn main() {
    let board = fs::read_to_string("../input.txt").unwrap();
    let board = board
        .split("\r\n")
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!("{}", bfs(&board));
}
// #############
// #...........#
// ###D#B#C#C###
//   #D#A#B#A#
//   #########
const MOVES: [[i64; 2]; 4] = [[0, 1], [1, 0], [-1, 0], [0, -1]];

fn bfs(board: &Vec<Vec<char>>) -> i64 {
    let mut queue = VecDeque::new();
    queue.push_back((board.clone(), 0));

    let permissible_columns = [3, 5, 7, 9];
    let mut visited = HashSet::new();
    while !queue.is_empty() {
        let (board, steps) = queue.pop_front().unwrap();
        
        let mut change_needed = 0;
        for (i, r) in board.iter().enumerate() {
            for (j, c) in r.iter().enumerate() {
                if c.is_alphabetic() {
                    for [y, x] in MOVES {
                        let [ny, nx] = [i as i64 + y, j as i64 + x];
                        if ny >= 0
                            && nx >= 0
                            && ny < board.len() as i64
                            && nx < board[ny as usize].len() as i64
                            && board[ny as usize][nx as usize] == '.'
                        {
                            let mut clone_board = board.clone();
                            let tmp = clone_board[i][j];
                            if y == 1 && permissible_columns[(*c as u8 - b'A') as usize] == nx {
                                clone_board[i][j] = '.';
                                clone_board[ny as usize][nx as usize] = tmp;
                                if !visited.contains(&clone_board) {
                                    visited.insert(clone_board.clone());
                                    queue.push_back((
                                        clone_board,
                                        steps + 10_i64.pow((*c as u8 - b'A') as u32),
                                    ));

                                    change_needed = 1;
                                }
                            } else if y != 1 {
                                
                                clone_board[i][j] = '.';
                                clone_board[ny as usize][nx as usize] = tmp;
                                if clone_board[ny as usize + 1][nx as usize] != '#' || ny == 2 {
                                    change_needed = 1;
                                    let fr = clone_board[ny as usize][nx as usize + 1];
                                    let back = clone_board[ny as usize][nx as usize - 1];
                                    if fr == '.' {
                                        clone_board[ny as usize].swap(nx as usize, (nx + 1) as usize);
                                        if !visited.contains(&clone_board) {
                                            queue.push_back((
                                                clone_board.clone(),
                                                steps + 10_i64.pow((*c as u8 - b'A') as u32),
                                            ));
                                            visited.insert(clone_board.clone());
                                            change_needed = 1;
                                        }
                                        // queue.push_back((clone_board.clone(), steps + 10_i64.pow((*c as u8 - b'A') as u32)));
                                        clone_board[ny as usize].swap(nx as usize, (nx + 1) as usize);
                                    } 
                                    if back == '.' {
                                        clone_board[ny as usize].swap(nx as usize, (nx - 1) as usize);
                                        if !visited.contains(&clone_board) {
                                            queue.push_back((
                                                clone_board.clone(),
                                                steps + 10_i64.pow((*c as u8 - b'A') as u32),
                                            ));
                                            visited.insert(clone_board.clone());
                                            change_needed = 1;
                                        }
                                        clone_board[ny as usize].swap(nx as usize, (nx - 1) as usize);
                                        // queue.push_back((clone_board.clone(), steps + 10_i64.pow((*c as u8 - b'A') as u32)));
                                    }
                                    else {
                                        if !visited.contains(&clone_board) {
                                            queue.push_back((
                                                clone_board.clone(),
                                                steps + 10_i64.pow((*c as u8 - b'A') as u32),
                                            ));
                                            visited.insert(clone_board);
                                            change_needed = 1;
                                        }
                                    }
                                }
                            } 

                        }
                    }
                }
            }
        }
        if change_needed == 0 {
            println!("{}", board.iter().map(|v| v.iter().collect::<String>()).collect::<Vec<_>>().join("\n"));
            return steps;
        }
    }
    0
}

