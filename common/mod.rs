use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn import(name: &str) -> Vec<String> {
    let file = File::open(name).unwrap();
    let reader = BufReader::new(file);
    let mut vec = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        vec.push(line.unwrap());
    }
    
    vec
}
