use itertools::Itertools;
fn main() {
    // 353096-843212
    let mut ans = 0;
    for i in 353096..=843212 {
        ans += valid_password(i) as u32;
    }
    println!("{}", ans);
}

// It is a six-digit number.
// The value is within the range given in your puzzle input.
// Two adjacent digits are the same (like 22 in 122345).
// Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
fn valid_password(num: i32) -> bool {
    let stringified =  num.to_string();

    let (mut increasing, mut doubles) = (true, false);

    let to_char_vec = stringified.chars().collect::<Vec<char>>();
    let mut prev: char = '0'; 

    for (c, group) in to_char_vec.iter().group_by(|f| **f).into_iter() {
        doubles |= group.count() == 2;
        increasing &= c > prev;
        prev = c;
    }
 
    
    increasing && doubles
}