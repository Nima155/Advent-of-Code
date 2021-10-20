use std::{collections::{BTreeMap, HashMap, HashSet}, fs};

macro_rules! hash_map {
    ($($k: expr => $v: expr ),+) => {{
        let mut map = HashMap::new();

        $(map.insert($k, $v);)+
        map
    }};
}

fn main() {
    let grid = fs::read_to_string("../input.txt").unwrap();

    let grid_to_vec = grid
        .split("\r\n")
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    
    // save the positions...
    let players = grid_to_vec
        .iter()
        .enumerate()
        .map(|(i, l)| {
            l.iter()
                .enumerate()
                .filter(|(_, c)| **c == '>' || **c == '<' || **c == 'v' || **c == '^')
                .map(|(j, _)| (i, j))
                .collect::<Vec<_>>()
        })
        .filter(|e| !e.is_empty())
        .flatten()
        .map(|f| (f, (grid_to_vec[f.0][f.1], 0)))
        .collect::<BTreeMap<_, _>>();

    println!("{:?}", walk_the_walk(players, &grid_to_vec))
}

fn obstacle_to_new_arrow(obstacle: char, arrow: char, turn: usize) -> char {
    // intersection: L S R
    let my_map = hash_map!(
        '\\' => hash_map!('v' => vec!['>'], '^' => vec!['<'], '>' => vec!['v'], '<' => vec!['^']), 
        '/' => hash_map!('v' => vec!['<'], '^' => vec!['>'], '>' => vec!['^'], '<' => vec!['v']),
        '+' => hash_map!('v' => vec!['>', 'v', '<'], '>' =>vec! ['^', '>', 'v'], '<' => vec!['v', '<', '^'], '^' => vec! ['<', '^', '>']));

    match obstacle {
        '\\' | '/' => my_map.get(&obstacle).unwrap().get(&arrow).unwrap()[0],
        '+' => my_map.get(&'+').unwrap().get(&arrow).unwrap()[turn % 3],
        _ => arrow,
    }
}

fn walk_the_walk(
    mut players: BTreeMap<(usize, usize), (char, usize)>,
    grid: &[Vec<char>],
) -> (usize, usize) {
    loop {
        
        if players.len() == 1 {
            println!("{:?}", players);
            break;
        }
        let mut players_after = BTreeMap::new();
        let mut black_list = HashSet::new();
        for ((y, x), (arrow, turn)) in players.iter() {
            if !black_list.contains(&(*y, *x)) {
                let mut arrow = *arrow;
    
                let (ny, nx) = match arrow {
                    '>' => (*y, *x + 1),
                    '^' => (*y - 1, *x),
                    '<' => (*y, *x - 1),
                    'v' => (*y + 1, *x),
                    _ => (*y, *x),
                };
                
                arrow = obstacle_to_new_arrow(grid[ny][nx], arrow, *turn);
                // println!("{} {}", players_after.contains_key(&(ny, nx)), players.contains_key(&(ny, nx)));
                match (players_after.contains_key(&(ny, nx)), players.contains_key(&(ny, nx))) {
                    (true, _) => {
                        players_after.remove(&(ny, nx));
                    }
                    (_, true) => {
                        black_list.insert((ny, nx));
                    }
                    _ => { players_after.insert((ny, nx), (arrow, *turn + (grid[ny][nx] == '+') as usize)); }
                }                
            }
                
            

            
        }
        
        players = players_after;
    }
    (0, 0)
}

