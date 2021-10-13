use std::fs;
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();

    let mut parsed = Vec::new();
    for l in lines.split("\r\n") {
        let line_to_vec = l.split('@').collect::<Vec<_>>();
        let dimensions = line_to_vec[1].split(':').collect::<Vec<_>>();
        let (starting, dims) = (
            dimensions[0].trim().split(',').collect::<Vec<_>>(),
            dimensions[1].trim().split('x').collect::<Vec<_>>(),
        );

        parsed.push((
            line_to_vec[0],
            (
                starting[0].parse::<usize>().unwrap(),
                starting[1].parse::<usize>().unwrap(),
            ),
            (
                dims[0].parse::<usize>().unwrap(),
                dims[1].parse::<usize>().unwrap(),
            ),
        ));
    }

    // print!("{} hi {}", 1, 2);
    brute_force_and_find_common(&parsed);
}

fn brute_force_and_find_common(parsed: &[(&str, (usize, usize), (usize, usize))]) {
    let mut all_points = vec![vec![0; 1000]; 1000];

    for (_, (s_y, s_x), (c_y, c_x)) in parsed {
        for i in 0..*c_y {
            for j in 0..*c_x {
                all_points[i + s_y][j + s_x] += 1;
            }
        }
    }

    all_points.iter().enumerate().for_each(|(i, r)| {
        // (j, e)
        for (j, e) in r.iter().enumerate() {
            if *e == 1 {
                if let Some((id, st, lim)) = parsed.iter().find(|(_, (y, x), _)| (i, j) == (*y, *x))
                {
                    if inspect_square(*st, *lim, &all_points) {
                        println!("{}", &id[1..]);
                    }
                }
            }
        }
    });
}

fn inspect_square(
    (s_y, s_x): (usize, usize),
    (e_y, e_x): (usize, usize),
    all_points: &[Vec<i32>],
) -> bool {
    for i in 0..e_y {
        for j in 0..e_x {
            if all_points[s_y + i][s_x + j] != 1 {
                return false;
            }
        }
    }

    true
}
