use std::fs;
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();

    let vectorized_lines = lines.split("\r\n").collect::<Vec<_>>();

    'a: for l in vectorized_lines.iter() {
        for l_1 in vectorized_lines.iter() {
            if l != l_1 {
                let not_matching = l.chars().zip(l_1.chars()).filter(|(a, b)| *a != *b).count();
                if not_matching == 1 {
                    println!(
                        "{:?}",
                        l.chars()
                            .zip(l_1.chars())
                            .filter(|(a, b)| *a == *b)
                            .map(|a| a.0)
                            .collect::<String>()
                    );
                    break 'a;
                }
            }
        }
    }
}
