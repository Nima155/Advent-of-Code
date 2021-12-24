use std::fs;
fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let scanners = input.split("\r\n\r\n").map(|data| {
        data.split("\r\n")
            .skip(1)
            .map(|coords| coords.split(',').map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    println!("{:?}", scanners);
}
