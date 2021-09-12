use std::collections::HashSet;

const MOVES: [(char, i32, i32); 4] = [('U', -1, 0), ('R', 0, 1), ('L', 0, -1), ('D', 1, 0)];

pub fn find_visited_points(move_instructions: &Vec<(&str, u64)>) -> HashSet<(i32, i32)> {
    let (mut s_x, mut s_y) = (0, 0);

    let mut visited_points = HashSet::new();

    for mv in move_instructions {
        let mv_moves = MOVES.iter().find(|f| f.0 == mv.0.chars().next().unwrap()).unwrap();
        for _ in 0..mv.1 {
            s_x += mv_moves.2;
            s_y += mv_moves.1;
            visited_points.insert((s_x, s_y));
        }
    }
    visited_points
}


pub fn find_closest_to_point_of_origin(intersections: &HashSet<&(i32, i32)>) -> i32 {
    let mut ans = i32::MAX;


    for p in intersections {
        ans = i32::min(p.0.abs() + p.1.abs(), ans);
    }
    ans
}