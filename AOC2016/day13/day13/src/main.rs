use std::collections::{HashSet, VecDeque};

const FAVORITE_NUMBER: i64 = 1364;
const MOVES: [[i64; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];
fn main() {
    // 31, 39
    println!("{}", minimum_to_destination(31, 39))
}

fn is_wall(x: i64, y: i64) -> bool {
    let mut number = x * x + 3 * x + 2 * x * y + y + y * y + FAVORITE_NUMBER;
    let mut set_bits = 0;
    while number != 0 {
        number &= number - 1;
        set_bits += 1;
    }
    set_bits % 2 != 0 
   
}
fn minimum_to_destination(targ_x: i64, targ_y: i64) -> usize {
    let mut vecdeck = VecDeque::new();
    let mut visited = HashSet::new();
    vecdeck.push_back((1, 1, 0));

    while !vecdeck.is_empty() {
        let (y, x, steps) = vecdeck.pop_front().unwrap();
        if y == targ_y && x == targ_x {
            return steps;
        }
        for [yy, xx] in MOVES {
            let (ny, nx) = (yy + y, x + xx);
            if ny >= 0 && nx >= 0 && !visited.contains(&(ny, nx)) && !is_wall(nx, ny) {
                visited.insert((ny, nx));
                vecdeck.push_back((ny, nx, steps + 1));
            }
        }
    }
    0
}
