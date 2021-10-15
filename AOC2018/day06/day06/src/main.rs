use std::{collections::{HashMap, HashSet}, fs};
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();
    let mut coordinates = lines
        .split("\r\n")
        .map(|l| {
            l.split(", ")
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    coordinates.sort_unstable();
        
    let maps = inefficient_run(&coordinates);
        
        
    
    println!("{:?}", maps.iter().max_by(|a, b| a.1.cmp(b.1)) );
        
}

type Point = (i64, i64);
const MOVES: [[i64; 2]; 4] = [[1, 0], [0, 1], [0, -1], [-1, 0]];
fn manhattan_distance(p1: Point, p2: Point) -> i64 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn inefficient_run(coordinates: &Vec<Vec<usize>>) ->  HashMap<(usize, usize), usize> {
    let min_y = coordinates.iter().map(|c| c[0]).min().unwrap();
    let max_y = coordinates.iter().map(|c| c[0]).max().unwrap();
    let min_x = coordinates.iter().map(|c| c[1]).min().unwrap();
    let max_x = coordinates.iter().map(|c| c[1]).max().unwrap();

    let mut points: HashMap<(usize, usize), usize> = HashMap::new();

    let mut stack: Vec<(usize, usize)> = Vec::with_capacity(256);
    let mut visited = HashSet::new();
    let mut infinites = HashSet::new();


    stack.push((coordinates[0][0], coordinates[0][1]));
    visited.insert((coordinates[0][0], coordinates[0][1]));
    // if out of bounds then put out of misery
    while !stack.is_empty() {
        let (y, x) = stack.pop().unwrap();      
        for [ny, nx] in MOVES {

            let (nyy, nxx) = ((y as i64 + ny) as usize, (x as i64 + nx) as usize);
            if !visited.contains(&(nyy, nxx)) {
                let (mut mins, mut min, mut min_c, mut coord_m) = (vec![0; coordinates.len()], i64::MAX, 0, (0, 0));

                for (i, coord) in coordinates.iter().enumerate() {
                    mins[i] = manhattan_distance((nyy as i64, nxx as i64), (coord[0] as i64, coord[1] as i64));
                    if mins[i] == 0 { mins[i] = i64::MAX; }
                    match (mins[i] == min, mins[i] < min) {
                        (true, _)=>{ min_c += 1; }
                        (_, true)=>{ min = mins[i]; min_c = 1; coord_m = (coord[0], coord[1]) }
                        (_, _)=>{ }
                    }
                    
                }
                if min_c == 1 {
                    if nyy < min_y || nyy > max_y || nxx < min_x || nxx > max_x {
                        infinites.insert(coord_m);
                    } else {
                        stack.push((nyy, nxx));
                        visited.insert((nyy, nxx));
                    }
                    let entry = points.entry(coord_m).or_default();
                    *entry += 1;
                }
            }
            
        }
        

    }

    points.into_iter().filter(|(k, _)| !infinites.contains(k)).collect()






} 