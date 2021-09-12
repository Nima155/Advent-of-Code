use std::collections::{HashMap, HashSet};



const MOVES: [(char, i32, i32); 4] = [('U', -1, 0), ('R', 0, 1), ('L', 0, -1), ('D', 1, 0)];

pub fn find_visited_points(move_instructions: &Vec<(&str, u64)>) -> (HashSet<(i32, i32)>, HashMap<(i32, i32), u32>) {
    let (mut s_x, mut s_y, mut steps) = (0, 0, 0);
    
    let mut visited_points = HashSet::new();
    let mut min_steps_to_point = HashMap::new();

    for mv in move_instructions {
        let mv_moves = MOVES
            .iter()
            .find(|f| f.0 == mv.0.chars().next().unwrap())
            .unwrap();
        for _ in 0..mv.1 {
            s_x += mv_moves.2;
            s_y += mv_moves.1;
            steps += 1;
            visited_points.insert((s_x, s_y));
            
            if !min_steps_to_point.contains_key(&(s_x, s_y)) {
                min_steps_to_point.insert((s_x, s_y), steps);
            }
        }
    }
    (visited_points, min_steps_to_point)
}

pub fn find_closest_to_point_of_origin(intersections: &HashSet<&(i32, i32)>, maps: &[&HashMap<(i32, i32), u32>;2]) -> u32 {
    let mut ans = u32::MAX;

    for p in intersections {
        ans = u32::min(ans, maps[0].get(p).unwrap() + maps[1].get(p).unwrap());
    }
    ans
}
