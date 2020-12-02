use crate::common;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
struct Policy {
    min: i32,
    max: i32,
    token: char,
    input: String,
}

impl Policy {
    pub fn is_valid_part1(self: &Self) -> bool {
        let m = self.input.matches(self.token).count() as i32;
        self.min <= m && m <= self.max
    }

    pub fn is_valid_part2(self: &Self) -> bool {
        let l = self.input.chars().nth((self.min - 1) as usize).unwrap();
        let r = self.input.chars().nth((self.max - 1) as usize).unwrap();

        (l == self.token && r != self.token) || (l != self.token && r == self.token)
    }
}

#[derive(Debug)]
pub struct ParsePolicyError {}

impl FromStr for Policy {
    type Err = ParsePolicyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
        let m = re.captures(s).unwrap();

        let min = m.get(1).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
        let max = m.get(2).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
        let token = m.get(3).map_or("", |m| m.as_str());
        let input = m.get(4).map_or("", |m| m.as_str());

        Ok(Policy {
            min: min,
            max: max,
            token: token.to_string().chars().nth(0).unwrap(),
            input: input.to_string(),
        })
    }
}

#[test]
fn test() {
    let policies = common::import("day2/input.txt")
        .iter()
        .map(|v| Policy::from_str(v).unwrap())
        .collect::<Vec<Policy>>();

    for p in &policies {
        println!("{:?}", p);
    }
}

pub fn part1() {
    let policies = common::import("day2/input.txt")
        .iter()
        .map(|v| Policy::from_str(v).unwrap())
        .collect::<Vec<Policy>>();

    let valid = policies.iter().filter(|a| a.is_valid_part1()).count();

    println!("Valid passwords : {}", valid);
}

pub fn part2() {
    let policies = common::import("day2/input.txt")
        .iter()
        .map(|v| Policy::from_str(v).unwrap())
        .collect::<Vec<Policy>>();

    let valid = policies.iter().filter(|a| a.is_valid_part2()).count();

    println!("Valid passwords : {}", valid);
}
