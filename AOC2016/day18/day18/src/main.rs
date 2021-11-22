const INPUT: &str = "...^^^^^..^...^...^^^^^^...^.^^^.^.^.^^.^^^.....^.^^^...^^^^^^.....^.^^...^^^^^...^.^^^.^^......^^^^";
const TARGET_SIZE: usize = 400000; // swap to another number for part 1

fn main() {
    let mut last_row = INPUT.chars().collect::<Vec<_>>();
    let (mut count, mut safe_count) = (0, 0);
    while count < TARGET_SIZE {
        let mut new_row = Vec::with_capacity(last_row.len());
        for i in 0..INPUT.len() {
            let lcr = [
                if i == 0 { true } else { last_row[i - 1] == '.' },
                last_row[i] == '.',
                if i == (INPUT.len() - 1) {
                    true
                } else {
                    last_row[i + 1] == '.'
                },
            ];
            new_row.push(if is_trap(lcr) { '^' } else { '.' });
        }
        safe_count += last_row.iter().filter(|c| **c == '.').count();
        last_row = new_row;
        count += 1;
    }
    println!("{}", safe_count);
}

fn is_trap(conds: [bool; 3]) -> bool {
    let [left, center, right] = conds;

    (!center && ((!right && left) || (!left && right)))
        || (center && ((!left && right) || (!right && left)))
}
