use std::{cmp::Ordering, f32::consts::PI};

use line_drawing::Bresenham;

fn angle_between((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> f64 {
    let mut ang =
        f64::atan2(y1 as f64 - y2 as f64, x1 as f64 - x2 as f64) * 180.0 / PI as f64 - 90.0;

    if ang < 0.0 {
        ang += 360.0;
    }

    if ang.round() == 360.0 {
        0.0
    } else {
        ang
    }
}

fn get_quadrant(p1: (i64, i64), cp: (i64, i64)) -> usize {
    if cp.0 > p1.0 {
        return if cp.1 > p1.1 { 4 } else { 1 };
    }
    if cp.1 > p1.1 {
        3
    } else {
        2
    }
}

fn sort_from_12((y1, x1): (i64, i64), (y2, x2): (i64, i64), (yc, xc): (i64, i64)) -> Ordering {
    let (quad_1, quad_2) = (
        get_quadrant((y1, x1), (yc, xc)),
        get_quadrant((y2, x2), (yc, xc)),
    );

    if quad_1 != quad_2 {
        return if quad_1 < quad_2 {
            Ordering::Less
        } else {
            Ordering::Greater
        };
    }
    let (ang_1, ang_2) = (
        angle_between((yc, xc), (y1, x1)),
        angle_between((yc, xc), (y2, x2)),
    );

    if ang_1 < ang_2 {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

pub fn find_all_the_visibles(
    y: usize,
    x: usize,
    grid: &[Vec<char>],
) -> (usize, Vec<(usize, usize)>) {
    // type DoubleSet = (HashSet<f64>, HashSet<(usize, usize)>);

    let mut visited_gradients = Vec::new();
    let mut points = Vec::new();
    for (y1, row) in grid.iter().enumerate() {
        for (x1, c) in row.iter().enumerate() {
            if *c == '#' && (y1, x1) != (y, x) {
                let point = angle_between((x as i64, y as i64), (x1 as i64, y1 as i64));
                if !visited_gradients.contains(&point) {
                    visited_gradients.push(point);
                    points.push((y1, x1));
                }
            }
        }
    }
    points.sort_by(|a, b| {
        sort_from_12(
            (a.0 as i64, a.1 as i64),
            (b.0 as i64, b.1 as i64),
            (y as i64, x as i64),
        )
    });
    (visited_gradients.len(), points)
}

pub fn vaporize(y: usize, x: usize, grid: &mut [Vec<char>]) -> (usize, usize) {
    let mut points = Vec::with_capacity(200);

    while points.len() < 200 {
        let mut vis: Vec<f64> = Vec::new();
        let mut good_points: Vec<((usize, usize), f64)> = Vec::new();
        for (yj, xj) in find_all_the_visibles(y, x, grid).1 {
            for (y1, x1) in
                Bresenham::new((y as isize, x as isize), (yj as isize, xj as isize)).skip(1)
            {
                let angle_between_center =
                    angle_between((x as i64, y as i64), (x1 as i64, y1 as i64));
                if grid[y1 as usize][x1 as usize] == '#' && !vis.contains(&angle_between_center) {
                    good_points.push(((y1 as usize, x1 as usize), angle_between_center));
                    grid[y1 as usize][x1 as usize] = '.';
                    vis.push(angle_between_center);
                }
            }
        }
        good_points.sort_by(|a, b| {
            if a.1 > b.1 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });

        points.extend(good_points.iter().map(|e| e.0));
    }

    (points[199].1, points[199].0)
}
