use crate::common;
use itertools::*;

struct Chain {
    adapters: Vec<i64>,
}

impl Chain {
    pub fn new(input: &Vec<i64>) -> Chain {
        let mut result = Chain {
            adapters: input.clone(),
        };
        result.chain();
        result
    }

    fn chain(&mut self) {
        self.adapters.push(0);
        self.adapters.sort();
        self.adapters
            .push(self.adapters[self.adapters.len() - 1] + 3);
    }

    fn distribution(&self, d: i64) -> usize {
        self.adapters
            .windows(2)
            .filter(|e| e[1] - e[0] == d)
            .count()
    }

    pub fn chains(&self) -> i64 {
        // Extract length of groups of '1's and use that to determine how many
        // branches that group introduces. Differences of 3 do not introduce extra branches as
        // all chains must go 'through' these adapters. The total number of possibilities is the product
        // of those that these groups of ones add
        self.adapters
            .windows(2)
            .map(|w| w[1] - w[0])   // Collect differences between adapter joltages
            .group_by(|&difference| difference == 1)  // Assuming differences are only 1 or three, group them
            .into_iter()
            .filter(|(is_one_group, _)| *is_one_group)    // Only interested in the *length* of the 1 difference groups, thus groups where d == 1 is true
            .map(|(_, group)| group.count())
            .fold(1, |acc, length| {
                acc * match length {
                    1 => 1, // No exta branches (enclosed by jumps of three)
                    2 => 2, // Sequence of 2 differences of 1 jolt add 2 branches
                    3 => 4, // Sequence of 3 differences of 1 jolt add 4 branches
                    4 => 7, // etc
                    5 => 13,
                    _ => panic!("Oh noes!"),
                }
            })
    }
}

pub fn test() {
    for input in &[
        vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4],
        vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ],
    ] {
        let chain = Chain::new(input);
        let one_jolt = chain.distribution(1);
        let three_jolts = chain.distribution(3);

        println!("1 jolts : {} three jolts: {}", one_jolt, three_jolts);

        println!("Chains : {}", chain.chains());

        // Collect differences
        let diffs = input.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();

        // Only differences of 1 and 3 are in there (i hope)
        assert!(diffs.iter().all(|&d| d == 1 || d == 3));
    }
}

pub fn part1() {
    let input = common::import("day10/input.txt")
        .iter()
        .map(|s| s.parse::<i64>().expect("Parse error"))
        .collect::<Vec<i64>>();

    let chain = Chain::new(&input);
    let one_jolt = chain.distribution(1);
    let three_jolts = chain.distribution(3);
    println!(
        "1 jolts : {} three jolts: {} --> {}",
        one_jolt,
        three_jolts,
        one_jolt * three_jolts
    );
}

pub fn part2() {
    let input = common::import("day10/input.txt")
        .iter()
        .map(|s| s.parse::<i64>().expect("Parse error"))
        .collect::<Vec<i64>>();

    let chain = Chain::new(&input);

    println!("Chains : {}", chain.chains());
}
