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

impl Bag {
    pub fn new(name: &str, amount: i32) -> Bag {
        Bag {
            name: name.to_string(),
            amount: amount,
        }
    }
}

type Bags = Vec<(String, Vec<Bag>)>;

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

        result.push((name.to_string(), bags));
    }
    result
}

fn print_bags(bags: &Bags) {
    for (k, v) in bags {
        print!("{} -->", k);
        for b in v {
            print!(" {}", b);
        }
        println!("");
    }
}

//#[test]
pub fn test() {
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

    // Find any bags that can contain a shimy gold one
    let mut current = bags
        .iter()
        .filter(|(_, v)| v.iter().any(|s| s.name == "shiny gold"))
        .cloned()
        .collect::<Bags>();

    let mut new = bags
        .iter()
        .filter(|(_, v)| {
            v.iter().any(|s| {
                current
                    .iter()
                    .any(|(_, v)| v.iter().any(|b| b.name == s.name))
            })
        })
        .cloned()
        .collect::<Bags>();
        
    while new.len() != current.len() {
        current = new;
        new = bags
            .iter()
            .filter(|(_, v)| {
                v.iter().any(|s| {
                    current
                        .iter()
                        .any(|(_, v)| v.iter().any(|b| b.name == s.name))
                })
            })
            .cloned()
            .collect::<Bags>();
    }

    println!("");
    print_bags(&new);
}

pub fn part1() {}

pub fn part2() {}
