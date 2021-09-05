use std::collections::HashMap;
#[derive(Debug)]
pub enum Rule<'a> {
    IndirectRule(u64, Vec<Vec<u64>>),
    DirectRule(u64, &'a str),
}

fn parse_rule<'a>(line: &'a str) -> Rule<'a> {
    let twos = line.split(": ").collect::<Vec<&str>>();

    let key = twos[0].parse::<u64>().unwrap();

    match twos[1].contains('"') {
        true => Rule::DirectRule(
            key,
            &twos[1][twos[1].find('"').unwrap() + 1..twos[1].len() - 1],
        ),

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

pub fn check_message(
    rules: &HashMap<u64, Rule>,
    message: &str,
    cur_rule: &Rule,
    mut msg_indx: usize,
) -> (bool, usize) {
    let mut mt = true;
    
    if msg_indx >= message.len() {
        return (false, msg_indx);
    }
    
    match cur_rule {
        &Rule::DirectRule(_rule_num, msg) => {
            
            mt &= msg == &message[msg_indx..msg.len() + msg_indx];
            
            if mt {
                msg_indx += msg.len();
            }
        }
        &Rule::IndirectRule(rule_num, ref others) => match rule_num {
            8 | 11 => {
                let (mut matches_42, mut prev, mut prev_success): (usize, i32, bool) = (0, -1, false);
                loop {
                    if prev != -1 && msg_indx == (prev as usize) {
                        return (prev_success, msg_indx);
                    }
                    let (status, i) = check_message(rules, message, rules.get(&42).unwrap(), msg_indx);

                    prev = msg_indx as i32;
                    prev_success = status;
                    
                    if status && i == message.len() {
                        println!("true");
                        return (true, i);
                    }

                    if rule_num == 11 || matches_42 >= 1 && rule_num != 11 {
                        let (stat1, mut ij) = check_message(rules, message, rules.get(&31).unwrap(), i);
                        if stat1 {
                            if ij == message.len() {
                                println!("true");
                                return (true, ij);
                            }
                            let mut matches_31 = 1;
                            loop {
                                let (stat2, iji) = check_message(rules, message, rules.get(&31).unwrap(), ij);
                                if !stat2 {
                                    return (false, iji);
                                } else if iji == message.len() && matches_42 - 1 == matches_31 {
                                    println!("true");
                                    return (true, iji);
                                } else if iji == message.len() && matches_42 - 1 != matches_31 {
                                    return (false, iji);
                                }
                                matches_31 += 1;
                                ij = iji;
                            }
                            
                        }
                    }
                    
                    matches_42 += 1;
                    msg_indx = i;
                }
            }
            _ => {
                mt &= others.iter().any(|e| {
                    let mut inner_indx = msg_indx;
                    let ans = e.iter().all(|c| {
                        let (stat, i) =
                            check_message(rules, message, rules.get(c).unwrap(), inner_indx);
                        inner_indx = i;
                        stat
                    });
                    if ans {
                        msg_indx = inner_indx
                    }
                    ans
                });
            }
        },
    }
    // println!("{} {}", mt, msg_indx == message.len());
    (mt, msg_indx)
}

// cache.insert((message, msg_indx), (mt, msg_indx));

// println!("{:?}", (vec_of_ans.iter().any(|e| e.0 && e.1 == message.len()), message.len()));

pub fn main_logic<'a>(lines: &[&'a str]) -> u64 {
    let (mut rules, messages) = parse_overall(lines);

    rules.insert(
        11,
        Rule::IndirectRule(11, vec![[42, 31].to_vec(), [42, 11, 31].to_vec()]),
    );
    rules.insert(
        8,
        Rule::IndirectRule(8, vec![[42].to_vec(), [42, 8].to_vec()]),
    );

    let mut ans = 0;

    for l in messages {
        let (status, _i) = check_message(&rules, l, rules.get(&0).unwrap(), 0);
        if status {
            println!("{} {}", l, _i == l.len());
        }

        ans += (status) as u64;
    }

    ans
    // complete matches_42 of rule 0
}
