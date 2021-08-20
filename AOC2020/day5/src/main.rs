// Start by considering the whole range, rows 0 through 127.
// F means to take the lower half, keeping rows 0 through 63.
// B means to take the upper half, keeping rows 32 through 63.
// F means to take the lower half, keeping rows 32 through 47.
// B means to take the upper half, keeping rows 40 through 47.
// B keeps rows 44 through 47.
// F keeps rows 44 through 45.
// The final F keeps the lower of the two, row 44.
use std::{cmp::max, fs};

fn search_assign(start: &mut i32, end: &mut i32, c: char, mid: i32) {
    match c {
        'B' | 'R' => *start = mid + 1,
        'F' | 'L' => *end = mid,
        _ => {}
    };
}

fn loop_through_instructions(instructions: &str, mut start: i32, mut end: i32) -> i32 {
    for c in instructions.chars() {
        let middle = (start + end) / 2;
        search_assign(&mut start, &mut end, c, middle);
    }
    match instructions.chars().last().unwrap() {
        'B' | 'R' => end,
        _ => start,
    }
}

// 8 => columns
fn main() {
    let read = fs::read_to_string("input.txt").unwrap();
    let mut max_ans = 0;
    for seat_info in read.split("\r\n") {
        let row_instructions = &seat_info[..seat_info.len() - 3];
        let row_ans = loop_through_instructions(row_instructions, 0, 127);
        let col_instructions = &seat_info[seat_info.len() - 3..];
        let col_ans = loop_through_instructions(col_instructions, 0, 7);
        // println!("{}", col_end);
        max_ans = max(max_ans, row_ans * 8 + col_ans);
        println!("{} {} {}", row_ans, col_ans, row_ans * 8 + col_ans);
    }
    println!("{}", max_ans)
}
