fn infix_to_postfix(expr: &[char]) -> Vec<char> {
    let (mut ops, mut post): (Vec<char>, Vec<char>)  = (vec![], vec![]);
    
    for i in expr {
        match *i {
            '+' | '*' | '(' => {
                while !ops.is_empty() && ops[ops.len() - 1] == '+' {
                    if *i == '*' {
                        post.push(ops.pop().unwrap());
                    } else { break; }
                } 
                ops.push(*i)
            }
            '0'..='9' => {
                post.push(*i)
            }
            ')' => {
                while ops[ops.len() - 1] != '(' {
                    post.push(ops.pop().unwrap());
                }
                ops.pop();
            }
            _ => {}
        }
    }
    while !ops.is_empty() {
        post.push(ops.pop().unwrap());
    }
    post
}

pub fn iterative_evaluator(expr: &[char]) -> u64 {
    let to_post = infix_to_postfix(expr);
    
    let mut nums: Vec<u64> = vec![];
    
    for i in to_post.iter() {
        
        match *i {
            dig @ '0'..='9' => {
                nums.push(dig.to_digit(10).unwrap() as u64);
            },
            op  => {
                let res;
                if let '+' = op {
                    res = nums.pop().unwrap() + nums.pop().unwrap();
                } else {
                    res = nums.pop().unwrap() * nums.pop().unwrap();
                }
                nums.push(res);
            } 
        }
    }
    
    nums.pop().unwrap()
}