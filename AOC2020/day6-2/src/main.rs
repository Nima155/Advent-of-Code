use std::{collections::HashMap, fs};
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut vec_of_sets: Vec<HashMap<char, u8>> = vec![HashMap::new(); 500];

    println!(
        "{}",
        input
            .split("\r\n\r\n")
            .enumerate()
            .map(|(i, e)| {
                let mut number_in_group = 0;
                e.split("\r\n").for_each(|s| {
                    number_in_group += 1;
                    s.chars().for_each(|c| {
                        let counter = vec_of_sets[i].entry(c).or_insert(0);
                        *counter += 1;
                    });
                    vec_of_sets[i] = vec_of_sets[i]
                        .iter()
                        .filter(|(_k, v)| **v == number_in_group)
                        .map(|(k, v)| (*k, *v))
                        .collect();
                });
                vec_of_sets[i].len()
            })
            .reduce(std::ops::Add::add)
            .unwrap()
    );
}
