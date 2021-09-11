use std::collections::{HashMap, VecDeque};
#[derive(Clone)]
pub enum Rule<'a> {
    IndirectRule(u64, Vec<Vec<u64>>),
    DirectRule(u64, &'a str),
}

fn parse_rule(line:  &str) -> Rule {
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
// To make the matcher work with part 2 the trick was recognizing the need to break out of the function once
//  the end of the string is reached while also checking that the last step completed was a
//  recursion and treat that as a success. Otherwise, if the last step wasn't recursive it means the
// end of the string was reached in the middle of a rule check and fails.

pub fn check_message(
    rules: &HashMap<u64, Rule>,
    message: &str,
    rules_to_match: &mut VecDeque<Rule>,
) -> bool {
    // println!("{}", message);
    if rules_to_match.is_empty() || message.is_empty() {
        return rules_to_match.is_empty() && message.is_empty();
    }

    match &rules_to_match[0] {
        Rule::DirectRule(_c, car) => {
            if car.chars().next().unwrap() == message.chars().next().unwrap() {
                let mut rulesy = rules_to_match.clone();
                rulesy.pop_front();

                return check_message(rules, &message[1..], &mut rulesy);
            }
        }

        Rule::IndirectRule(_o, ref rest) => {
            for r in rest {
                let mut clonesy = rules_to_match.clone();
                clonesy.pop_front();

                for e in r.iter().rev() {
                    clonesy.push_front(rules.get(e).unwrap().clone());
                }

                let status = check_message(rules, message, &mut clonesy);

                if status {
                    return true;
                }
            }
        }
    }

    false
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
        let status = check_message(
            &rules,
            l,
            &mut VecDeque::from([rules.get(&0).unwrap().clone()]),
        );

        ans += status as u64;
    }
    println!("{}", ans);
    ans
    // complete matches_42 of rule 0
}
