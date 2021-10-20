use std::{collections::{BTreeMap, HashMap}, fs};

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
                .filter(|(_, c)| **c == '>' || **c == '<' || **c == 'V' || **c == '^')
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
        '\\' => hash_map!('V' => vec!['>'], '^' => vec!['<'], '>' => vec!['V'], '<' => vec!['^']), 
        '/' => hash_map!('V' => vec!['<'], '^' => vec!['>'], '>' => vec!['^'], '<' => vec!['V']),
        '+' => hash_map!('V' => vec!['>', 'V', '<'], '>' =>vec! ['^', '>', 'V'], '<' => vec!['V', '<', '^'], '^' => vec! ['<', '^', '>']));

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
        let mut players_after = BTreeMap::new();
        for ((y, x), (arrow, turn)) in players.iter() {
            
            let mut arrow = *arrow;

            let (ny, nx) = match arrow {
                '>' => (*y, *x + 1),
                '^' => (*y - 1, *x),
                '<' => (*y, *x - 1),
                'V' => (*y + 1, *x),
                _ => (*y, *x),
            };

            arrow = obstacle_to_new_arrow(grid[ny][nx], arrow, *turn);

            if players_after.contains_key(&(ny, nx)) || players.contains_key(&(ny, nx)) {                
                return (nx, ny);
            }

            players_after.insert((ny, nx), (arrow, *turn + (grid[ny][nx] == '+') as usize));
        }
        
        players = players_after;
    }
    (0, 0)
}

