use std::fs;
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();
    let lines = lines.split("\r\n").map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let (mut prev, mut count) = (-1, 0);
    for w in lines.windows(3) {
        let reduced = w.iter().fold(0, |up_till, now| up_till + now);
        if reduced > prev {
            count += 1;
        }
        prev = reduced;


    }
    println!("{}", count - 1);
}
