use crate::common;

use common::Vector;

type ForestType = Vec<Vec<char>>;

fn parse_forest_line(line: &str) -> Vec<char> {
    line.chars().collect()
}

fn parse_forest(input: &Vec<String>) -> ForestType {
    input
        .iter()
        .map(|l| parse_forest_line(l))
        .collect::<ForestType>()
}

fn traverse_forest(forest: &ForestType, pattern: &Vector) -> i64 {
    let mut p = Vector::new(0, 0, 0);

    let mut trees = 0;
    while (p.y as usize) < forest.len() {
        let l = &forest[p.y as usize];
        if (l[p.x as usize % l.len()]) == '#' {
            trees += 1;
        }
        p.x += pattern.x;
        p.y += pattern.y;
    }
    trees
}

#[test]
pub fn test() {
    let forest = parse_forest(&common::import("day3/test.txt"));

    for l in &forest {
        for c in l {
            print!("{}", c);
        }
        println!();
    }

    let p1 = traverse_forest(&forest, &Vector::new(3, 1, 0));
    println!("Number of trees {}", p1);
    assert_eq!(p1, 7);

    let patterns = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let p2 = patterns.iter().fold(1, |acc, &c| {
        acc * traverse_forest(&forest, &Vector::new(c.0, c.1, 0))
    });

    println!("Number of trees {}", p2);
    assert_eq!(p2, 336);
}

pub fn part1() {
    let forest = parse_forest(&common::import("day3/input.txt"));

    println!(
        "Number of trees {}",
        traverse_forest(&forest, &Vector::new(3, 1, 0))
    );
}

pub fn part2() {
    let forest = parse_forest(&common::import("day3/input.txt"));

    let patterns = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    println!(
        "Number of trees {}",
        patterns.iter().fold(1, |acc, &c| acc
            * traverse_forest(&forest, &Vector::new(c.0, c.1, 0)))
    );
}
