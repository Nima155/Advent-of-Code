use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    f64::consts::PI,
    fs,
};

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();

    // println!("{}")
    let scanners_and_rots = input
        .split("\r\n\r\n")
        .map(|data| {
            data.split("\r\n")
                .skip(1)
                .map(|coords| {
                    get_all_rotations(
                        &coords
                            .split(',')
                            .map(|n| n.parse::<i64>().unwrap())
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // println!("{:?}", scanners_and_rots);
    println!("{}", find_beacons(&scanners_and_rots));
}

fn find_beacons(scanners_and_rots: &[Vec<Vec<Vec<i64>>>]) -> i64 {
    let mut composite_scanner = vec![(vec![0, 0, 0], 0, 0)];
    let mut visited = HashSet::new();
    while visited.len() < scanners_and_rots.len() {
        let mut expando = vec![];
        

        for (point, scan_id, rot) in &composite_scanner {
            let rots = scanners_and_rots[*scan_id as usize]
                .iter()
                .map(|p| p[*rot].clone())
                .collect::<HashSet<_>>();

            for (scan_id_sec, beac) in scanners_and_rots.iter().enumerate() {
                // let scan_id_sec = scan_id_sec + 1;
                if *scan_id != scan_id_sec {
                    for p_main in &rots {
                        let [x, y, z] = [p_main[0], p_main[1], p_main[2]];
                        if !visited.contains(&scan_id_sec) {
                            'a: for j in 0..24 {
                                let of_one_rot_sec =
                                    beac.iter().map(|p| p[j].clone()).collect::<Vec<_>>();
                                for p in &of_one_rot_sec {
                                    let [xx, yy, zz] = [x - p[0], y - p[1], z - p[2]];
                                    let mut matches = Vec::new();
                                    for p_1 in &of_one_rot_sec {
                                        let veco = vec![xx + p_1[0], yy + p_1[1], zz + p_1[2]];
                                        if rots.contains(&veco) {
                                            matches.push(veco);
                                        }
                                    }

                                    if matches.len() >= 12 {
                                        let k = (
                                            vec![xx + point[0], yy + point[1], zz + point[2]],
                                            scan_id_sec,
                                            j,
                                        );
                                        if !composite_scanner.contains(&k) {
                                            expando.push(k);
                                        }

                                        visited.insert(scan_id_sec);
                                        break 'a;
                                    }
                                }
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        composite_scanner.extend(expando);
    }
    let mut all = HashSet::new();
    for (p_start, id, rot) in composite_scanner {
        for p in scanners_and_rots[id].iter().map(|d| d[rot].clone()) {
            all.insert([p_start[0] + p[0], p[1] + p_start[1], p[2] + p_start[2]]);
        }
    }
    all.len() as i64
}

const ROTATIONS: [i64; 12] = [0, 0, 0, 90, 90, 90, 180, 180, 180, 270, 270, 270];

fn get_all_rotations(point: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut ans = Vec::with_capacity(24);

    for rot in ROTATIONS.into_iter().permutations(3) {
        let [rot, rot_1, rot_2] = [rot[0], rot[1], rot[2]];
        let mut n_point = point.clone();

        let rot_cos = ((rot as f64 * PI) / 180.0).cos() as i64;
        let rot_sin = ((rot as f64 * PI) / 180.0).sin() as i64;
        let rot_1_cos = ((rot_1 as f64 * PI) / 180.0).cos() as i64;
        let rot_1_sin = ((rot_1 as f64 * PI) / 180.0).sin() as i64;
        let rot_2_cos = ((rot_2 as f64 * PI) / 180.0).cos() as i64;
        let rot_2_sin = ((rot_2 as f64 * PI) / 180.0).sin() as i64;

        n_point = vec![
            n_point[0],
            rot_cos * n_point[1] + -rot_sin * n_point[2],
            rot_sin * n_point[1] + rot_cos * n_point[2],
        ];

        n_point = vec![
            rot_1_cos * n_point[0] + rot_1_sin * n_point[2],
            n_point[1],
            -rot_1_sin * n_point[0] + rot_1_cos * n_point[2],
        ];

        n_point = vec![
            rot_2_cos * n_point[0] + -rot_2_sin * n_point[1],
            rot_2_sin * n_point[0] + rot_2_cos * n_point[1],
            n_point[2],
        ];

        if !ans.contains(&n_point) {
            ans.push(n_point);
        }
    }
    // println!("{:?}", ans);
    ans
}
