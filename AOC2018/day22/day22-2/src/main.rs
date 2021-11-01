use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

// depth: 7305
// target: 13,734
// The region at 0,0 (the mouth of the cave) has a geologic index of 0.
// The region at the coordinates of the target has a geologic index of 0.
// If the region's Y coordinate is 0, the geologic index is its X coordinate times 16807.
// If the region's X coordinate is 0, the geologic index is its Y coordinate times 48271.
// Otherwise, the region's geologic index is the result of multiplying the erosion levels of the regions at X-1,Y and X,Y-1.

// A region's erosion level is its geologic index plus the cave system's depth, all modulo 20183. Then: % 3
fn main() {
    let cave_mapping = calculate_erosion_levels(734, 13, 7305);
    shortest_path_to_target(734, 13, &cave_mapping);
}

const MODULO_CONST: i128 = 20183;
const TYPE_MODULO: i128 = 3;

// In rocky regions, you can use the climbing gear or the torch. You cannot use neither (you'll likely slip and fall).
// In wet regions, you can use the climbing gear or neither tool. You cannot use the torch (if it gets wet, you won't have a light source).
// In narrow regions, you can use the torch or neither tool. You cannot use the climbing gear (it's too bulky to fit).
#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Hash, Clone, Copy)]
enum GearLoad {
    Neither,
    ClimbingGear,
    Torch,
}

fn calculate_erosion_levels(height: usize, width: usize, depth: usize) -> Vec<Vec<i128>> {
    let mut ans = vec![vec![0i128; width + 733]; height + 733];
    // 10378 too high
    for i in 0..height + 733 {
        for j in 0..width + 733 {
            let mut geo_indx = if i == 0 {
                j as i128 * 16807
            } else if j == 0 {
                i as i128 * 48271
            } else {
                ans[i][j - 1] * ans[i - 1][j]
            };

            if i == height && j == width {
                geo_indx = 0;
            }

            ans[i][j] = (geo_indx + depth as i128) % MODULO_CONST;
        }
    }

    ans
}

const MOVES: [[i64; 2]; 4] = [[1, 0], [0, 1], [-1, 0], [0, -1]];

fn shortest_path_to_target(height: usize, width: usize, mappings: &[Vec<i128>]) {
    use GearLoad::Torch;

    let mut queue = BinaryHeap::new();
    let mut visited: HashMap<(i64, i64, GearLoad), usize> = HashMap::new();
    queue.push(Reverse((0, 0, 0, Torch)));
    visited.insert((0, 0, Torch), 0);

    while !queue.is_empty() {
        let Reverse((minutes, y, x, equipped_item)) = queue.pop().unwrap();

        try_all_tools(
            Reverse((minutes, y, x, equipped_item)),
            &mut visited,
            &mut queue,
            mappings[y][x] % TYPE_MODULO,
        );

        // println!("{} {} {} {:?}", y, x, minutes, equipped_item);
        if y == height && x == width && equipped_item == Torch {
            println!("{}", minutes);
            break;
        }
        // 995 too low
        for [yy, xx] in MOVES {
            let (ny, nx) = (y as i64 + yy, x as i64 + xx);
            if ny >= 0 && nx >= 0 {
                let is_compatible = check_compatibility(
                    mappings[ny as usize][nx as usize] % TYPE_MODULO,
                    equipped_item,
                );
                let minutes = (if !is_compatible { 7 } else { 0 }) + minutes;

                if is_compatible
                    && (!visited.contains_key(&(ny, nx, equipped_item))
                        || *visited.get_key_value(&(ny, nx, equipped_item)).unwrap().1
                            > (minutes + 1))
                {
                    visited.insert((ny, nx, equipped_item), minutes + 1);
                    queue.push(Reverse((
                        minutes + 1,
                        ny as usize,
                        nx as usize,
                        equipped_item,
                    )));
                }
            }
        }
    }
}

fn check_compatibility(type_: i128, current_type: GearLoad) -> bool {
    use GearLoad::{ClimbingGear, Neither, Torch};

    match current_type {
        ClimbingGear => type_ == 0 || type_ == 1,
        Torch => type_ == 2 || type_ == 0,
        Neither => type_ == 1 || type_ == 2,
        _ => true,
    }
}

fn try_all_tools(
    cream_on_top: Reverse<(usize, usize, usize, GearLoad)>,
    visited: &mut HashMap<(i64, i64, GearLoad), usize>,
    heap: &mut BinaryHeap<Reverse<(usize, usize, usize, GearLoad)>>,
    cur_type: i128,
) {
    use GearLoad::{ClimbingGear, Neither, Torch};
    let Reverse((minutes, y, x, _)) = cream_on_top;

    let mut friends = vec![];

    match cur_type {
        0 => friends = vec![ClimbingGear, Torch],
        1 => friends = vec![ClimbingGear, Neither],
        2 => friends = vec![Torch, Neither],
        _ => {}
    }

    for it in friends {
        let y = y as i64;
        let x = x as i64;
        if !visited.contains_key(&(y, x, it))
            || *visited.get_key_value(&(y, x, it)).unwrap().1 > (minutes + 7)
        {
            visited.insert((y, x, it), minutes + 7);
            heap.push(Reverse((minutes + 7, y as usize, x as usize, it)));
        }
    }
}
