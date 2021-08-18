use std::fs;
fn main() {
    let passwords = fs::read_to_string("input.txt").unwrap();

    let disected_passwords = passwords.split("\r\n")
        .map(|e| e.split(" ").collect::<Vec<_>>()).collect::<Vec<_>>();
    

    disected_passwords.iter().filter(|e| {
            let range = e[0].split("-"); // range
    })

}
