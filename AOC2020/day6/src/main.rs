use std::{collections::HashSet, fs};
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut vec_of_sets: Vec<HashSet<char>> = vec![HashSet::new();500];
    
    println!("{}", input.split("\r\n\r\n").enumerate().map(|(i, e)| {
        
        e.split("\r\n").for_each(|s| s.chars().for_each(|c| {
            vec_of_sets[i].insert(c);
        }));
        vec_of_sets[i].len()
    }).reduce(std::ops::Add::add).unwrap());

}
