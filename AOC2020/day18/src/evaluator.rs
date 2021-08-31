use std::collections::VecDeque;

fn evaluate_two(ops: &mut VecDeque<char>, nums: &mut VecDeque<u64>) {
    if nums.len() < 2 || ops.is_empty() {
        return;
    }

    let (one, two) = (nums.pop_front().unwrap(), nums.pop_front().unwrap());
    match ops.front().unwrap() {
        '+' => {
            nums.push_back(one + two);
        }
        '*' => {
            nums.push_back(one * two);
        }
        _ => {}
    }
    ops.pop_front();
}

pub fn recursive_evaluator(expression: &[char], mut indx: usize) -> (u64, usize) {
    let exp_len = expression.len();

    if indx == exp_len {
        return (0, indx);
    }

    let (mut ops, mut nums) = (VecDeque::new(), VecDeque::new());

    let mut cur_num = None;

    while indx < exp_len {
        let c = expression[indx];
        // println!("{:?} {:?} {} {} {}", nums, ops, indx, expression.len(), c);
        match ("+*".contains(c), c == '(', c.is_ascii_digit(), c == ')') {
            (true, _, _, _) => {
                if let Some(n) = cur_num {
                    nums.push_back(n);
                    cur_num = None;
                }
                ops.push_back(c)
            }
            (_, true, _, _) => {
                let (expr_value, i) = recursive_evaluator(expression, indx + 1);
                indx = i;

                nums.push_back(expr_value);
            }
            (_, _, true, _) => {
                if let Some(mut n) = cur_num {
                    n *= 10;
                    n += c.to_digit(10).unwrap() as u64;
                    cur_num = Some(n);
                } else {
                    cur_num = Some(c.to_digit(10).unwrap() as u64)
                }
            }
            (_, _, _, true) => {
                if let Some(n) = cur_num {
                    nums.push_back(n);
                }
                evaluate_two(&mut ops, &mut nums);
                return (*nums.back().unwrap(), indx);
            }
            _ => {}
        }
        evaluate_two(&mut ops, &mut nums);
        indx += 1;
    }
    if let Some(n) = cur_num {
        nums.push_back(n);
    }
    evaluate_two(&mut ops, &mut nums);
    (*nums.back().unwrap_or(&0), indx)
}