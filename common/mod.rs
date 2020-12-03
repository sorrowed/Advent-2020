use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Vector {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vector {
    pub fn new(x: i64, y: i64, z: i64) -> Vector {
        Vector { x: x, y: y, z: z }
    }
}
pub fn import(name: &str) -> Vec<String> {
    let file = File::open(name).unwrap();
    let reader = BufReader::new(file);
    let mut vec = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        vec.push(line.unwrap());
    }
    vec
}
