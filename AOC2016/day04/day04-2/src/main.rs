use std::fs;
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();

    let lines = lines.split("\r\n").collect::<Vec<_>>();

    for l in lines {
        room_sum(l);
    }
}

fn room_sum(line: &str) {
    let split_up = line.split('-').collect::<Vec<_>>();

    let conct = split_up[..split_up.len() - 1].join(" ");

    let last_split = split_up.last().unwrap().split('[').collect::<Vec<_>>();
    let rotate_by = last_split[0].parse::<i64>().unwrap();

    let mapped = conct
        .chars()
        .map(|c| {
            if c != ' ' {
                ((((c as i64 - b'a' as i64) + rotate_by) % 26) as u8 + b'a') as char
            } else {
                c
            }
        })
        .collect::<String>();

    if mapped == "northpole object storage" {
        println!("{}", rotate_by);
    }
}
