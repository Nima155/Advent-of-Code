use std::fs;



// Adapted from https://www.youtube.com/watch?v=YKpViLcTp64
fn main() {
    let instructions = fs::read_to_string("../input.txt").unwrap();
    let instructions = instructions
        .split("\r\n")
        .map(|l| {
            let mut split_up = l.split(' ');
            let [action, steps] = [split_up.next().unwrap(), split_up.next().unwrap()];
            let veco = steps
                .split(',')
                .map(|ins| {
                    let one_two = ins.split("..").collect::<Vec<_>>();
                    (
                        one_two[0][2..].parse::<i128>().unwrap(),
                        one_two[1].parse::<i128>().unwrap(),
                    )
                })
                .collect::<Vec<_>>();
            (action, veco)
        })
        .collect::<Vec<_>>();
    // x + HEIGHT* (y + WIDTH* z)

    let [mut xs, mut ys, mut zs] = [vec![], vec![], vec![]];
    for (_, v) in &instructions {
        xs.push(v[0].0);
        xs.push(v[0].1 + 1);
        ys.push(v[1].0);
        ys.push(v[1].1 + 1);
        zs.push(v[2].0);
        zs.push(v[2].1 + 1);
    }
    xs.sort_unstable();
    ys.sort_unstable();
    zs.sort_unstable();
    
    work_with_compressed([xs, ys, zs], &instructions);
}


fn get_index<T: Eq + PartialEq>(slice: &[T], value: T) -> usize {
    slice.iter().position(|f| *f == value).unwrap()
}

fn work_with_compressed(
    compressed_cords: [Vec<i128>; 3],
    instructions: &[(&str, Vec<(i128, i128)>)],
) {
    let mut three_d_space =
        vec![
            vec![vec![false; compressed_cords[2].len()]; compressed_cords[1].len()];
            compressed_cords[0].len()
        ];
    for (act, inst) in instructions {
        let (x_min, x_max) = (
            get_index(&compressed_cords[0], inst[0].0),
            get_index(&compressed_cords[0], inst[0].1 + 1),
        );
        let (y_min, y_max) = (
            get_index(&compressed_cords[1], inst[1].0),
            get_index(&compressed_cords[1], inst[1].1 + 1),
        );
        let (z_min, z_max) = (
            get_index(&compressed_cords[2], inst[2].0),
            get_index(&compressed_cords[2], inst[2].1 + 1),
        );

        for x in x_min..x_max {
            for y in y_min..y_max {
                for z in z_min..z_max {
                    three_d_space[x][y][z] = *act == "on";
                }
            }
        }
    }

    let mut ans = 0;
    for (x, row) in three_d_space.iter().enumerate() {
        for (y, row_1) in row.iter().enumerate() {
            for (z, finals) in row_1.iter().enumerate() {
                if z + 1 < row_1.len() && x + 1 < three_d_space.len() && y + 1 < row.len() {
                    ans += *finals as i128
                        * (compressed_cords[0][x + 1] - compressed_cords[0][x])
                            * (compressed_cords[1][y + 1] - compressed_cords[1][y])
                            * (compressed_cords[2][z + 1] - compressed_cords[2][z]);
                        
                }
            }
        }
    }
    println!("{}", ans);
}
