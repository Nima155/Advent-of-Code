use std::fs;

#[derive(Eq, PartialEq, Debug)]
enum SearchMode {
    Free,
    Bunches,
    Lengths,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum OperatorMode {
    Sum,
    Product,
    Minimum,
    Maximum,
    GT,
    LT,
    Equal,
}

#[derive(Eq, PartialEq, Debug)]
enum LengthMode {
    Bunches(usize),
    Lengths(usize),
    Free,
}

fn main() {
    let input_str = fs::read_to_string("./test.txt").unwrap();

    let bin_rep = input_str
        .chars()
        .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
        .collect::<String>();

    println!(
        "{}",
        main_driver(&bin_rep, &mut 0, LengthMode::Free, OperatorMode::Equal)
    );
}

fn main_driver(bin_rep: &str, pp: &mut usize, mut _lim: LengthMode, typ: OperatorMode) -> i64 {
    let mut lngth = bin_rep.len();

    if let LengthMode::Lengths(n) = _lim {
        lngth = n;
    }

    let mut contained_values = vec![];

    let mut last_op_mode = OperatorMode::Sum;
    let mut search_mode = SearchMode::Free;
    // println!("{:?} {:?} {:?} {}", _lim, contained_values, typ, pp);
    while *pp < lngth {
        // println!("{:?} {:?} {:?} {}", _lim, contained_values, typ, pp);

        match search_mode {
            SearchMode::Lengths if *pp + 15 <= lngth => {
                let tots_length = usize::from_str_radix(&bin_rep[*pp..*pp + 15], 2).unwrap();
                // println!("{}", &bin_rep[*pp..*pp + 15]);

                *pp += 15;
                contained_values.push(main_driver(
                    bin_rep,
                    pp,
                    LengthMode::Lengths(tots_length + *pp),
                    last_op_mode,
                ));
                search_mode = SearchMode::Free;
            }
            SearchMode::Bunches if *pp + 11 <= lngth => {
                let groups = usize::from_str_radix(&bin_rep[*pp..*pp + 11], 2).unwrap();
                // println!("{:?} {:?}", last_op_mode, _lim);
                *pp += 11;
                contained_values.push(main_driver(
                    bin_rep,
                    pp,
                    LengthMode::Bunches(groups),
                    last_op_mode,
                ));
                search_mode = SearchMode::Free;
            }
            SearchMode::Free if *pp + 3 + 3 < lngth => {
                if let LengthMode::Bunches(ref mut n) = _lim {
                    if *n > 0 {
                        *n -= 1;
                    } else {
                        break;
                    }
                }
                let type_id = &bin_rep[*pp + 3..*pp + 3 + 3];
                match type_id {
                    "100" => {
                        *pp += 6;
                        contained_values.push(literal_group(pp, bin_rep));
                    }
                    _ => {
                        match type_id {
                            "000" => last_op_mode = OperatorMode::Sum,
                            "001" => last_op_mode = OperatorMode::Product,
                            "010" => last_op_mode = OperatorMode::Minimum,
                            "011" => last_op_mode = OperatorMode::Maximum,
                            "101" => last_op_mode = OperatorMode::GT,
                            "110" => last_op_mode = OperatorMode::LT,
                            "111" => last_op_mode = OperatorMode::Equal,
                            _ => {}
                        }

                        search_mode = if &bin_rep[*pp + 3 + 3..*pp + 3 + 4] == "1" {
                            SearchMode::Bunches
                        } else {
                            SearchMode::Lengths
                        };

                        *pp += 7;
                    }
                }
            }
            _ => {
                *pp += 1;
            }
        }
        // println!("{:?} {:?} {:?} {}", _lim, contained_values, typ, pp);
    }

    type_cycler(&contained_values, typ)
}

fn literal_group(pointer: &mut usize, bin_rep: &str) -> i64 {
    //TODO: fix slight bug here!
    let mut bin = String::new();
    while &bin_rep[*pointer..*pointer + 1] == "1" {
        bin.push_str(&bin_rep[*pointer + 1..*pointer + 5]);
        *pointer += 5;
    }
    bin.push_str(&bin_rep[*pointer + 1..*pointer + 5]);
    *pointer += 5;

    i64::from_str_radix(&bin, 2).unwrap()
}

fn type_cycler(contained_values: &[i64], typ: OperatorMode) -> i64 {
    match typ {
        OperatorMode::Sum => contained_values.iter().sum(),
        OperatorMode::Product => contained_values.iter().product(),
        OperatorMode::Minimum => *contained_values.iter().min().unwrap(),
        OperatorMode::Maximum => *contained_values.iter().max().unwrap(),
        OperatorMode::GT if contained_values.len() > 1 => {
            (contained_values[0] > contained_values[1]) as i64
        }
        OperatorMode::LT if contained_values.len() > 1 => {
            (contained_values[0] < contained_values[1]) as i64
        }
        OperatorMode::Equal if contained_values.len() > 1 => {
            (contained_values[0] == contained_values[1]) as i64
        }
        _ => contained_values[0],
    }
}
