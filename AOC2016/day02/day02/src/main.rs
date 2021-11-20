use std::fs;


const KEYPAD: [[i64; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();
    let lines = lines.split("\r\n").collect::<Vec<_>>();

    let (mut y, mut x) = (1, 1);

    for l in lines {
        for letter in l.chars() {
            match letter {
                'U' => { y = (y-1).max(0) }
                'R' => { x = (x + 1).min(2) }
                'L' => { x = (x - 1).max(0) }
                'D' => { y = (y + 1).min(2) }
                _ => {}
            }
        }
        
        print!("{}", KEYPAD[y as usize][x as usize]);
    }

}
