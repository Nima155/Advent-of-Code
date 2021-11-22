use std::collections::{HashMap, VecDeque};

use fancy_regex::Regex;

const SALT: &str = "yjdafjpo";
fn main() {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r###"([a-z0-9])\1{2}"###).unwrap();
        static ref ALPHA_FINDER: Regex = Regex::new(r###"([a-z0-9])\1{4}"###).unwrap();
    };

    let mut queue: VecDeque<(String, i32)> = VecDeque::new();
    let mut available_fives = HashMap::new();
    let mut actual_key_indx = 0;

    'a: for i in 0.. {
        let md5_rep = key_stretcher(&format!("{}{}", SALT, i), 2017); // sub 2017 for 1 for answer to part 1

        for mt in ALPHA_FINDER.find_iter(&md5_rep).flatten() {
            let to_str = mt.as_str();
            available_fives.insert(to_str[..3].to_owned(), i);
        }

        while !queue.is_empty() && (i - queue.front().unwrap().1) > 1000 {
            queue.pop_front();
        }

        if !queue.is_empty() {
            let queue_fr = queue.front().unwrap();
            if *available_fives.get(&queue_fr.0).or(Some(&-1)).unwrap() > queue_fr.1 {
                let (_, indx) = queue.pop_front().unwrap();
                if actual_key_indx == 63 {
                    println!("{}", indx);
                    break 'a;
                }
                actual_key_indx += 1;
            }
        }

        let mt = RE.find(&md5_rep);
        if let Ok(Some(m)) = mt {
            queue.push_back((m.as_str().to_owned(), i));
        }
    }
}
// 22034 too low

fn key_stretcher(key: &str, total_revo: i64) -> String {
    let mut cycles = 0;
    let mut key_string = key.to_owned();
    while cycles < total_revo {
        key_string = format!("{:0x}", md5::compute(key_string));
        cycles += 1;
    }
    key_string
}
