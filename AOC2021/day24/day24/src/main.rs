fn main() {
    let mut registers = [0; 26];

    println!("{:?}", recursive_builder(&mut registers, 0, 0));
}

const DISTINCT_INSTRUCTIONS: [[i64; 3]; 14] = [
    [1, 13, 10],
    [1, 11, 16],
    [1, 11, 0],
    [1, 10, 13],
    [26, -14, 7], // 23 == 9 is doable
    [26, -4, 11], //
    [1, 11, 11],
    [26, -3, 10],
    [1, 12, 16],
    [26, -12, 8],
    [1, 13, 15],
    [26, -12, 2],
    [26, -15, 5],
    [26, -12, 10],
];

fn recursive_builder(registers: &mut [i64], no_dig: usize, mut build: i64) -> (bool, i64) {
    if no_dig == 14 {
        if registers[25] == 0 {
            return (true, build);
        }
        return (false, build);
    }

    let [z, x, y] = [25, 23, 24];

    for dig in (1..10).rev() { // change to 1..10 for answer to part 2 
        let clone_bef = registers.to_owned();
        registers[x] = registers[z] % 26;
        registers[z] /= DISTINCT_INSTRUCTIONS[no_dig][0];
        registers[x] += DISTINCT_INSTRUCTIONS[no_dig][1];

        if DISTINCT_INSTRUCTIONS[no_dig][1] < 0 && registers[x] != dig {
            reset_reg(registers, &clone_bef);
            continue;
        }

        registers[x] = (registers[x] != dig) as i64;
        registers[y] = 25 * registers[x] + 1;
        registers[z] *= registers[y];
        registers[y] = (dig + DISTINCT_INSTRUCTIONS[no_dig][2]) * registers[x];
        registers[z] += registers[y];
        let build_bef = build;
        build *= 10;
        build += dig;

        let (found, n) = recursive_builder(registers, no_dig + 1, build);

        if found {
            return (found, n);
        }

        build = build_bef;

        reset_reg(registers, &clone_bef);
    }
    (false, build)
}

fn reset_reg(registers: &mut [i64], prev_reg: &[i64]) {
    for (i, n) in prev_reg.iter().enumerate() {
        registers[i] = *n;
    }
}
