use std::fs;


fn make_fuel_from_fuel(mut fuel: i64) -> i64 {
    let mut tots = 0;

    while fuel > 0 {
        fuel /= 3;
        fuel -= 2;
        if fuel > 0 {
            tots += fuel;
        }
    }

    tots
}

fn main() {
    let read = fs::read_to_string("../input.txt").unwrap();
    
    println!("{}", read.split("\r\n").map(|c| make_fuel_from_fuel(c.parse::<i64>().unwrap())).sum::<i64>());
}
