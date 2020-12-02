use crate::common;

fn find_tuple_2(a: &Vec<i64>) -> (i64, i64) {
    for f in 0..a.len() {
        for s in f..a.len() {
            if a[f] + a[s] == 2020 {
                return (a[f], a[s]);
            }
        }
    }
    panic!("Help!")
}

fn find_tuple_3(a: &Vec<i64>) -> (i64, i64, i64) {
    for f in 0..a.len() {
        for s in f..a.len() {
            for t in s..a.len() {
                if a[f] + a[s] + a[t] == 2020 {
                    return (a[f], a[s], a[t]);
                }
            }
        }
    }

    panic!("Help!")
}

pub fn part1() {
    let m = find_tuple_2(&common::import("day1/input.txt").iter().map(|a|a.parse::<i64>().unwrap()).collect());
    println!("{} * {} = {}", m.0, m.1, m.0 * m.1);
}

pub fn part2() {
    let m = find_tuple_3(&common::import("day1/input.txt").iter().map(|a|a.parse().unwrap()).collect());
    println!("{} * {} * {} = {}", m.0, m.1, m.2, m.0 * m.1 * m.2);
}
