use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashSet, fs};

fn main() {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"(\d+?)[-](\d+)"#).unwrap();
    };
    let mut all_valid_nums = HashSet::new();

    let read = fs::read_to_string("input.txt").unwrap();

    let (mut sum_of_invalids, mut nearby_tickets) = (0, false);

    for line in read.split("\r\n") {
        if !nearby_tickets {
            nearby_tickets |= line.contains("nearby");
        }

        if RE.is_match(line) {
            for gr in RE.captures_iter(line) {
                let l: i32 = gr.get(1).unwrap().as_str().parse().unwrap();
                let r = gr.get(2).unwrap().as_str().parse().unwrap();
                (l..=r).for_each(|n| {
                    all_valid_nums.insert(n);
                })
            }
        } else if nearby_tickets {
            line.split(',').for_each(|n| {
                if let Ok(s) = n.parse::<i32>() {
                    if all_valid_nums.get(&s).is_none() {
                        // println!("{}", s);
                        sum_of_invalids += s;
                    }
                }
            })
        }
    }
    println!("{}", sum_of_invalids);
}
