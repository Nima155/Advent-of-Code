use std::{collections::{HashMap, HashSet}, fs};

fn main() {
    let read = fs::read_to_string("../input.txt").unwrap();

    let mut occupied_points = HashSet::new();
    let mut grid_bounds = HashMap::new();
    read.split("\r\n").for_each(|l | {
        if let [one, two, ..] = l.split(", ").collect::<Vec<_>>()[..] {
            if let [num1, num2, ..] = two.split("..").collect::<Vec<_>>()[..] {
                let y = if &one[0..1] == "y" { one[2..].parse::<i64>().unwrap() } else { 0 };
                let x = if &one[0..1] == "x" { one[2..].parse::<i64>().unwrap() } else { 0 };
                for i in num1[2..].parse::<i64>().unwrap()..=num2.parse::<i64>().unwrap() {
                    
                    occupied_points.insert(if x == 0 { (y, i) } else { (i, x) });
                    let entry = grid_bounds.entry(if x == 0 { y } else { i }).or_insert(vec![]);
                    entry.push(if x == 0 { i } else { x }); 
                }
                
            
            }
        }
    });
    let max_y = occupied_points.iter().max().unwrap().0;
    let min_y = occupied_points.iter().min().unwrap().0;

    println!("{}", max_y);

    println!("{}", run_the_water(&mut occupied_points, max_y, min_y));
    
    // println!("{:?}", occupied_points.iter().filter(|p| p.2 != -1).map(|(p1, p2, _)| (p1, p2)).collect::<BTreeSet<_>>());
}



fn run_the_water(occupied_points: &mut HashSet<(i64, i64)>, max_y: i64, min_y: i64) -> i64 {
    
    let occupied_clone = occupied_points.clone();
    let mut qu = vec![(min_y, 500)];
    let mut visited_ups = HashSet::new();
    
    while !qu.is_empty() {
        let (y, x) = qu.pop().unwrap();
        

      
        // println!("{}", occupied_points.len() - occupied_clone.len());
           
        if y > max_y || y < min_y {
            occupied_points.remove(&(y, x));
            continue;
        }

        occupied_points.insert((y, x));

        let [up, down, right, left] = [(y - 1, x), (y + 1, x), (y, x + 1), (y, x - 1)];


        if !occupied_points.contains(&down) {    
            qu.push(down);
        } 

        else 
        {
            // println!("{} {}", y, x);
            
            match (!occupied_points.contains(&left), !occupied_points.contains(&right)) {
                (true, true) => {
                    qu.push(left);
                    qu.push(right);
                }        
                (true, _) => {
                    qu.push(left);
                }        
                (_, true) => {
                    qu.push(right);
                }        
                _ =>
                {
                    
                    if !visited_ups.contains(&up) {
                        visited_ups.insert(up);
                        // left --> right or right --> left
                        let (mut left, mut right) = (up, (up.0, up.1 + 1));

                        let mut there_is_hole = false;

                        while !occupied_clone.contains(&left) {
                            occupied_points.insert(left);
                            if !occupied_points.contains(&(left.0 + 1, left.1)) {
                                if !occupied_clone.contains(&(left.0 + 1, left.1 + 1)) {
                                    there_is_hole = true;
                                    break;
                                }
                                qu.push((left.0 + 1, left.1));
                                there_is_hole = true;
                                break;
                            }
                            left.1 -= 1;
                            
                        }

                        while !occupied_clone.contains(&right) {
                            occupied_points.insert(right);
                            if !occupied_points.contains(&(right.0 + 1, right.1)) {
                                if !occupied_clone.contains(&(right.0 + 1, right.1 - 1)) { 
                                    there_is_hole = true; 
                                    break;
                                }
                                qu.push((right.0 + 1, right.1));
                                there_is_hole = true;
                                break;
                            }
                            right.1 += 1;
                            
                        }

                        if !there_is_hole {
                            if right.1 - 1 != x {
                                qu.push((right.0, right.1 - 1));
                            } if left.1 + 1 != x {
                                qu.push((left.0, left.1 + 1));
                            }
                        }
                    }
                }
            }
        }
       

    }
    
    // println!("{:?}", occupied_points.difference(&occupied_clone).collect::<BTreeSet<_>>());
    
    occupied_points.difference(&occupied_clone).collect::<HashSet<_>>().iter().filter(|e| e.0 >= min_y).count() as i64
}


