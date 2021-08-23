use self::runner::run_through;

pub mod runner;


pub fn interpreter(vec: Vec<Vec<&str>>) -> i32 {
    
    for (i, v) in vec.iter().enumerate() {
        if v[0] == "nop" || v[0] == "jmp" {
            let mut deep_copy = vec.clone();
            deep_copy[i][0] = if v[0] == "nop" {"jmp"} else {"nop"};
            let (status, ans1) = run_through(deep_copy);
            if let true = status {
                return ans1;
            }
        }
    };
    0
}