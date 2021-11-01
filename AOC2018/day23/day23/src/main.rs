use std::fs;
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();

    let mut parsed_data = lines
        .split("\r\n")
        .map(|l| {
            if let [pos, signal_range] = l.split(", ").collect::<Vec<_>>()[..] {
                let coords = pos[5..pos.len() - 1]
                    .split(',')
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect::<Vec<_>>();
                return (coords, signal_range[2..].parse::<i64>().unwrap());
            }
            (vec![], 0)
        })
        .collect::<Vec<_>>();

    parsed_data.sort_by(|a, b| b.1.cmp(&a.1));

    println!("{}", eligible_members(&parsed_data));
}

fn manhattan_is_far(point1: &Vec<i64>, point2: &Vec<i64>) -> i64 {
    point1
        .iter()
        .zip(point2.iter())
        .fold(0, |up_to, (a, b)| up_to + (a - b).abs())
}

fn eligible_members(data: &Vec<(Vec<i64>, i64)>) -> i64 {
    let rest = &data[1..];

    let (points, range) = &data[0];

    let mut ans = 0;

    for (other_points, r) in rest {
        if *range >= manhattan_is_far(points, other_points) {
            ans += 1;
        }
    }
    ans + 1
}
