use std::fs;
use std::collections::HashMap; 


fn main() {
    let mut triple_counter = 0_i32;
    let mut double_counter = 0_i32;
    let input = fs::read_to_string("input").unwrap();
    for word in input.lines() {
        let mut hash = HashMap::new();
        
        for chars in word.chars() {
            let count = hash.entry(chars).or_insert(0);
            *count += 1;
        }

        if hash.values().any(|&x| x == 2 ) {
            double_counter += 1;
        }
        
        if hash.values().any(|&x| x == 3) {
            triple_counter += 1; 
        }
    }
    println!("checksum: {}", triple_counter * double_counter);
    
    // part II 
    for (i, potato) in input.lines().enumerate(){
        for mut  x in input.lines().skip(i).map(|x| x.to_string()){
            let actual: Vec<usize> = x.chars().zip(potato.chars())
                .enumerate()
                .filter(|(_, (x, y))| x != y)
                .map(|(x, (_, _))| x )
                .take(2)
                .collect();
            if actual.len() == 1 {
                x.remove(actual[0]);
                println!("{}", x);
            }
        }
    }
}
