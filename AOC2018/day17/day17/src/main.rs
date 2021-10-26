use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let read = fs::read_to_string("../input.txt").unwrap();

    let mut occupied_points = HashSet::new();
    let mut grid_bounds = HashMap::new();
    read.split("\r\n").for_each(|l| {
        if let [one, two, ..] = l.split(", ").collect::<Vec<_>>()[..] {
            if let [num1, num2, ..] = two.split("..").collect::<Vec<_>>()[..] {
                let y = if &one[0..1] == "y" {
                    one[2..].parse::<i64>().unwrap()
                } else {
                    0
                };
                let x = if &one[0..1] == "x" {
                    one[2..].parse::<i64>().unwrap()
                } else {
                    0
                };
                for i in num1[2..].parse::<i64>().unwrap()..=num2.parse::<i64>().unwrap() {
                    occupied_points.insert(if x == 0 {
                        (y, i, WaterType::Wall)
                    } else {
                        (i, x, WaterType::Wall)
                    });
                    let entry = grid_bounds
                        .entry(if x == 0 { y } else { i })
                        .or_insert(vec![]);
                    entry.push(if x == 0 { i } else { x });
                }
            }
        }
    });
    let max_y = occupied_points
        .iter()
        .max_by(|a, b| a.0.cmp(&b.0))
        .unwrap()
        .0;
    let min_y = occupied_points
        .iter()
        .min_by(|a, b| a.0.cmp(&b.0))
        .unwrap()
        .0;

    // println!("{}", max_y);

    run_the_water(&mut occupied_points, max_y, min_y);

    // println!("{:?}", occupied_points.iter().filter(|p| p.2 != -1).map(|(p1, p2, _)| (p1, p2)).collect::<BTreeSet<_>>());
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum WaterType {
    Stale,
    Running,
    Wall,
}

fn run_the_water(occupied_points: &mut HashSet<(i64, i64, WaterType)>, max_y: i64, min_y: i64) {
    use WaterType::{Running, Stale};

    let occupied_clone = occupied_points.clone();
    let mut qu = vec![(min_y, 500, Running)];
    let mut last_source = 500;
    let mut runnings = vec![vec![0; 900]; 2000];
    while !qu.is_empty() {
        let (y, x, water_type) = qu.pop().unwrap();

        // println!("{}", occupied_points.len() - occupied_clone.len());
        
        if y > max_y || y < min_y {
            occupied_points.remove(&(y, x, water_type));
            continue;
        }



        occupied_points.insert((y, x, water_type));

        let [down, mut right, mut left] = [
            (y + 1, x, water_type),
            (y, x + 1, water_type),
            (y, x - 1, water_type),
        ];

        if !is_occupied(&occupied_points, down) && water_type == Running {
            qu.push(down);
        } 
        
        else {
            let mut intermediate_map = HashSet::new();


            while is_occupied(&occupied_points, (left.0 + 1, left.1, water_type))
                && !is_occupied(&occupied_clone, left)
            {
                intermediate_map.insert(left);
                left.1 -= 1;
            }

            while is_occupied(&occupied_points, (right.0 + 1, right.1, water_type))
                && !is_occupied(&occupied_clone, right)
            {
                intermediate_map.insert(right);
                right.1 += 1;
            }

            if is_occupied(&occupied_points, left) && is_occupied(&occupied_points, right) {
                
                intermediate_map.iter().for_each(|p| {
                    occupied_points.insert((p.0, p.1, Stale));
                });

                occupied_points.insert((y, x, Stale));
                last_source = x;
     
                if (last_source - left.1).abs() < (last_source - right.1).abs()
                {
                    qu.push((left.0 - 1, left.1 + 1, Running));
                } 
                else if (last_source - left.1).abs() > (last_source - right.1).abs()
                {
                    qu.push((right.0 - 1, right.1 - 1, Running));
                } 
                else {

                    
                        qu.push((right.0 - 1, right.1 - 1, Running));
                    

                    
                        qu.push((left.0 - 1, left.1 + 1, Running));
                    

                }
            } else {
                
                if !is_occupied(occupied_points, (right.0 + 1, right.1, water_type)) 
                    && runnings[(right.0 + 1) as usize][right.1 as usize] < 100 {

                    runnings[(right.0 + 1) as usize][right.1 as usize] += 1;
                    occupied_points.insert((right.0, right.1, Running));
                    last_source = right.1;
                    
                    qu.push((right.0 + 1, right.1, Running));
                }

                if !is_occupied(occupied_points, (left.0 + 1, left.1, water_type)) 
                && runnings[(left.0 + 1) as usize][left.1 as usize] < 100 {

                    runnings[(left.0 + 1) as usize][left.1 as usize] += 1;
                    occupied_points.insert((left.0, left.1, Running));
                    last_source = left.1;
                    qu.push((left.0 + 1, left.1, Running));
                }
                
                occupied_points.extend(intermediate_map.iter().map(|p| (p.0, p.1, Running)));
            }
        }
    }

    // println!("{:?}", occupied_points.difference(&occupied_clone).collect::<BTreeSet<_>>());
    // let mut ans = vec![
    //     vec![' '; occupied_points.iter().map(|e| e.1).max().unwrap() as usize + 1];
    //     (max_y + 1) as usize
    // ];

    // for p in occupied_clone.iter() {
    //     ans[p.0 as usize][p.1 as usize] = '#';
    // }

    // println!("{:?}", occupied_points.difference(&occupied_clone).collect::<BTreeSet<_>>());

    println!("{}", occupied_points
        .difference(&occupied_clone).map(|e| (e.0, e.1))
        .collect::<HashSet<_>>()
        .iter()
        .count());

    // fs::write(
    //     "./output.txt",
    //     ans.iter()
    //         .map(|l| l.iter().collect::<String>())
    //         .collect::<Vec<_>>()
    //         .join("\r\n"),
    // )
    // .unwrap();
    // println!("{:?}", occupied_points.iter().filter(|p| p.2 != -1).map(|(p1, p2, _)| (p1, p2)).collect::<BTreeSet<_>>());
    // occupied_points.difference(&occupied_clone).collect::<HashSet<_>>().iter().filter(|e| e.0 >= min_y).count() as i64
}

fn is_occupied(
    occupied_points: &HashSet<(i64, i64, WaterType)>,
    cur_point: (i64, i64, WaterType),
) -> bool {
    occupied_points.contains(&(cur_point.0, cur_point.1, WaterType::Wall))
        || occupied_points.contains(&(cur_point.0, cur_point.1, WaterType::Stale))
}
