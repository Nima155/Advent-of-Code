use std::fs;

fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();
    let mut asn = lines
        .split("\r\n")
        .map(|l| {
            let (o, t) = &l[10..24].split_at(6);
            let (oo, tt) = &l[36..l.len() - 1].split_at(2);

            // println!("{} {}",  pos.1, velo.1);
            (
                (
                    o[0..].trim().parse::<i64>().unwrap(),
                    t[1..].trim().parse::<i64>().unwrap(),
                ),
                (
                    oo[0..].trim().parse::<i64>().unwrap(),
                    tt[1..].trim().parse::<i64>().unwrap(),
                ),
            )
        })
        .collect::<Vec<_>>();

    let (mut ans_vec, mut min_diff) = (Vec::new(), i64::MAX);
    let (mut seconds, mut sec_ans) = (0, 0);

    'a: loop {
        let diff =
            asn.iter().map(|e| e.0 .0).max().unwrap() - asn.iter().map(|e| e.0 .0).min().unwrap();
        if diff < min_diff {
            min_diff = diff;
            ans_vec = asn.clone();
            sec_ans = seconds;
        }
        for ((y, x), (y_inx, x_inx)) in asn.iter_mut() {
            if *y_inx < 0 && *y < y_inx.abs() || *x_inx < 0 && *x < x_inx.abs() {
                break 'a;
            }
            *y += *y_inx;
            *x += *x_inx;
        }
        seconds += 1;
    }

    let (y_max, y_min, x_max, x_min) = ans_vec
        .iter()
        .fold((ans_vec[0].0.0, ans_vec[0].0.0, ans_vec[0].0.1, ans_vec[0].0.1), |acc, ot| 
        (acc.0.max(ot.0.0), acc.1.min(ot.0.0), acc.2.max(ot.0.1), acc.3.min(ot.0.1))
    );


    let mut vec_ans = vec![vec![' '; (y_max - y_min + 1) as usize]; (x_max - x_min + 1) as usize];

    for ((i, j), _) in ans_vec {
        vec_ans[(x_max - j) as usize][(y_max - i) as usize] = '#';
    }

    println!(
        "{}",
        vec_ans
            .iter()
            .map(|e| e.iter().rev().collect::<String>())
            .rev()
            .collect::<Vec<_>>()
            .join("\n")
    );

    println!("Completed in seconds: {}", sec_ans);
    // let canva
}
