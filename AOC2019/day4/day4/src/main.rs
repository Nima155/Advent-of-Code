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

    let (mut repeats, mut increasing) = (false, true);

    for (bef, now) in stringified.chars().zip(stringified[1..].chars()) {
        repeats |= bef == now;
        increasing &= now >= bef;
    }

    repeats && increasing
}