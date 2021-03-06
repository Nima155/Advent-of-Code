use std::{collections::HashMap, fs};

macro_rules! hashmap {
    ($($k: expr => $v: expr),*) => {{
        let mut map = HashMap::new();
        $(map.insert($k, $v);)+
        map
    }}
}

fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();
    let lines = lines
        .split("\r\n")
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let scores = hashmap!( '{' => 3, '<' => 4, '[' => 2, '(' => 1);
    let opposites = hashmap!('}' => '{', ']' => '[', ')' => '(', '>' => '<');

    let mut score_list = Vec::new();
    for l in &lines {
        let temp = calculate_score(&scores, &opposites, l);
        if temp != 0 {
            score_list.push(temp);
        }
    }
    score_list.sort_unstable();

    println!("{}", score_list[score_list.len() / 2]);
}

fn calculate_score(
    scores: &HashMap<char, i128>,
    opposites: &HashMap<char, char>,
    line: &[char],
) -> i128 {
    let mut stack = Vec::new();

    for brack in line {
        match brack {
            '[' | '{' | '<' | '(' => {
                stack.push(*brack);
            }
            _ => {
                if stack.is_empty() || opposites[brack] != *stack.last().unwrap() {
                    return 0;
                }
                stack.pop();
            }
        }
    }


    stack.iter().rev().fold(0, |acc, cur| acc * 5 + scores[cur])
}
