use crate::common;

use itertools::Itertools;

fn combinations(source: &[i64]) -> Vec<(i64, i64)> {
    source
        .iter()
        .tuple_combinations()
        .map(|(&a, &b)| (a, b))
        .collect()
}

fn find_missing_combinations(input: &[i64], preamble: usize) -> Vec<i64> {
    let mut result = vec![];

    for i in preamble..input.len() - 1 {
        let current = input[i];

        if let Some((a, b)) = combinations(&input[i - preamble..i])
            .iter()
            .find(|(a, b)| a + b == current)
        {
            println!("{} is the sum of {} and {}", current, a, b);
        } else {
            println!("Cannot find tuple for {}", current);
            result.push(current)
        }
    }

    result
}

fn find_window_sum(input: &[i64], target: i64) -> Option<Vec<i64>> {
    for length in 2..input.len() {
        for s in input.windows(length) {
            if s.iter().fold(0, |acc, x| acc + x) == target {
                return Some(s.iter().map(|&x| x).collect::<_>());
            }
        }
    }

    None
}

pub fn test() {
    let input = [
        "35", "20", "15", "25", "47", "40", "62", "55", "65", "95", "102", "117", "150", "182",
        "127", "219", "299", "277", "309", "576",
    ];

    let numbers = input
        .iter()
        .map(|token| token.parse::<i64>().expect("Parse errro"))
        .collect::<Vec<i64>>();

    let missing = find_missing_combinations(&numbers, 5);
    assert_eq!(missing.len(), 1);
    assert_eq!(missing[0], 127);
}

pub fn part1() {
    let input = common::import("day9/input.txt")
        .iter()
        .map(|s| s.parse::<i64>().expect("Parse error"))
        .collect::<Vec<i64>>();

    let missing = find_missing_combinations(&input, 25usize);
    assert!(missing.len() > 0);
    println!("First missing sum in input : {}", missing[0]);
}

pub fn part2() {
    let input = common::import("day9/input.txt")
        .iter()
        .map(|s| s.parse::<i64>().expect("Parse error"))
        .collect::<Vec<i64>>();

    let missing = find_missing_combinations(&input, 25usize)[0];
    if let Some(mut window) = find_window_sum(&input, missing) {
        window.sort();

        println!(
            "Sum of smallest and largest in window : {}",
            window[0] + window[window.len() - 1]
        );
    } else {
        panic!("Owh noes!");
    }
}
