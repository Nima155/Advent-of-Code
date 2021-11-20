use std::fs;

fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();

    let points = lines
        .split("\r\n")
        .map(|l| {
            l.trim()
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let vec_1 = points.iter().map(|l| l[0]).collect::<Vec<_>>();
    let vec_1 = vec_1.chunks_exact(3).collect::<Vec<_>>();
    let vec_2 = points.iter().map(|l| l[1]).collect::<Vec<_>>();
    let vec_2 = vec_2.chunks_exact(3).collect::<Vec<_>>();
    let vec_3 = points.iter().map(|l| l[2]).collect::<Vec<_>>();
    let vec_3 = vec_3.chunks_exact(3).collect::<Vec<_>>();

    println!(
        "{}",
        valid_triangle_count(&vec_1) + valid_triangle_count(&vec_2) + valid_triangle_count(&vec_3) 
    );
}

fn valid_triangle_count(vector: &[&[i64]]) -> i64 {
    vector
    .iter()
    .filter(|l| triangle_test((l[0], l[1], l[2])))
    .count() as i64

}

type Point = (i64, i64, i64);

fn triangle_test(point: Point) -> bool {
    (point.0 + point.1 > point.2) && (point.0 + point.2 > point.1) && (point.2 + point.1 > point.0)
}
