pub fn layer_cake(width: u64, height: u64, input: &[u32]) -> Vec<Vec<Vec<u32>>> {
    let mut j = 0;

    let mut ans = Vec::new();
    while j < input.len() {
        let mut intermediate_vec = Vec::with_capacity(height as usize);

        for _ in 0..height {
            intermediate_vec.push(input[j..usize::min(input.len(), j + width as usize)].to_vec());
            j += width as usize;
        }
        ans.push(intermediate_vec);
    }
    ans
}

pub fn find_0_layer(layers: &[Vec<Vec<u32>>]) -> usize {
    layers.iter().fold((usize::MAX, 0), |(cur_zeroes, ans), cur| {
        let (zeroes, ones, twos) = cur.iter()
        .fold((0, 0, 0), |(zs, os, ts), row| 
        (zs + row.iter().filter(|c| **c == 0).count(), os + row.iter().filter(|c| **c == 1).count(), ts + row.iter().filter(|c| **c == 2).count()));
        if cur_zeroes > zeroes { (zeroes, ones * twos) } else { (cur_zeroes, ans) }
    }).1
}
