use std::fs;

use regex::Regex;
fn main() {
    let sequence = fs::read_to_string("../input.txt").unwrap();
    
    let mut pat = ('a'..='z').zip('A'..='Z').map(|(a, b)| format!("{}{}|", a, b)).collect::<String>();
    pat.push_str(&('A'..='Z').zip('a'..='z').map(|(a, b)| format!("{}{}{}", a, b, if a != 'Z' {'|'} else {' '}  )).collect::<String>());
    let re = Regex::new(pat.trim()).unwrap();
    
    let mut ans = [usize::MAX; 26];
    ('a'..='z').for_each(|c| {
        let seq_clone = sequence.clone();
        let seq_clone = seq_clone.replace(c, "");
        let mut seq_clone = seq_clone.replace(c.to_ascii_uppercase(), "");

        loop {
            let after = re.replace_all(&seq_clone, "").to_string();
            if after == seq_clone {
                ans[(c.to_ascii_lowercase() as u8 - b'a') as usize] = after.len();
                break;
            }
            seq_clone = after;
        }
    });
    println!("{:?}", ans.iter().min().unwrap());
    
}
