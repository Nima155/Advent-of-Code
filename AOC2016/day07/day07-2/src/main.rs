use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();
    let lines = lines.split("\r\n").collect::<Vec<_>>();

    println!("{}", lines.iter().filter(|l| supports_ssl(l)).count())
}

const WINDOW_SIZE: usize = 3;
fn supports_ssl(line: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r###"\[\w+?\]|[a-z]+"###).unwrap();
    }

    let matches = RE.find_iter(line).map(|l| l.as_str()).collect::<Vec<_>>();

    let (mut mt_outs, mut mt_ins) = (vec![], vec![]);

    for mtch in matches {
        let vecs = mtch.chars().collect::<Vec<_>>();
        if mtch.starts_with('[') {
            mt_ins.extend(
                vecs[1..vecs.len() - 1]
                    .windows(WINDOW_SIZE)
                    .filter(|w| w[0] == w[2] && w[0] != w[1])
                    .map(|e| e.to_owned().iter().collect::<String>()),
            );
        } else {
            mt_outs.extend(
                vecs.windows(WINDOW_SIZE)
                    .filter(|w| w[0] == w[2] && w[0] != w[1])
                    .map(|e| e.to_owned().iter().collect::<String>()),
            );
        }
    }
    
    mt_outs.iter().any(|e| {
        mt_ins.contains(&format!(
            "{}{}{}",
            e.chars().nth(1).unwrap(),
            e.chars().next().unwrap(),
            e.chars().nth(1).unwrap()
        ))
    })
}
