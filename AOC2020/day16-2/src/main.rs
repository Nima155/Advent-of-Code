mod input_parser;

use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

use crate::input_parser::{recursive_field_assignment, vec_if_valid};

fn main() {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"(\d+?)[-](\d+)"#).unwrap();
    };
    let mut all_valid_nums: HashMap<&str, HashSet<usize>> = HashMap::new();
    let mut all_valids_together: [bool; 1000] = [false; 1000];

    let mut valid_tickets = Vec::with_capacity(100);

    let read = fs::read_to_string("input.txt").unwrap();

    let (mut others, mut me, mut my_line) = (false, false, "");

    for line in read.split("\r\n") {
        others |= line.contains("nearby");
        me |= line.contains("your");
        if RE.is_match(line) {
            let parsed_field = line.split(':').collect::<Vec<_>>()[0];
            for gr in RE.captures_iter(line) {
                let l: usize = gr.get(1).unwrap().as_str().parse().unwrap();
                let r = gr.get(2).unwrap().as_str().parse().unwrap();

                (l..=r).for_each(|n| {
                    all_valids_together[n] = true;
                    if let Some(set) = all_valid_nums.get_mut(parsed_field) {
                        set.insert(n);
                    } else {
                        all_valid_nums.insert(parsed_field, HashSet::new());
                        all_valid_nums.get_mut(parsed_field).unwrap().insert(n);
                    }
                })
            }
        } else if line.contains(',') && (others || me) {
            if me {
                me = false;
                my_line = line;
            } else if let Some(v) = vec_if_valid(line, &all_valids_together) {
                valid_tickets.push(v);
            }
        }
    }
    // println!("{:?}", valid_tickets);
    let mut my_map = HashMap::new();
    let mut seen = HashSet::new();
    recursive_field_assignment(&valid_tickets, &all_valid_nums, &mut my_map, &mut seen);

    println!(
        "{}",
        my_line
            .split(',')
            .enumerate()
            .filter(|(i, _)| {
                my_map
                    .get(i)
                    .unwrap()
                    .iter()
                    .next()
                    .unwrap()
                    .starts_with("departure")
            })
            .map(|(_, e)| e.parse::<usize>().unwrap())
            .product::<usize>()
    );
}
