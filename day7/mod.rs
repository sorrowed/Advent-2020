use crate::common;
use regex::Regex;

#[derive(Clone)]
struct Bag {
    name: String,
    amount: i32,
}

impl std::fmt::Display for Bag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", self.amount, self.name)
    }
}

impl std::cmp::PartialEq for Bag {
    fn eq(&self, rhs: &Bag) -> bool {
        self.name == rhs.name
    }
}

impl Bag {
    pub fn new(name: &str, amount: i32) -> Bag {
        Bag {
            name: name.to_string(),
            amount: amount,
        }
    }
}

type Bags = Vec<(Bag, Vec<Bag>)>;

fn parse_bags(input: &Vec<String>) -> Bags {
    let mut result = vec![];

    let parent_re = Regex::new(r"(\w+ \w+) bags contain").unwrap();
    let child_re = Regex::new(r" (\d+) (\w+ \w+) bag").unwrap();
    for line in input.iter() {
        let name = parent_re.captures(line).unwrap().get(1).unwrap().as_str();

        let bags = child_re
            .captures_iter(line)
            .map(|cap| Bag::new(&cap[2], cap[1].parse::<i32>().unwrap()))
            .collect::<Vec<Bag>>();

        result.push((Bag::new(name, 1), bags));
    }
    result
}

fn print_bags(bags: &Bags) {
    print!("{{ ");
    for (k, v) in bags {
        print!("({} -->", k);
        for b in v {
            print!(" {}", b);
        }
        print!(") ");
    }
    println!(" }}");
}

fn test_part_1() {
    let input = [
        "light red bags contain 1 bright white bag, 2 muted yellow bags.",
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
        "bright white bags contain 1 shiny gold bag.",
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
        "faded blue bags contain no other bags.",
        "dotted black bags contain no other bags.",
    ];

    let bags = parse_bags(&input.iter().map(|s| s.to_string()).collect::<Vec<String>>());

    // Find any bags that can contain a shiny gold one
    let mut all = bags
        .iter()
        .filter(|(_, contains)| contains.iter().any(|bag| bag.name == "shiny gold"))
        .cloned()
        .collect::<Bags>();

    let mut current = all.clone();

    while current.len() > 0 {
        let new = bags
            .iter()
            .filter(|(_, contains)| {
                contains
                    .iter()
                    .any(|bag| current.iter().any(|(parent, _)| bag.name == parent.name))
            })
            .cloned()
            .collect::<Bags>();

        all.append(&mut new.clone());

        current = new;
    }

    println!("Day 7 test 1");
    print_bags(&all);
    assert_eq!(all.len(), 4);
}

fn test_part_2_1() {
    let input = [
        "light red bags contain 1 bright white bag, 2 muted yellow bags.",
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
        "bright white bags contain 1 shiny gold bag.",
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
        "faded blue bags contain no other bags.",
        "dotted black bags contain no other bags.",
    ];

    let bags = parse_bags(&input.iter().map(|s| s.to_string()).collect::<Vec<String>>());
    // Find the shiny gold bag
    let mut all = bags
        .iter()
        .filter(|(parent, _)| parent.name == "shiny gold")
        .cloned()
        .collect::<Bags>();

    let mut current = all.clone();
    print_bags(&current);
    while current.len() > 0 {
        let mut contained = bags
            .iter()
            .filter(|(parent, _)| {
                current
                    .iter()
                    .any(|(_, b)| b.iter().any(|bag| bag.name == parent.name))
            })
            .cloned()
            .collect::<Bags>();

        print_bags(&contained);
        for bag in current.iter() {
            for c in bag.1.iter() {
                if let Some(d) = contained.iter_mut().find(|i| i.0.name == c.name) {
                    println!("{}", d.0);
                    for e in d.1.iter_mut() {
                        e.amount *= c.amount;
                    }
                } else {
                    panic!("Help");
                }
            }
        }
        print_bags(&contained);
        all.append(&mut contained.clone());

        current = contained;
    }

    let total_bags = all
        .iter()
        .filter(|(_, bags)| bags.len() > 0)
        .fold(0, |acc, bag| {
            acc + bag.1.iter().fold(0, |acc, children| acc + children.amount)
        });

    assert_eq!(total_bags, 32);
}

pub fn test_part_2_2() {
    let input = [
        "shiny gold bags contain 2 dark red bags.",
        "dark red bags contain 2 dark orange bags.",
        "dark orange bags contain 2 dark yellow bags.",
        "dark yellow bags contain 2 dark green bags.",
        "dark green bags contain 2 dark blue bags.",
        "dark blue bags contain 2 dark violet bags.",
        "dark violet bags contain no other bags.",
    ];
    let bags = parse_bags(&input.iter().map(|s| s.to_string()).collect::<Vec<String>>());

    let mut all = bags
        .iter()
        .filter(|(parent, _)| parent.name == "shiny gold")
        .cloned()
        .collect::<Bags>();

    let mut current = all.clone();
    print_bags(&current);
    while current.len() > 0 {
        let mut contained = bags
            .iter()
            .filter(|(parent, _)| {
                current
                    .iter()
                    .any(|(_, b)| b.iter().any(|bag| bag.name == parent.name))
            })
            .cloned()
            .collect::<Bags>();

        print_bags(&contained);
        for bag in current.iter() {
            for c in bag.1.iter() {
                if let Some(d) = contained.iter_mut().find(|i| i.0.name == c.name) {
                    for e in d.1.iter_mut() {
                        e.amount *= c.amount;
                    }
                } else {
                    panic!("Help");
                }
            }
        }
        print_bags(&contained);
        all.append(&mut contained.clone());

        current = contained;
    }

    let total_bags = all
        .iter()
        .filter(|(_, bags)| bags.len() > 0)
        .fold(0, |acc, bag| {
            acc + bag.1.iter().fold(0, |acc, children| acc + children.amount)
        });

    assert_eq!(total_bags, 126);
}

//#[test]
pub fn test() {
    test_part_1();
    test_part_2_1();
    test_part_2_2();
}

pub fn part1() {
    let input = common::import("day7/input.txt");
    let bags = parse_bags(&input.iter().map(|s| s.to_string()).collect::<Vec<String>>());

    // Find any bags that can contain a shiny gold one
    let mut all = bags
        .iter()
        .filter(|(_, contains)| contains.iter().any(|bag| bag.name == "shiny gold"))
        .cloned()
        .collect::<Bags>();

    let mut current = all.clone();

    while current.len() > 0 {
        let containing = bags
            .iter()
            .filter(|(_, contains)| {
                contains
                    .iter()
                    .any(|bag| current.iter().any(|(parent, _)| bag.name == parent.name))
            })
            .cloned()
            .collect::<Bags>();

        all.append(&mut containing.clone());

        current = containing;
    }

    println!("Day 7 part 1");
    all.sort_by(|a, b| a.0.name.partial_cmp(&b.0.name).unwrap());
    all.dedup();
    println!(
        "Number of bags that may eventually contain a shiny gold one : {}",
        all.len()
    );
}

pub fn part2() {
    let input = common::import("day7/input.txt");
    let bags = parse_bags(&input.iter().map(|s| s.to_string()).collect::<Vec<String>>());

    let mut all = vec![];

    let mut current = bags
        .iter()
        .filter(|(parent, _)| parent.name == "shiny gold")
        .cloned()
        .collect::<Bags>();

    while current.len() > 0 {
        let mut contained = bags
            .iter()
            .filter(|(parent, _)| {
                current
                    .iter()
                    .any(|(_, children)| children.iter().any(|bag| bag.name == parent.name))
            })
            .cloned()
            .collect::<Bags>();

        for rule in current.iter() {
            for bag in rule.1.iter() {
                if let Some(contained_rule) = contained.iter_mut().find(|i| i.0.name == bag.name) {
                    for contained_bag in contained_rule.1.iter_mut() {
                        contained_bag.amount *= bag.amount;
                    }
                } else {
                    panic!("Help");
                }
            }
        }
        all.append(&mut current);

        current = contained;
    }

    let total_bags = all
        .iter()
        .filter(|(_, children)| children.len() > 0)
        .fold(0, |acc, bag| {
            acc + bag.1.iter().fold(0, |acc, children| acc + children.amount)
        });

    println!(
        "Number of bags that a shiny gold one must contain: {}",
        total_bags
    );
}
