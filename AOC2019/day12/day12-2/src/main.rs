use num::integer::lcm;
use std::fs;
fn main() {
    let read = fs::read_to_string("../input.txt").unwrap();

    let mut parsed = read
        .split("\r\n")
        .map(|f| {
            f.split(", ")
                .map(|c| {
                    c.split('=')
                        .nth(1)
                        .unwrap()
                        .trim_end_matches('>')
                        .parse::<i64>()
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let last_veloc = simulate_the_process(&mut parsed);

    println!(
        "{:?}",
        last_veloc.iter().fold(1, |acc, prev| lcm(acc, *prev))
    );
}

fn apply_gravity_to_velocity(dimension: &mut [i64], velocities: &mut [i64; 4]) -> usize {
    let mut steps = 0;
    let clone = dimension.to_vec();
    // println!("{:?}", clone);
    while steps == 0 || !velocities.iter().all(|c| *c == 0) || clone != dimension {
        for (i, e) in dimension.iter().enumerate() {
            for (j, num) in dimension.iter().enumerate() {
                if i != j {
                    velocities[i] += if e < num {
                        1
                    } else {
                        [-1, 0][(e == num) as usize]
                    };
                }
            }
        }

        dimension
            .iter_mut()
            .enumerate()
            .for_each(|(j, c)| *c += velocities[j]);
        steps += 1;
    }

    steps
}

fn simulate_the_process(moons: &mut [Vec<i64>]) -> [usize; 3] {
    let (mut j, mut ans) = (0, [0; 3]);

    while j < 3 {
        let mut velocities = [0; 4];

        let stepado = apply_gravity_to_velocity(
            &mut moons.iter().map(|f| f[j]).collect::<Vec<_>>(),
            &mut velocities,
        );

        ans[j] = stepado;
        j += 1;
    }
    // println!("{:?}", moons);
    ans
}
