use std::fs;

const KEYPAD: [[char; 5]; 5] = [
    [' ', ' ', '1', ' ', ' '],
    [' ', '2', '3', '4', ' '],
    ['5', '6', '7', '8', '9'],
    [' ', 'A', 'B', 'C', ' '],
    [' ', ' ', 'D', ' ', ' '],
];

fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();
    let lines = lines.split("\r\n").collect::<Vec<_>>();

    let (mut y, mut x) = (2, 0);

    for l in lines {
        for letter in l.chars() {
            let (bef_y, bef_x) = (y, x);
            match letter {
                'U' => y = (y - 1).max(0),
                'R' => x = (x + 1).min(4),
                'L' => x = (x - 1).max(0),
                'D' => y = (y + 1).min(4),
                _ => {}
            }
            if KEYPAD[y as usize][x as usize] == ' ' {
                y = bef_y;
                x = bef_x;
            }
        }

        print!("{}", KEYPAD[y as usize][x as usize]);
    }
}
