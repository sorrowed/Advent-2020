use crate::common::*;
use text_io::scan;

use std::collections::{HashMap, HashSet};

fn apply_value_mask(value: u64, set_mask: u64, clr_mask: u64) -> u64 {
    (value | set_mask) & (!clr_mask)
}

fn apply_address_mask(address: u64, set_mask: u64, float_mask: u64) -> HashSet<u64> {
    let mut result = HashSet::<u64>::new();

    result.insert(address | set_mask);
    let mut mask = 0x1u64;
    while mask != 0 {
        if float_mask & mask != 0 {
            let mut copy = HashSet::<u64>::new();

            for address in result {
                copy.insert(address | mask);
                copy.insert(address & !mask);
            }
            result = copy;
        }
        mask <<= 1;
    }

    result
}

fn extract_mask(input: &str, m: char) -> u64 {
    input
        .chars()
        .rev()
        .enumerate()
        .map(|(pos, c)| if c == m { 1u64 << pos } else { 0 })
        .fold(0, |mask, v| mask | v)
}

fn extract_masks(input: &str, left: char, right: char) -> (u64, u64) {
    (extract_mask(input, left), extract_mask(input, right))
}

pub fn test() {
    let masks = extract_masks("1XXXX0X", '1', '0');

    assert_eq!(apply_value_mask(11, masks.0, masks.1), 73);

    assert_eq!(apply_value_mask(101, masks.0, masks.1), 101);
    assert_eq!(apply_value_mask(0, masks.0, masks.1), 64);

    let mut registers = HashMap::<u64, u64>::new();

    for input in &[
        ("000000000000000000000000000000X1001X", 42, 100),
        ("00000000000000000000000000000000X0XX", 26, 1),
    ] {
        let set_mask = extract_mask(input.0, '1');
        let float_mask = extract_mask(input.0, 'X');

        let addresses = apply_address_mask(input.1, set_mask, float_mask);
        for v in addresses {
            registers.insert(v, input.2);
        }
    }
    assert_eq!(registers.values().fold(0, |acc, v| acc + v), 208);
}

pub fn part1() {
    let mut registers = HashMap::<u64, u64>::new();
    let instructions = import("day14/input.txt");

    let mut masks = (0u64, 0u64);

    for line in instructions {
        let mut tokens = line.split_ascii_whitespace();
        match tokens.next() {
            Some("mask") => {
                tokens.next();
                masks = extract_masks(tokens.next().unwrap(), '1', '0');
            }
            mem => {
                let register_index: u64;

                scan!(mem.unwrap().bytes()=>"mem[{}]",register_index);
                tokens.next();
                let value = tokens.next().unwrap().parse::<u64>().unwrap();

                registers.insert(register_index, apply_value_mask(value, masks.0, masks.1));
            }
        }
    }

    println!(
        "The sum for part 1 is {}",
        registers.values().fold(0, |acc, v| acc + v)
    );
}

pub fn part2() {
    let mut registers = HashMap::<u64, u64>::new();
    let instructions = import("day14/input.txt");

    let mut masks = (0u64, 0u64);

    for line in instructions {
        let mut tokens = line.split_ascii_whitespace();
        match tokens.next() {
            Some("mask") => {
                tokens.next();
                masks = extract_masks(tokens.next().unwrap(), '1', 'X');
            }
            mem => {
                let register_index: u64;
                scan!(mem.unwrap().bytes()=>"mem[{}]",register_index);
                tokens.next();
                let value = tokens.next().unwrap().parse::<u64>().unwrap();

                let addresses = apply_address_mask(register_index, masks.0, masks.1);
                for address in addresses {
                    registers.insert(address, value);
                }
            }
        }
    }

    println!(
        "The sum for part 2 is {}",
        registers.values().fold(0, |acc, v| acc + v)
    );
}
