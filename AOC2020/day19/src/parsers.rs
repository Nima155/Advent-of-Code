use std::collections::HashMap;
pub enum Rule<'a> {
    IndirectRule(u64, Vec<Vec<u64>>),
    DirectRule(u64, &'a str),
}

fn parse_rule<'a>(line: &'a str) -> Rule<'a> {
    let twos = line.split(": ").collect::<Vec<&str>>();

    let key = twos[0].parse::<u64>().unwrap();
    
    match twos[1].contains('"') {
        true => Rule::DirectRule(key, &twos[1][twos[1].find('"').unwrap()+1..twos[1].len() - 1]),

        _ => Rule::IndirectRule(
            key,
            twos[1]
                .split(" | ")
                .map(|e| {
                    e.split(' ')
                        .map(|i| i.parse::<u64>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        ),
    }
}

pub fn parse_overall<'a>(lines: &[&'a str]) -> (HashMap<u64, Rule<'a>>, Vec<&'a str>) {
    let mut rules: HashMap<u64, Rule> = HashMap::new();
    let mut messages: Vec<&'a str> = Vec::with_capacity(200);
    for l in lines {
        if !l.is_empty() {
            if let true = l.chars().next().unwrap().is_digit(10) {
                let parsed_rule = parse_rule(l);

                match parsed_rule {
                    Rule::DirectRule(rule_num, _) => {
                        rules.insert(rule_num, parsed_rule);
                    }
                    Rule::IndirectRule(rule_num, ref _other_rules) => {
                        rules.insert(rule_num, parsed_rule);
                    }
                }
            } else {
                messages.push(*l);
            }
        }
    }
    (rules, messages)
}

pub fn check_message(rules: &HashMap<u64, Rule>, message: &str, cur_rule: &Rule, mut msg_indx: usize) -> (bool, usize) {
    let mut mt = true;
    
    
    

    match cur_rule {
        &Rule::DirectRule(_rule_num, msg) => {
            
            mt &= msg == &message[msg_indx..msg.len() + msg_indx];
            msg_indx += msg.len();
            
        }
        &Rule::IndirectRule(_rule_num, ref others) => {
            // let mut outer_indx = msg_indx;
            mt &= others.iter().any(|e| {
                let mut inner_indx = msg_indx;
                let ans = e.iter()
                    .all(|c| {
                        let (stat, i) = check_message(rules, message, rules.get(c).unwrap(), inner_indx);
                        inner_indx = i;
                        stat
                    });
                if ans { msg_indx = inner_indx }
                ans
            });
            
            
        }
    }
 
    
    (mt, msg_indx)
}

pub fn main_logic<'a>(lines: &[&'a str]) -> u64 {
    let (rules, messages) = parse_overall(lines);

    let mut ans = 0;

    for l in messages {
        let (status, _i) = check_message(&rules, l, rules.get(&0).unwrap(), 0);
        
        ans += (status && _i == l.len()) as u64;
        
    }

    ans
    // complete matches of rule 0
}
