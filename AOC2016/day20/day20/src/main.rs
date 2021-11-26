use std::fs;
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();

    let mut ranges = lines
        .split("\r\n")
        .map(|e| {
            e.split('-')
                .map(|f| f.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    ranges.sort();

    let mut last_high = 0;

    for low_high in &ranges {
        if low_high[0] > (last_high + 1) {
            println!("{}", last_high + 1);
            break;
        }
        last_high = i64::max(last_high, low_high[1]);
    }
}
