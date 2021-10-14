use std::fs;

use regex::Regex;
fn main() {
    let mut sequence = fs::read_to_string("../input.txt").unwrap();
    
    let mut pat = ('a'..='z').zip('A'..='Z').map(|(a, b)| format!("{}{}|", a, b)).collect::<String>();
    pat.push_str(&('A'..='Z').zip('a'..='z').map(|(a, b)| format!("{}{}{}", a, b, if a != 'Z' {'|'} else {' '}  )).collect::<String>());
    let re = Regex::new(pat.trim()).unwrap();
    
    
    loop {
        let after = re.replace_all(&sequence, "").to_string();
        if after == sequence {
            println!("{}", after.len());
            break;
        }
        sequence = after;
    }
}
