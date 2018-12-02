use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

fn main() {
    let raw_inputs = fs::read_to_string("input").unwrap();
    let mut freq = 0_i32;
    let mut seen = HashSet::new();
    for lines in raw_inputs.lines().cycle() {
        let diff = i32::from_str(lines).unwrap();
        seen.insert(freq);

        freq += diff;
        if seen.contains(&freq) {
            println!("{}", freq);
            return;
        }
    }
}
