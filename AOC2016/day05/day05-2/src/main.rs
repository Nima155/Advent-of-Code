const INPUT: &str = "uqwqemis";
fn main() {
    let mut password = [' '; 8];
    let mut i = 0;
    let mut visited_indices = [0; 8];

    while visited_indices.iter().any(|n| *n == 0) {
        let test = format!("{}{}", INPUT, i);
        let md5_rep = format!("{:x}", md5::compute(test));

        let indx = md5_rep.chars().nth(5).unwrap();
        let indx = if indx.is_digit(10) { indx.to_digit(10).unwrap() } else { 9 };
        if md5_rep.starts_with("00000") && indx < 8 && visited_indices[indx as usize] == 0 {
            password[indx as usize] = md5_rep.chars().nth(6).unwrap();
            visited_indices[indx as usize] = 1;
        }
        i += 1;
    }
    println!("{}", password.iter().collect::<String>());
}
