use std::{fs, ops::RangeBounds};

fn main() {
    let grid = fs::read_to_string("./input.txt").unwrap();

    let [mut souths, mut easts] = [Vec::new(), Vec::new()];

    let [mut x_lim, mut y_lim] = [0, 0];

    grid.split("\r\n").enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            if c == 'v' {
                souths.push((i, j));
            } else if c == '>' {
                easts.push((i, j));
            }
            x_lim = j;
        });
        y_lim = i;
    });

    println!(
        "{}",
        simulate_movement([easts, souths], (y_lim + 1, x_lim + 1))
    );
}
fn simulate_movement(
    mut east_south: [Vec<(usize, usize)>; 2],
    (y_lim, x_lim): (usize, usize),
) -> i64 {
    let mut steps = 0;

    loop {
        let mut veco = [Vec::new(), Vec::new()];
        let mut mv = false;
        for (i, dirs) in east_south.iter().enumerate() {
            for (y, x) in dirs.iter().rev() {
                let [ny, nx] = [
                    (y + (i == 1) as usize) % y_lim,
                    (x + (i == 0) as usize) % x_lim,
                ];

                if (i == 0 && !east_south[0].contains(&(ny, nx)) ||(i != 0 && !veco[0].contains(&(ny, nx))))
                    && !east_south[1].contains(&(ny, nx))
                    
                {
                    mv = true;
                    veco[i].push((ny, nx));
                } else {
                    veco[i].push((*y, *x));
                }
            }
        }
        steps += 1;

        if !mv {
            return steps;
        }
        east_south = veco;
    }
}
