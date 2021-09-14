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

pub fn determine_final_image(cake: &[Vec<Vec<u32>>]) {
    let mut final_image = cake[0].clone();

    for j in 0..cake[0].len() {
        for i in 0..cake[0][0].len() {
            for m in 0..cake.len() {
                match cake[m][j][i] {
                    1 | 0 => {
                        final_image[j][i] = cake[m][j][i];
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
    
    let f = final_image
            .iter()
            .map(|f|  
                String::from_iter(f.iter().map(|f| char::from_digit(*f, 10).unwrap()))
            )
            .collect::<Vec<_>>();    

    println!(
        "{:?}",
        f
    );
}
