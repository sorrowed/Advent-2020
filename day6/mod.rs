use crate::common;
use std::collections::HashSet;

struct Group {
    any: HashSet<char>,
    all: Vec<char>,
}

impl Group {
    pub fn new() -> Group {
        Group {
            any: HashSet::new(),
            all: Vec::new(),
        }
    }

    pub fn add_any(&mut self, c: char) {
        self.any.insert(c);
    }
    
    pub fn add_all(&mut self, c: char) {
        self.all.push(c);
    }
}

fn parse_group(input: &[String]) -> Group {
    let mut result = Group::new();

    for traveler in input {
        for c in traveler.chars() {
            result.add_any(c);
        }
    }

    for q in [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ]
    .iter()
    {
        if input.iter().all(|t| t.chars().find(|c| c == q) != None) {
            result.add_all(*q);
        }
    }

    result
}

fn parse_groups(input: &Vec<String>) -> Vec<Group> {
    input
        .split(|s| s == "")
        .map(|s| parse_group(s))
        .collect::<Vec<Group>>()
}

//#[test]
pub fn test() {}

pub fn part1() {
    let groups = parse_groups(&common::import("day6/input.txt"));
    let yes = groups.iter().fold(0, |acc, g| acc + g.any.len());

    println!(
        "Total number of questions to which anyone answered yes : {}",
        yes
    );
}

pub fn part2() {
    let groups = parse_groups(&common::import("day6/input.txt"));
    let yes = groups.iter().fold(0, |acc, g| acc + g.all.len());

    println!(
        "Total number of questions to which all answered yes : {}",
        yes
    );
}
