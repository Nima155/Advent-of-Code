use std::collections::HashSet;


type Point = (i32, i32, i32);

pub fn generate_3d_moves(
) -> [[i8;3];26] {
     
    let mut ans = [[0;3];26];

    let mut indx = 0;

    for i in -1..2 {
        for j in -1..2 {
            for m in -1..2 {
                if i != 0 || j != 0 || m != 0 {
                    ans[indx] = [i, j, m];
                    indx += 1;
                }
            }
        }
    }
    ans
}

pub fn get_active_nodes(grid: &[Vec<char>]) -> HashSet<Point> {
    grid.iter().enumerate().map(|(i, subgrid)| {
        subgrid.iter().enumerate().filter_map(|(j,c)| 
            if let '#' = *c {
                Some((i as i32, j as i32, 0_i32))
            } else {
                None
            }
        ).collect::<HashSet<Point>>()
    }).flatten().collect::<HashSet<Point>>()
}

// If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
// If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
pub fn node_cycle(actives: &HashSet<Point>) -> HashSet<Point> {

    let moves = generate_3d_moves();

    let mut new_actives = HashSet::with_capacity(actives.len());

    for (y, x, z) in actives.iter() {
        let mut count = 0;
        for [i, j, v] in moves {
            let new_point = (*y + i as i32, *x + j as i32, *z + v as i32);
            if actives.contains(&new_point) {
                count += 1;
            }
            else {
                let mut bad_count_actives = 0;
                for [i_bad, j_bad, v_bad] in moves {
                    let new_point_inactive = (new_point.0 + i_bad as i32, new_point.1 + j_bad as i32, new_point.2 + v_bad as i32);
                    if actives.contains(&new_point_inactive) {
                        bad_count_actives += 1;
                    }
                }
                if bad_count_actives == 3 {
                    new_actives.insert(new_point);
                }
            }
        }
        if count == 2 || count == 3 {
            new_actives.insert((*y, *x, *z));
        }
    }
    
    new_actives
}