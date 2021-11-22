use std::collections::{HashMap, VecDeque};

use fancy_regex::Regex;


const SALT: &str = "yjdafjpo";
fn main() {
    lazy_static::lazy_static!{
        static ref RE: Regex = Regex::new(r###"([a-z0-9])\1{2}"###).unwrap();
        static ref ALPHA_FINDER: Regex = Regex::new(r###"([a-z0-9])\1{4}"###).unwrap();
    };

    let mut queue: HashMap<String, VecDeque<i32>> = HashMap::new();
    let mut actual_key_indx = 0;
    
    'a: for i in 0.. {

        let md5_rep = format!("{:0x}", md5::compute(format!("{}{}", SALT, i)));

        for mt in ALPHA_FINDER.find_iter(&md5_rep) {
            if let Ok(fives) = mt {
                let to_str = fives.as_str();

                if queue.contains_key(&to_str[..3]) {
                    let entry = queue.get_mut(&to_str[..3]).unwrap();

                    while (i as i32 - entry.front().unwrap()) > 1000 {
                        entry.pop_front();
                    }

                    while !entry.is_empty() {
                        let fr = entry.pop_front().unwrap();
                        actual_key_indx += 1;
                        if actual_key_indx == 64 {
                            println!("{}", fr);
                            break 'a;
                        }
                    }

                    
                }
            }
        }

        let mt = RE.find(&md5_rep);
        if let Ok(matchs) = mt {
            if let Some(m) = matchs {
                let entry = queue.entry(m.as_str().to_owned()).or_default();
                entry.push_back(i);
            }
        }
        
    }
}
