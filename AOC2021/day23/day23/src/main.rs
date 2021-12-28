use std::{
    collections::{ HashMap, VecDeque},
    fs,
};

fn main() {
    let board = fs::read_to_string("../input.txt").unwrap();
    let board = board
        .split("\r\n")
        .map(|l| {
            let mut res = l.chars().collect::<Vec<_>>();
            while res.len() < 13 {
                res.push(' ');
            }
            res
        })
        .collect::<Vec<_>>();
    println!("{}", bfs(&board));
}

const FINAL_STATE: [[char; 13]; 7] = [
    [
        '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
    ],
    [
        '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#',
    ],
    [
        '#', '#', '#', 'A', '#', 'B', '#', 'C', '#', 'D', '#', '#', '#',
    ],
    [
        ' ', ' ', '#', 'A', '#', 'B', '#', 'C', '#', 'D', '#', ' ', ' ',
    ],
    [
        ' ', ' ', '#', 'A', '#', 'B', '#', 'C', '#', 'D', '#', ' ', ' ',
    ],
    [
        ' ', ' ', '#', 'A', '#', 'B', '#', 'C', '#', 'D', '#', ' ', ' ',
    ],
    [
        ' ', ' ', '#', '#', '#', '#', '#', '#', '#', '#', '#', ' ', ' ',
    ],
];
const PERMISSIBLE_COLUMNS: [usize; 4] = [3, 5, 7, 9];
fn bfs(board: &[Vec<char>]) -> i64 {
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((0, board.to_owned()));
    let mut ans = i64::MAX;
    while !queue.is_empty() {
        let (steps, board) = queue.pop_front().unwrap();
        // println!
        if board == FINAL_STATE {
            ans = i64::min(ans, steps);
            continue;
        }

        for (i, r) in board.iter().enumerate() {
            for (j, c) in r.iter().enumerate() {
                if *c as u8 >= b'A' && *c as u8 <= b'D' {
                    let mut possib = vec![];

                    if (i == 1
                        && is_viable(&board, PERMISSIBLE_COLUMNS[(*c as u8 - b'A') as usize], *c)
                        && can_move_to(&board, j, PERMISSIBLE_COLUMNS[(*c as u8 - b'A') as usize]))
                        || (i != 1 && not_right_place(&board, i, *c, j))
                    {
                        possib = make_move(&board, i, j);
                    }

                    for (v, costs) in possib {
                        if !visited.contains_key(&v) || (steps + costs) < *visited.get(&v).unwrap()
                        {
                            visited.insert(v.clone(), costs + steps);
                            queue.push_back((costs + steps, v));
                        }
                    }
                }
            }
        }
    }

    ans
}

fn make_move(board: &[Vec<char>], mut i: usize, j: usize) -> Vec<(Vec<Vec<char>>, i64)> {
    let board = board.to_owned();
    let c = board[i][j];
    let mut ans = vec![];
    let steps_step = 10_i64.pow(c as u32 - b'A' as u32);
    let perm_col = PERMISSIBLE_COLUMNS[(board[i][j] as u8 - b'A') as usize] as i64;
    if i == 1 {
        let [col_s, row_s] = [(perm_col - j as i64).abs(), i as i64];
        while i < 6 && board[i][perm_col as usize] == '.' {
            i += 1;
        }
        i -= 1;
        let mut board = board;
        board[i][perm_col as usize] = c;
        board[row_s as usize][j] = '.';
        ans.push((board, steps_step * (col_s + i as i64 - row_s)))
    } else {
        let mut new_i = i - 1;
        while new_i > 1 && board[new_i][j] == '.' {
            new_i -= 1;
        }
        let costo = (i - new_i) as i64 * steps_step;
        if new_i == 1 && board[new_i][j] == '.' {
            let [mut b_j, mut f_j] = [j - 1, j + 1];

            while board[new_i][b_j] == '.' && b_j >= 1 {
                if hallway_is_correct(&board, b_j) {
                    let mut board_c = board.clone();
                    board_c[new_i][b_j] = c;
                    board_c[i][j] = '.';
                    ans.push((board_c, steps_step * (j - b_j + i - new_i) as i64))
                } else if b_j as i64 == perm_col
                    && is_viable(&board, b_j, c)
                    && board[new_i + 1][b_j] == '.'
                {
                    let mut ii = new_i;
                    while ii < 6 && board[ii][perm_col as usize] == '.' {
                        ii += 1;
                    }
                    ii -= 1;
                    if board[ii][perm_col as usize] == '.' {
                        let mut board = board.clone();
                        board[ii][perm_col as usize] = c;
                        board[i][j] = '.';
                        ans.push((board, steps_step * (j - b_j + ii - new_i) as i64 + costo));
                    }
                }

                b_j -= 1;
            }

            while board[new_i][f_j] == '.' && f_j <= 11 {
                if hallway_is_correct(&board, f_j) {
                    let mut board_c = board.clone();
                    board_c[new_i][f_j] = c;
                    board_c[i][j] = '.';
                    ans.push((board_c, steps_step * (f_j - j + i - new_i) as i64))
                } else if f_j as i64 == perm_col
                    && is_viable(&board, f_j, c)
                    && board[new_i + 1][f_j] == '.'
                {
                    let mut ii = new_i;
                    while ii < 6 && board[ii][perm_col as usize] == '.' {
                        ii += 1;
                    }
                    ii -= 1;

                    if board[ii][perm_col as usize] == '.' {
                        let mut board = board.clone();
                        board[ii][perm_col as usize] = c;
                        board[i][j] = '.';
                        ans.push((board, steps_step * (f_j - j + ii - new_i) as i64 + costo));
                    }
                }

                f_j += 1;
            }
        }
    }
    ans
}

fn hallway_is_correct(board: &[Vec<char>], j: usize) -> bool {
    if board[2][j] != '#' {
        return false;
    }
    true
}

fn not_right_place(board: &[Vec<char>], start_r: usize, c: char, column: usize) -> bool {
    !(board[start_r..6].iter().all(|r| r[column] == c)
        && PERMISSIBLE_COLUMNS[(c as u8 - b'A') as usize] == column)
}

fn can_move_to(board: &[Vec<char>], mut j: usize, target_col: usize) -> bool {
    match j > target_col {
        true => {
            j -= 1;
        }
        _ => j += 1,
    }

    while j > target_col && board[1][j] == '.' {
        j -= 1;
    }

    while j < target_col && board[1][j] == '.' {
        j += 1;
    }

    j == target_col && board[1][j] == '.' && board[2][j] == '.'
}

fn is_viable(board: &[Vec<char>], column: usize, target: char) -> bool {
    board[2..6]
        .iter()
        .all(|r| r[column] == target || r[column] == '.')
}
