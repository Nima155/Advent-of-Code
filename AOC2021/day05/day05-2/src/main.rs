use std::{collections::HashMap, fs};
fn main() {
    let points = fs::read_to_string("../input.txt").unwrap();
    let points = points
        .split("\r\n")
        .map(|l| {
            l.split(" -> ")
                .map(|p| {
                    p.split(',')
                        .map(|n| n.parse::<i64>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    print!("{}", intersection_counter(&points));
}

fn intersection_counter(points: &[Vec<Vec<i64>>]) -> usize {
    let mut visited_counts: HashMap<(i64, i64), i64> = HashMap::new();
    for two_points in points {
        let [x_eq, y_eq] = [
            two_points[0][0] == two_points[1][0],
            two_points[0][1] == two_points[1][1],
        ];
        if x_eq || y_eq {
            let [mut p_1, p_2] = [
                two_points[0][if x_eq { 1 } else { 0 }],
                two_points[1][if x_eq { 1 } else { 0 }],
            ];
            let stable_point = if x_eq {
                two_points[0][0]
            } else {
                two_points[0][1]
            };
            loop {
                
                let ent = visited_counts
                    .entry((
                        if x_eq { stable_point } else { p_1 },
                        if y_eq { stable_point } else { p_1 },
                    ))
                    .or_default();
                *ent += 1;

                if p_1 == p_2 {
                    break;
                }
                if p_1 > p_2 {
                    p_1 -= 1;
                } else {
                    p_1 += 1;
                }

                
            }
            
        } else {
            let [mut p1, mut p2] = [two_points[0][0], two_points[0][1]];
            loop {
                
                let ent = visited_counts
                    .entry((
                        p1,
                        p2
                    ))
                    .or_default();
                *ent += 1;

                if [p1, p2] == two_points[1][..] {
                    break;
                }
                if p1 > two_points[1][0] {
                    p1 -= 1;
                } else {
                    p1 += 1;
                }
                if p2 > two_points[1][1] {
                    p2 -= 1;
                } else {    
                    p2 += 1;
                }

                
            }

        }
    }
    visited_counts.iter().filter(|(_, v)| **v >= 2).count()
}
