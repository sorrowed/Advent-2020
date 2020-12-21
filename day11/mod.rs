use crate::common::*;
use std::collections::HashMap;
use std::{thread, time};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Tile {
    FLOOR,
    EMPTY,
    OCCUPIED,
}

impl Tile {
    fn new(c: char) -> Tile {
        match c {
            'L' => Tile::EMPTY,
            '.' => Tile::FLOOR,
            '#' => Tile::OCCUPIED,
            _ => panic!("Oopsie"),
        }
    }
}

type Tiles = HashMap<Vector, Tile>;

///
/// New one for me, a rust callback function
///
type Callback = for<'a> fn(&'a Tiles, &Vector) -> Vec<&'a Tile>;

struct Map {
    tiles: Tiles,
    br: Vector,
}

impl Map {
    fn new() -> Map {
        Map {
            tiles: HashMap::new(),
            br: Vector::new(0, 0, 0),
        }
    }

    fn parse(input: Vec<String>) -> Map {
        let mut result = Map::new();

        for (y, line) in input.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let tile = Tile::new(c);

                result.add(Vector::new(x as i64, y as i64, 0), tile);
            }
        }
        result
    }

    fn add(&mut self, location: Vector, tile: Tile) {
        self.br.x = i64::max(self.br.x, location.x);
        self.br.y = i64::max(self.br.y, location.y);

        self.tiles.insert(location, tile);
    }

    fn round<'a>(&'a mut self, tolerance: usize, s: Callback) {
        let mut tiles = Tiles::new();

        for (location, tile) in &self.tiles {
            let n = s(&self.tiles, location);

            if tile == &Tile::EMPTY && n.iter().all(|&t| t != &Tile::OCCUPIED) {
                tiles.insert(location.clone(), Tile::OCCUPIED);
            } else if tile == &Tile::OCCUPIED
                && n.iter().filter(|&&t| t == &Tile::OCCUPIED).count() >= tolerance
            {
                tiles.insert(location.clone(), Tile::EMPTY);
            } else {
                tiles.insert(location.clone(), tile.clone());
            }
        }

        self.tiles = tiles;
    }

    fn occupied(&self) -> usize {
        self.tiles
            .values()
            .filter(|&t| t == &Tile::OCCUPIED)
            .count()
    }

    fn print(&self) {
        for y in 0..=self.br.y {
            for x in 0..=self.br.x {
                print!(
                    "{}",
                    if let Some(tile) = self.tiles.get(&Vector::new(x, y, 0)) {
                        match tile {
                            Tile::EMPTY => 'L',
                            Tile::FLOOR => '.',
                            Tile::OCCUPIED => '#',
                            _ => '?',
                        }
                    } else {
                        '!'
                    }
                );
            }
            println!();
        }
    }
}

///
/// Slow, because it checks a lot of locations
///
fn chairs_adjacent<'a>(all: &'a Tiles, location: &Vector) -> Vec<&'a Tile> {
    all.iter()
        .filter(|(loc, _)| {
            loc != &location
                && loc.x >= location.x - 1
                && loc.x <= location.x + 1
                && loc.y >= location.y - 1
                && loc.y <= location.y + 1
        })
        .map(|(_, t)| t)
        .collect::<Vec<_>>()
}

fn chairs_in_los<'a>(all: &'a Tiles, location: &Vector) -> Vec<&'a Tile> {
    let mut result = vec![];

    // Look in all directions
    let directions = [
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    for (x, y) in directions.iter() {
        if let Some(n) = look_far(all, location, &Vector::new(*x, *y, 0)) {
            result.push(n);
        }
    }
    result
}

///
/// Recursively look for a spot that is not a floor in the specified direction
///
fn look_far<'a>(all: &'a Tiles, location: &Vector, direction: &Vector) -> Option<&'a Tile> {
    let mut result = None;

    let new_location = location.offset(direction);
    if let Some(tile) = all.get(&new_location) {
        if tile != &Tile::FLOOR {
            result = Some(tile);
        } else {
            result = look_far(all, &new_location, direction);
        }
    }
    result
}

pub fn test() {
    let input = [
        "L.LL.LL.LL",
        "LLLLLLL.LL",
        "L.L.L..L..",
        "LLLL.LL.LL",
        "L.LL.LL.LL",
        "L.LLLLL.LL",
        "..L.L.....",
        "LLLLLLLLLL",
        "L.LLLLLL.L",
        "L.LLLLL.LL",
    ];

    let mut map = Map::parse(input.iter().map(|s| s.to_string()).collect::<Vec<_>>());
    for _ in 0..10 {
        print!("\x1B[2J");
        map.print();
        println!();
        map.round(4, |all, loc| chairs_adjacent(all, loc));
        thread::sleep(time::Duration::from_millis(100));
    }
    println!("Map has {} occupied tiles", map.occupied());

    let mut map = Map::parse(input.iter().map(|s| s.to_string()).collect::<Vec<_>>());
    for _ in 0..10 {
        print!("\x1B[2J");
        map.print();
        println!();
        map.round(5, |all, loc| chairs_in_los(all, loc));
        thread::sleep(time::Duration::from_millis(100));
    }
    println!("Map has {} occupied tiles", map.occupied());
}

pub fn part1() {
    print!("\x1B[2J");
    let mut map = Map::parse(import("day11/input.txt"));
    for _ in 0..100 {
        print!("\x1B[2J");
        map.print();
        println!();
        map.round(4, |all, loc| chairs_adjacent(all, loc));
    }
    println!();
    println!("Map has {} occupied tiles", map.occupied());
}

pub fn part2() {
    print!("\x1B[2J");
    let mut map = Map::parse(import("day11/input.txt"));
    for _ in 0..100 {
        print!("\x1B[2J");
        map.print();
        println!();
        map.round(5, |all, loc| chairs_in_los(all, loc));
        thread::sleep(time::Duration::from_millis(100));
    }
    println!();
    println!("Map has {} occupied tiles", map.occupied());
}
