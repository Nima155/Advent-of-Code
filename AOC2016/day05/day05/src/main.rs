
const INPUT: &str = "uqwqemis";
fn main() {
    let mut password = String::with_capacity(8);
    let mut i = 0;

    while password.len() < 8 {
        let test = format!("{}{}", INPUT, i);
        
        let md5_rep = format!("{:x}", md5::compute(test));
        
        if md5_rep.starts_with("00000") {
            password.push(md5_rep.chars().nth(5).unwrap());
        }
        i += 1;
    }
    println!("{}", password);
}
