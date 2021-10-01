use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

const MOVES: [[i16; 2]; 4] = [[0, 1], [1, 0], [-1, 0], [0, -1]];

type Point = (usize, usize);

pub fn find_portals(grid: &[Vec<char>]) -> (HashMap<Point, Point>, Point, Point) {
    let mut ans = HashMap::new();

    let (mut end_point, mut start_point) = ((0, 0), (0, 0));

    for (j, row) in grid.iter().enumerate() {
        for (i, c) in row.iter().enumerate() {
            if *c == '.' {
                for [ny, nx] in MOVES {
                    let (nyy, nxx) = (j as i16 + ny, i as i16 + nx);
                    if nyy >= 0
                        && nyy < grid.len() as i16
                        && nxx >= 0
                        && nxx < grid[nyy as usize].len() as i16
                        && grid[nyy as usize][nxx as usize].is_ascii_uppercase()
                    {
                        let mut d = [
                            grid[nyy as usize][nxx as usize],
                            grid[(nyy + ny) as usize][(nxx + nx) as usize],
                        ];
                        d.sort_unstable();

                        match d {
                            ['A', 'A'] => {
                                start_point = (j, i);
                            }
                            ['Z', 'Z'] => {
                                end_point = (j, i);
                            }
                            _ => {
                                let entry = ans.entry(d).or_insert([(0, 0); 2]);
                                entry[(entry[0] != (0, 0)) as usize] = (j, i);
                            }
                        }
                    }
                }
            }
        }
    }

    (
        ans.iter()
            .map(|(_, [v1, v2])| [(*v1, *v2), (*v2, *v1)])
            .flatten()
            .collect::<HashMap<(usize, usize), (usize, usize)>>(),
        end_point,
        start_point,
    )
}

pub fn dijkstra(
    portals: &HashMap<Point, Point>,
    starting_point: Point,
    end_point: Point,
    grid: &[Vec<char>],
) -> usize {
    let ans = 0;

    let mut heap = BinaryHeap::new();

    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    // println!("{} {:?}", grid.len(), starting_point);

    heap.push(Reverse((starting_point, 0)));

    while !heap.is_empty() {
        let Reverse(((y, x), dist)) = heap.pop().unwrap();

        if (y, x) == end_point {
            return dist;
        }

        for [ny, nx] in MOVES {
            let (mut ny, mut nx) = ((y as i16 + ny) as usize, (x as i16 + nx) as usize);
            let mut dist = dist;
            if let Some((yy, xx)) = portals.get(&(ny, nx)) {
                ny = *yy;
                nx = *xx;
                dist += 1;
            }

            if (!visited.contains_key(&(ny, nx)) || *visited.get(&(ny, nx)).unwrap() > dist + 1)
                && grid[ny][nx] == '.'
            {
                visited.insert((ny, nx), dist + 1);
                heap.push(Reverse(((ny, nx), dist + 1)));
            }
        }
    }

    ans
}
