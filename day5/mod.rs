use crate::common;

fn divide_and_conquer(specification: &str, lower: char, upper: char) -> i32 {
    let mut l = 0;
    let mut u = (1 << specification.len() as i32) - 1;

    for c in specification.chars() {
        let d = (u - l + 1) / 2;

        if c == lower {
            u -= d;
        } else if c == upper {
            l += d;
        } else {
            panic!("Help!");
        }
    }

    assert_eq!(l, u);
    l
}

fn seat_id(specification: &str) -> (i32, i32, i32) {
    let rid = divide_and_conquer(&specification[0..7], 'F', 'B');
    let cid = divide_and_conquer(&specification[7..10], 'L', 'R');

    (rid, cid, rid * 8 + cid)
}

//#[test]
pub fn test() {
    let r = seat_id("FBFBBFFRLR");
    assert!(r.0 == 44 && r.1 == 5 && r.2 == 357);

    let r = seat_id("BFFFBBFRRR");
    assert!(r.0 == 70 && r.1 == 7 && r.2 == 567);

    let r = seat_id("FFFBBBFRRR");
    assert!(r.0 == 14 && r.1 == 7 && r.2 == 119);

    let r = seat_id("BBFFBBFRLL");
    assert!(r.0 == 102 && r.1 == 4 && r.2 == 820);
}

pub fn part1() {
    let highest = common::import("day5/input.txt")
        .iter()
        .map(|p| seat_id(p))
        .max()
        .unwrap();
    println!(
        "Highest seat id {} (R:{} C:{})",
        highest.2, highest.0, highest.1
    );
}

pub fn part2() {
    let passes = common::import("day5/input.txt")
        .iter()
        .map(|p| seat_id(p).2)
        .collect::<Vec<i32>>();

    for row in 1..=126 {
        for column in 0..=7 {
            let id = row * 8 + column;

            // Empirical lower limit
            if id > 39 && id <= 801 && !passes.contains(&id) {
                println!("Seat id missing {}", id);
            }
        }
    }
}
