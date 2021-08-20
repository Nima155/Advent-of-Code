use lazy_static::lazy_static;
use regex::Regex;

pub fn four_digit_validator(val: &str, min: u32, max: u32) -> u8 {

    match u32::from_str_radix(val, 10) {
        Ok(n) if n >= min && n <= max => 1,
        _ => 0,
    }
}

const MAX_IN_CM: u32 = 193;
const MIN_IN_CM: u32 = 150;
const MAX_IN_IN: u32 = 76;
const MIN_IN_IN: u32 = 59;

pub fn height_validator(val: &str) -> u8 {
    match (val.ends_with("in"), val.ends_with("cm")) {
        (false, true) => match u32::from_str_radix(&val[..val.len() - 2], 10) {
            Ok(e) if e >= MIN_IN_CM && e <= MAX_IN_CM => 1,
            _ => 0,
        },
        (true, false) => match u32::from_str_radix(&val[..val.len() - 2], 10) {
            Ok(e) if  e >= MIN_IN_IN && e <= MAX_IN_IN => 1,
            _ => 0,
        },
        _ => 0,
    }
    
}
pub fn hair_color_validator(hcl: &str) -> u8 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    };
    RE.is_match(hcl) as u8
}

pub fn eye_color_validator(ecl: &str) -> u8 {
    match ecl {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => 1,
        _ => 0,
    }
}
pub fn pid_validator(pid: &str) -> u8 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    };
    
    RE.is_match(pid) as u8
}
