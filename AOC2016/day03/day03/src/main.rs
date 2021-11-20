use std::fs;
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();

    let points = lines
        .split("\r\n")
        .map(|l| {
            l.trim()
                .split(' ').filter(| n | !n.is_empty())
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!(
        "{}",
        points
            .iter()
            .filter(|p| triangle_test((p[0], p[1], p[2])))
            .count()
    );
}

type Point = (i64, i64, i64);

fn triangle_test(point: Point) -> bool {
    (point.0 + point.1 > point.2)
        && (point.0 + point.2 > point.1)
        && (point.2 + point.1 > point.0)
}
