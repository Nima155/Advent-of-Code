use std::{fs, collections::HashMap};

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
   
 
    let scores = hashmap!( '}' => 1197, '>' => 25137, ']' => 57, ')' => 3);
    let opposites = hashmap!('}' => '{', ']' => '[', ')' => '(', '>' => '<');
    let mut ans = 0;
    for l in &lines {
        ans += calculate_score(&scores, &opposites, l);
    }
    println!("{}", ans);


}

fn calculate_score(scores: &HashMap<char, i32>, opposites: &HashMap<char, char>, line: &[char]) -> i64 {
    let mut stack = Vec::new();

    for brack in line {
        match brack {
            '[' | '{' | '<' | '(' => {
                stack.push(*brack);
            }
            _ => {
                if stack.is_empty() || opposites[brack] != *stack.last().unwrap()  {
                    return scores[brack] as i64;
                }
                stack.pop();
            }
        }
    }
    0
}
