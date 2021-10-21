#![feature(map_first_last)]
use std::{
    collections::{BTreeMap, BTreeSet, HashSet, VecDeque},
    fs,
};
fn main() {
    let grid_string = fs::read_to_string("../input.txt").unwrap();

    let grid_to_vec = grid_string
        .split("\r\n")
        .into_iter()
        .map(|r| {
            r.chars()
                .map(|c| (c, if c == 'G' || c == 'E' { 200 } else { 0 }))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let player_stats = grid_to_vec
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

    // elves health
    for i in 3.. {
        let ans = play_the_game(&mut grid_to_vec.clone(), player_stats.clone(), i, grid_to_vec.iter().flatten().filter(|c| c.0 == 'E').count());

        if ans.2
        {
            println!("{} {:?}", ans.0, ans.1);
            break;
        }
    }
}

// up left right down -> order is important
const MOVES: [[i64; 2]; 4] = [[-1, 0], [0, -1], [0, 1], [1, 0]];

// each unit has 3 attack power
fn play_the_game(
    grid: &mut [Vec<(char, i64)>],
    mut player_stats: BTreeMap<(usize, usize), (char, i64)>,
    elf_power: i64,
    total_count: usize
) -> (i64, BTreeMap<(usize, usize), (char, i64)>, bool) {
    let mut rounds = 0;
    loop {
        let mut clono: BTreeMap<(usize, usize), (char, i64)> = BTreeMap::new();

        while !player_stats.is_empty() {
            // println!("{}: {}", player_stats.len(), "hey there");
            if only_of_one_type(grid) {
                clono.extend(player_stats.iter());
                return (
                    grid.iter().flatten().map(|(_, v)| v).sum::<i64>() * rounds,
                    clono,
                    grid.iter().flatten().filter(|c| c.0 == 'E').count() == total_count
                );
            }

            let ((mut ny, mut nx), (warrior_type, hp)) = player_stats.pop_first().unwrap();

            let mut viable_neighbours = BTreeSet::new();

            let (nyy, nxx) = shortest_distance(grid, (ny, nx));

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
                let inco = if warrior_type == 'E' { elf_power } else { 3 };

                if rem_hp > inco {
                    clono.insert((y, x), (grid[y][x].0, rem_hp - inco));
                    
                    grid[y][x].1 -= inco;
                } else {
                    grid[y][x] = ('.', 0);
                    player_stats.remove(&(y, x));
                    clono.remove(&(y, x));
                }
            }
        }

        rounds += 1;
        player_stats = clono;
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

fn only_of_one_type(players: &[Vec<(char, i64)>]) -> bool {
    // println!("{:?}", players);
    players
        .iter()
        .flatten()
        .filter(|c| c.0 == 'E' || c.0 == 'G')
        .all(|(car, _)| *car == 'E')
        || players
            .iter()
            .flatten()
            .filter(|c| c.0 == 'E' || c.0 == 'G')
            .all(|(car, _)| *car == 'G')
}
