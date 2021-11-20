use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();
    let lines = lines.split("\r\n").collect::<Vec<_>>();

    println!("{}", lines.iter().filter(|l| supports_tls(l)).count())
}

const WINDOW_SIZE: usize = 4;
fn supports_tls(line: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r###"\[\w+?\]|[a-z]+"###).unwrap();
    }

    let matches = RE.find_iter(line).map(|l| l.as_str()).collect::<Vec<_>>();

    let (mut mt_out, mut mt_in) = (false, false);

    for mtch in matches {
        let vecs = mtch.chars().collect::<Vec<_>>();
        if mtch.starts_with("[") {
            mt_in |= vecs[1..vecs.len() - 1].windows(WINDOW_SIZE).any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
        } else {
            
            mt_out |= vecs.windows(WINDOW_SIZE).any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
        }
    }
    mt_out && !mt_in
}