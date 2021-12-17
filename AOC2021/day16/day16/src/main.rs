use std::fs;

#[derive(Eq, PartialEq)]
enum SearchMode {
    Free,
    Bunches,
    Lengths,
}

#[derive(Eq, PartialEq, Debug)]
enum LengthMode {
    Bunches(usize),
    Lengths(usize),
    Free,
}

fn main() {
    let input_str = fs::read_to_string("../test.txt").unwrap();

    let bin_rep = input_str
        .chars()
        .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
        .collect::<String>();

    

    println!("{}", main_driver(&bin_rep, &mut 0, LengthMode::Free));
}

fn main_driver(bin_rep: &str, pp: &mut usize, _lim: LengthMode) -> i32 {
    let lngth = bin_rep.len();
    println!("{:?}", _lim);
    let (mut ans, mut search_mode) = (0, SearchMode::Free);

    while *pp < lngth {
            
            match search_mode {
                SearchMode::Lengths if *pp + 15 <= lngth => {
                    let tots_length = usize::from_str_radix(&bin_rep[*pp..*pp + 15], 2).unwrap();
                    println!("{}", &bin_rep[*pp..*pp + 15]);
                    *pp += 15;
                    ans += main_driver(bin_rep, pp, LengthMode::Lengths(tots_length));
                    
                }
                SearchMode::Bunches if *pp + 11 <= lngth  => {
                    let groups = usize::from_str_radix(&bin_rep[*pp..*pp + 11], 2).unwrap();
                    // println!("{}", &bin_rep[*pp..*pp + 11]);
                    *pp += 11;
                    ans += main_driver(bin_rep, pp, LengthMode::Bunches(groups));
                }
                SearchMode::Free if *pp + 3 < lngth => {
                    // println!("{}", i32::from_str_radix(&bin_rep[*pp..*pp + 3], 2).unwrap());
                    ans += i32::from_str_radix(&bin_rep[*pp..*pp + 3], 2).unwrap();
                    
                    match &bin_rep[*pp + 3..*pp + 3 + 3] {
                        "100" => {
                            
                            *pp += 6;
                            literal_group(pp, &bin_rep);
                            
                        }
                        _ => {
                            
                            search_mode = if &bin_rep[*pp + 3 + 3..*pp + 3 + 4] == "1" {
                                SearchMode::Bunches
                            } else {
                                SearchMode::Lengths
                            };
                            *pp += 7;
                            println!("pp: {}", pp);
                        }
                    }
                    
                }
                _ => { *pp += 1;}
            }
        
    
    }
    ans
}

fn literal_group(pointer: &mut usize, bin_rep: &str) {

    //TODO: fix slight bug here!
    
    while &bin_rep[*pointer..*pointer + 1] == "1" {
        println!("{}", &bin_rep[*pointer..*pointer + 1]);
        *pointer += 5;
    }
    *pointer += 5;

    
    
}
