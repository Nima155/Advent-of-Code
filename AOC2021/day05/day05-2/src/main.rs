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
        map_the_points((two_points[0][0], two_points[0][1]), (two_points[1][0], two_points[1][1] ), &mut visited_counts);
    }
    visited_counts.iter().filter(|(_, v)| **v >= 2).count()
}

fn map_the_points(mut source: (i64, i64), dest: (i64, i64), visited_counts: &mut HashMap<(i64, i64), i64>) {

    loop {
        let ent = visited_counts
                    .entry((
                        source.0, source.1
                    ))
                    .or_default();
        *ent += 1;

        if source == dest {
            break;
        }

        if source.0 > dest.0 {
            source.0 -= 1;
        } else if source.0 != dest.0 {
            source.0 += 1;
        }
        if source.1 > dest.1 {
            source.1 -= 1;
        } else if source.1 != dest.1 {    
            source.1 += 1;
        }
    }
}
