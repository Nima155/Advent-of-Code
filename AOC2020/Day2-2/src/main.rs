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

                let first_index = range[0].parse::<usize>().unwrap();
                let sec_index = range[1].parse::<usize>().unwrap();

                let first_letter = e[2].chars().nth(first_index - 1).unwrap();
                let second_letter = e[2].chars().nth(sec_index - 1).unwrap();
                // println!("{} {} {} {} {}",first_index, first_letter, sec_index, second_letter, e[2]);
                first_letter == letter && second_letter != letter || first_letter != letter && second_letter == letter

            })
            .count()
    );
}
