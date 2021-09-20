use std::fs;
const LOOP_COUNT: u16 = 1000;
fn main() {
    let read = fs::read_to_string("../input.txt").unwrap();

    let mut parsed = read
        .split("\r\n")
        .map(|f| {
            f.split(", ")
                .map(|c| {
                    c.split('=').nth(1).unwrap().trim_end_matches('>').parse::<i64>().unwrap()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let last_veloc = simulate_the_process(&mut parsed);
    println!("{:?} {:?}", last_veloc, parsed);
    println!("{}", 
    parsed.iter().enumerate().fold(0, |acc, prev| 
        acc + ( prev.1.iter().map(|e| e.abs()).sum::<i64>() * last_veloc[prev.0].iter().map(|f| f.abs()).sum::<i64>() )));

}



fn apply_gravity_to_velocity(moons: &[Vec<i64>], veclocities: &mut [[i64;3];4])  {
    
    
    for (i, e) in moons.iter().enumerate() {
        for (j, e2) in moons.iter().enumerate() {
            
            if j != i {
                veclocities[i][0] += if e[0] < e2[0] { 1 } else {  [-1, 0][(e[0] == e2[0]) as usize]  };
                veclocities[i][1] += if e[1] < e2[1] { 1 } else {  [-1, 0][(e[1] == e2[1]) as usize]  };
                veclocities[i][2] += if e[2] < e2[2] { 1 } else {  [-1, 0][(e[2] == e2[2]) as usize]  };
            }
        }
    }

    
}


fn simulate_the_process(moons: &mut [Vec<i64>]) -> [[i64; 3]; 4] {
    let mut j = 0;
    let mut velocities = [[0; 3]; 4];
    while j < LOOP_COUNT {
        apply_gravity_to_velocity(moons, &mut velocities);

        for (i, e) in moons.iter_mut().enumerate() {
            for (x, n) in e.iter_mut().enumerate() {
                *n += velocities[i][x];
            }
        }

        j += 1;

    }
    velocities
}