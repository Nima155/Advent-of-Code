fn main() {
    // println!("{:?}", part1);
    run_program();
}

fn run_program() {
    let [mut v0, ip, v2, mut v3, v4, v5] = [0i128, 1, 23550 * 14 * 32 + 1003, 0, 0, 0];

    // we reduce all the input down into this program... which basically gives us the sum of all factors of the number in register 2
    while v3 <= v2 {
        // optimized step... o(n) instead of o(n^2)
        if v3 != 0 && (v2 % v3) == 0 {
            v0 += v3;
        }
        v3 += 1;
        // ip += 1;
    }
    println!("[{}, {}, {}, {}, {}, {}]", v0, ip, v2, v3, v4, v5);

    // initial program
    // while ip <= 35 {
    //     match ip {
    //         0 => { ip += 16; }
    //         1 => { v3 = 1; }
    //         2 => { v5 = 1; }
    //         3 => { v4 = v3 * v5 }
    //         4 => { v4 = (v2 == v4) as i64 }
    //         5 => { ip += v4   }
    //         6 => { ip += 1 }
    //         7 => { v0 += v3 }
    //         8 => { v5 += 1 }
    //         9 => { v4 = (v5 > v2) as i64 }
    //         10 => { ip += v4  }
    //         11 => { ip = 2 }
    //         12 => { v3 += 1  }
    //         13 => { v4 = (v3 > v2) as i64 }
    //         14 => { ip += v4 }
    //         15 => { ip = 1 }
    //         16 => { ip *= ip }

    //         // 17 => { v2 += 2 }
    //         // 18 => { v2 *= 2 }
    //         // 19 => { v2 *= ip }
    //         // 20 => { v2 *= 11 }
    //         // 21 => { v4 += 7 }
    //         // 22 => { v4 *= ip }
    //         // 23 => { v4 += 13 }
    //         // 24 => { v2 += v4 }
    //         // 25 => { ip += 1 }
    //         26 => { ip = 0 }
    //         // 27 => { v4 = ip }
    //         // 28 => { v4 *= ip }
    //         // 29 => { v4 += ip }
    //         // 30 => { v4 *= ip }
    //         // 31 => { v4 *= 14 }
    //         // 32 => { v4 *= ip }
    //         // 33 => { v2 += v4 }
    //         // 34 => { v0 = 0 }
    //         // 35 => { ip = 0 }
    //         _ => {}
    //     }
    //     ip += 1;
    // }
}
