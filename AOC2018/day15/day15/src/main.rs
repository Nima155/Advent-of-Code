#![feature(map_first_last)]
use std::{
    collections::{BTreeMap, BTreeSet, HashSet, VecDeque},
    fs,
};
fn main() {
    let grid_string = fs::read_to_string("../input.txt").unwrap();

    let mut grid_to_vec = grid_string
        .split("\r\n")
        .into_iter()
        .map(|r| {
            r.chars()
                .map(|c| (c, if c == 'G' || c == 'E' { 200 } else { 0 }))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut player_stats = grid_to_vec
        .iter()
        .enumerate()
        .map(|(i, r)| {
            r.iter()
                .enumerate()
                .filter(|(_, (c, _))| *c == 'G' || *c == 'E')
                .map(|(j, (c, _))| ((i, j), (*c, 200)))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<BTreeMap<_, _>>();

    println!("{}", play_the_game(&mut grid_to_vec, &mut player_stats));
}

// up left right down -> order is important
const MOVES: [[i64; 2]; 4] = [[-1, 0], [0, -1], [0, 1], [1, 0]];

// each unit has 3 attack power
fn play_the_game(
    grid: &mut [Vec<(char, i64)>],
    player_stats: &mut BTreeMap<(usize, usize), (char, i64)>,
) -> i64 {
    let mut rounds = 0;
    loop {
        let mut clono = BTreeMap::new();

        if only_of_one_type(player_stats) {
            // println!("{:?} {}", player_stats, rounds);
            return player_stats.iter().map(|(_, v)| v.1).sum::<i64>() * (rounds - 1);
        }

        while !player_stats.is_empty() {
            // println!("{}: {}", player_stats.len(), "hey there");

            let ((mut ny, mut nx), (warrior_type, hp)) = player_stats.pop_first().unwrap();

            let mut viable_neighbours = BTreeSet::new();

            let (nyy, nxx) = shortest_distance(grid, (ny, nx));
            // println!("{} {}", nyy, nxx );

            if (nyy, nxx) != (1000, 1000) && grid[nyy][nxx].0 == '.' {
                grid[nyy][nxx] = grid[ny][nx];
                grid[ny][nx] = ('.', 0);
                clono.insert((nyy, nxx), (warrior_type, hp));
                ny = nyy;
                nx = nxx;
            } else if !clono.contains_key(&(ny, nx)) {
                clono.insert((ny, nx), (warrior_type, hp));
            }

            for [y, x] in MOVES {
                let (ny, nx) = (ny as i64 + y, nx as i64 + x);
                let target = if warrior_type == 'G' { 'E' } else { 'G' };

                if ny >= 0
                    && ny < grid.len() as i64
                    && nx >= 0
                    && nx < grid[0].len() as i64
                    && grid[ny as usize][nx as usize].0 == target
                {
                    viable_neighbours
                        .insert((grid[ny as usize][nx as usize].1, (ny as usize, nx as usize)));
                }
            }

            if !viable_neighbours.is_empty() {
                let (rem_hp, (y, x)) = viable_neighbours.pop_first().unwrap();

                if !clono.contains_key(&(ny, nx)) {
                    clono.insert((ny, nx), (warrior_type, hp));
                }

                if rem_hp > 3 {
                    clono.insert((y, x), (grid[y][x].0, rem_hp));
                    clono.entry((y, x)).and_modify(|c| c.1 -= 3);
                    grid[y][x].1 -= 3;
                } else {
                    grid[y][x] = ('.', 0);
                    player_stats.remove(&(y, x));
                    clono.remove(&(y, x));
                }
            }
        }

        rounds += 1;
        *player_stats = clono;
        // println!("{:?} \n{}\n", player_stats, grid.iter().map(|l| l.iter().map(|c| c.0).collect::<String>()).collect::<Vec<_>>().join("\n"));
    }
}

fn shortest_distance(grid: &[Vec<(char, i64)>], cur_point: (usize, usize)) -> (usize, usize) {
    let mut queue: VecDeque<(usize, usize, (i64, i64))> = VecDeque::new();
    let mut visited = HashSet::new();
    let target = if grid[cur_point.0][cur_point.1].0 == 'G' {
        'E'
    } else {
        'G'
    };
    queue.push_back((cur_point.0, cur_point.1, (-1, -1)));

    while !queue.is_empty() {
        let (y, x, path) = queue.pop_front().unwrap();
        if grid[y][x].0 == target {
            return (path.0 as usize, path.1 as usize);
        }
        for [i, j] in MOVES {
            let (ny, nx) = (i + y as i64, j + x as i64);
            if ny >= 0
                && ny < grid.len() as i64
                && nx >= 0
                && nx < grid[0].len() as i64
                && !visited.contains(&(ny, nx))
                && grid[ny as usize][nx as usize].0 != '#'
                && grid[ny as usize][nx as usize].0 != grid[cur_point.0][cur_point.1].0
            {
                let n_path = (
                    if path.0 != -1 { path.0 } else { ny },
                    if path.1 != -1 { path.1 } else { nx },
                );
                visited.insert((ny, nx));
                queue.push_back((ny as usize, nx as usize, n_path));
            }
        }
    }
    (1000, 1000)
}

fn only_of_one_type(players: &BTreeMap<(usize, usize), (char, i64)>) -> bool {
    // println!("{:?}", players);
    players
        .iter()
        .all(|(_, c)| c.0 == players.first_key_value().unwrap().1 .0)
}

// for (p, (c, hp)) in player_stats.iter() {
//     if grid[p.0][p.1] == 'E' || grid[p.0][p.1] == 'G' {

//         let mut viable_neighbours = BTreeSet::new();
//         let target = if *c == 'E' { 'G' } else { 'E' };

//         for [y, x] in MOVES {
//             let (ny, nx) = (p.0 as i64 + y, p.1 as i64 + x);
//             if ny >= 0
//                 && ny < grid.len() as i64
//                 && nx >= 0
//                 && nx < grid[0].len() as i64
//                 && grid[ny as usize][nx as usize] == target
//             {
//                 let neighbor_hp =
//                     if players_stats_clone.contains_key(&(ny as usize, nx as usize)) {
//                         players_stats_clone
//                             .get(&(ny as usize, nx as usize))
//                             .unwrap()
//                             .1
//                     } else {
//                         player_stats.get(&(ny as usize, nx as usize)).unwrap().1
//                     };
//                 viable_neighbours.insert((neighbor_hp, (ny, nx)));
//             }
//         }
//         if !viable_neighbours.is_empty() {
//             let (n_hp, (yy, xx)) = viable_neighbours.pop_first().unwrap();
//             if n_hp <= 3 {
//                 grid[yy as usize][xx as usize] = '.';
//                 players_stats_clone.remove(&(yy as usize, xx as usize));
//             } else {
//                 let entry = new_hps.entry((yy as usize, xx as usize)).or_insert(n_hp);
//                 if *entry <= 3 {
//                     grid[yy as usize][xx as usize] = '.';
//                     players_stats_clone.remove(&(yy as usize, xx as usize));
//                     // new_hps.remove(&(yy as usize, xx as usize));
//                 } else {
//                     *entry -= 3;
//                 }
//             }
//         } else {
//             // TODO: refine
//             let (ty, tx) = shortest_distance(grid, *p);

//             if (ty, tx) != (1000, 1000) {
//                 grid[ty][tx] = grid[p.0][p.1];
//                 grid[p.0][p.1] = '.';
//                 players_stats_clone
//                     .insert((ty, tx), (*c, *new_hps.entry((ty, tx)).or_insert(*hp)));
//             }
//         }
//     }
