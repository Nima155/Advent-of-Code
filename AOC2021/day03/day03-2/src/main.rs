use std::fs;
// 499212 too low!
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();
    let lines = lines
        .split("\r\n")
        .map(|e| e.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (mut oxy, mut scrubber) = (lines.clone(), lines.clone());

    for j in 0..lines[0].len() {
        reducer(&mut oxy, j, '1');
        reducer(&mut scrubber, j, '0');
    }
    // println!("{:?} {:?}", oxy, scrubber);
    println!(
        "{}",
        u64::from_str_radix(&oxy[0].iter().collect::<String>(), 2).unwrap()
            * u64::from_str_radix(&scrubber[0].iter().collect::<String>(), 2).unwrap()
    )
}

fn reducer(remaining_binaries: &mut Vec<Vec<char>>, column_number: usize, first_car: char) {
    let column = remaining_binaries
        .iter()
        .map(|e| e[column_number])
        .collect::<Vec<_>>();
    let count = column.iter().filter(|e| **e == '1').count();
    let oppo = if first_car == '0' { '1' } else { '0' };

    if column.len() > 1 {
        let cond = count > column.len() / 2 || count == column.len() / 2 && column.len() % 2 == 0;

        *remaining_binaries = remaining_binaries
            .clone()
            .into_iter()
            .filter(|e| {
                if cond {
                    e[column_number] == first_car
                } else {
                    e[column_number] == oppo
                }
            })
            .collect::<Vec<_>>();
    }
}
