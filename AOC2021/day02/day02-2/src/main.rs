use std::fs;
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();

    let instructions = lines
        .split("\r\n")
        .map(|f| {
            let mut split_up = f.split(' ');
            let dir = split_up.next().unwrap();
            let by = split_up.next().unwrap().parse::<i64>().unwrap();
            match dir {
                "forward" => (0, by),
                "down" => (by, 0),
                "up" => (-by, 0),
                _ => (0, 0),
            }
        })
        .collect::<Vec<_>>();

    let (mut y, mut x, mut aim_pos) = (0, 0, 0);

    for (yy, xx) in &instructions {
        if *xx != 0 {
            y += aim_pos * *xx;
        }
        aim_pos += *yy;
        x += *xx;
    }
    println!("{}", y * x);
}
