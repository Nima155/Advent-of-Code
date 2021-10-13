use std::fs;
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();

    let mut parsed = Vec::new();
    for l in lines.split("\r\n") {
        let line_to_vec = l.split('@').collect::<Vec<_>>();
        let dimensions = line_to_vec[1].split(':').collect::<Vec<_>>();
        let (starting, dims) = (
            dimensions[0].trim().split(',').collect::<Vec<_>>(),
            dimensions[1].trim().split('x').collect::<Vec<_>>(),
        );
        
        
            parsed.push(
                (
                (starting[0].parse::<usize>().unwrap(),
                starting[1].parse::<usize>().unwrap()), (
                    dims[0].parse::<usize>().unwrap(),
                    dims[1].parse::<usize>().unwrap(),
                ))
            );
            
        
    }
    
    // print!("{} hi {}", 1, 2);
    println!("{}", brute_force_and_find_common(&parsed));
}


fn brute_force_and_find_common(parsed: &Vec<((usize, usize), (usize, usize))>) -> usize {
    let mut all_points = vec![vec![0; 1000]; 1000];
    
    for ((s_y, s_x), (c_y, c_x)) in parsed {
        for i in 0..*c_y {
            for j in 0..*c_x {
                all_points[i + s_y][j + s_x] += 1;
            }
        }
    };

    all_points.iter().flatten().filter(|c| **c >= 2).count()
}