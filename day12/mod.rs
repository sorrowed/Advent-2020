use crate::common::*;

enum Action {
    Move(char, i32),
}

#[derive(Debug)]
enum Heading {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl Heading {
    fn turn_left(&mut self, mut angle: i32) {
        while angle != 0 {
            match &self {
                Heading::NORTH => *self = Heading::WEST,
                Heading::WEST => *self = Heading::SOUTH,
                Heading::SOUTH => *self = Heading::EAST,
                Heading::EAST => *self = Heading::NORTH,
            }
            angle -= 90;
        }
    }

    fn turn_right(&mut self, mut angle: i32) {
        while angle != 0 {
            match &self {
                Heading::NORTH => *self = Heading::EAST,
                Heading::EAST => *self = Heading::SOUTH,
                Heading::SOUTH => *self = Heading::WEST,
                Heading::WEST => *self = Heading::NORTH,
            }
            angle -= 90;
        }
    }
}

struct Ferry {
    location: Vector,
    heading: Heading,
    waypoint: Vector,
}

fn rotate_around_left(center: &Vector, location: &Vector, mut angle: i32) -> Vector {
    let mut d = location.offset(&Vector::new(-center.x, -center.y, -center.z));

    while angle != 0 {
        angle -= 90;

        let x = d.x;
        d.x = -d.y;
        d.y = x;
    }
    center.offset(&d)
}

fn rotate_around_right(center: &Vector, location: &Vector, mut angle: i32) -> Vector {
    let mut d = location.offset(&Vector::new(-center.x, -center.y, -center.z));

    while angle != 0 {
        angle -= 90;

        let x = d.x;
        d.x = d.y;
        d.y = -x;
    }
    center.offset(&d)
}

impl Ferry {
    fn new() -> Ferry {
        Ferry {
            location: Vector::new(0, 0, 0),
            heading: Heading::EAST,
            waypoint: Vector::new(10, 1, 0),
        }
    }

    fn move_absolute(&mut self, action: &Action) {
        match action {
            Action::Move(c, n) => match c {
                'N' => self.location.y += *n as i64,
                'S' => self.location.y -= *n as i64,
                'E' => self.location.x += *n as i64,
                'W' => self.location.x -= *n as i64,
                'L' => self.heading.turn_left(*n),
                'R' => self.heading.turn_right(*n),
                'F' => match self.heading {
                    Heading::NORTH => self.move_absolute(&Action::Move('N', *n)),
                    Heading::EAST => self.move_absolute(&Action::Move('E', *n)),
                    Heading::SOUTH => self.move_absolute(&Action::Move('S', *n)),
                    Heading::WEST => self.move_absolute(&Action::Move('W', *n)),
                },
                _ => panic!("Unsupported move!"),
            },
        }
    }

    fn move_relative(&mut self, action: &Action) {
        match action {
            Action::Move(c, n) => match c {
                'N' => self.waypoint.y += *n as i64,
                'S' => self.waypoint.y -= *n as i64,
                'E' => self.waypoint.x += *n as i64,
                'W' => self.waypoint.x -= *n as i64,
                'L' => self.waypoint = rotate_around_left(&self.location, &self.waypoint, *n),
                'R' => self.waypoint = rotate_around_right(&self.location, &self.waypoint, *n),

                'F' => {
                    let d = self.waypoint.offset(&Vector::new(
                        -self.location.x,
                        -self.location.y,
                        -self.location.z,
                    ));

                    for _ in 0..*n {
                        self.location = self.location.offset(&d);
                    }
                    self.waypoint = self.location.offset(&d);
                }
                _ => panic!("Unsupported move!"),
            },
        }
    }

    fn print(&self) {
        println!("{:?} --> {:?}", self.location, self.heading);
    }
}

pub fn test() {
    let instructions = vec!["F10", "N3", "F7", "R90", "F11"]
        .iter()
        .map(|s| {
            let tokens = s.split_at(1);
            Action::Move(
                tokens.0.chars().nth(0).unwrap(),
                tokens.1.parse::<i32>().expect("Parse error"),
            )
        })
        .collect::<Vec<_>>();

    let mut ferry = Ferry::new();
    for action in instructions.iter() {
        ferry.move_absolute(action);
    }

    println!(
        "Ferry's manhattan distance : {}",
        ferry.location.manhattan(&Vector::new(0, 0, 0))
    );

    let mut ferry = Ferry::new();
    for action in instructions {
        ferry.move_relative(&action);
    }

    println!(
        "Ferry's manhattan distance : {}",
        ferry.location.manhattan(&Vector::new(0, 0, 0))
    );
}

pub fn part1() {
    let instructions = import("day12/input.txt")
        .iter()
        .map(|s| {
            let tokens = s.split_at(1);
            Action::Move(
                tokens.0.chars().nth(0).unwrap(),
                tokens.1.parse::<i32>().expect("Parse error"),
            )
        })
        .collect::<Vec<_>>();

    let mut ferry = Ferry::new();
    for action in instructions {
        ferry.move_absolute(&action);
    }

    println!(
        "Part 1 Ferry's manhattan distance : {}",
        ferry.location.manhattan(&Vector::new(0, 0, 0))
    );
}

pub fn part2() {
    let instructions = import("day12/input.txt")
        .iter()
        .map(|s| {
            let tokens = s.split_at(1);
            Action::Move(
                tokens.0.chars().nth(0).unwrap(),
                tokens.1.parse::<i32>().expect("Parse error"),
            )
        })
        .collect::<Vec<_>>();

    let mut ferry = Ferry::new();
    for action in instructions {
        ferry.move_relative(&action);
    }

    println!(
        "Part 2 Ferry's manhattan distance : {}",
        ferry.location.manhattan(&Vector::new(0, 0, 0))
    );
}
