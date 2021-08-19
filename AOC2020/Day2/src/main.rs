use std::fs;
fn main() {
    let passwords = fs::read_to_string("input.txt").unwrap();

    let disected_passwords = passwords
        .split("\r\n")
        .map(|e| e.split(' ').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!(
        "{}",
        disected_passwords
            .iter()
            .filter(|e| {
                let range = e[0].split('-').collect::<Vec<_>>(); // range
                let letter = e[1].chars().next().unwrap();
                let low = range[0].parse::<usize>().unwrap();
                let high = range[1].parse::<usize>().unwrap();
                let counts = e[2].chars().filter(|z| *z == letter).count();
                counts >= low && counts <= high
            })
            .count()
    );
}
