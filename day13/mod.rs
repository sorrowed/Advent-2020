fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    i64::abs(a) / gcd(a, b) * i64::abs(b)
}

pub fn test() {
    let timestamp = 939;

    let mut bus_ids = "7,13,x,x,59,x,31,19"
        .split(",")
        .filter(|token| token != &"x")
        .map(|token| token.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    bus_ids.sort();

    let upper = bus_ids
        .iter()
        .map(|e| f64::ceil(timestamp as f64 / *e as f64) as i32 * *e - timestamp)
        .collect::<Vec<_>>();

    let minutes_to_wait = upper.iter().min().unwrap();
    let busline_id = bus_ids[upper.iter().position(|e| e == minutes_to_wait).unwrap()];
    println!(
        "Should wait for {} min for bus line {} -> {}",
        minutes_to_wait,
        busline_id,
        minutes_to_wait * busline_id
    );
}

pub fn part1() {
    let timestamp = 1000510;

    let mut bus_ids = "19,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,523,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,17,13,x,x,x,x,x,x,x,x,x,x,29,x,853,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,23"
        .split(",")
        .filter(|token| token != &"x")
        .map(|token| token.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    bus_ids.sort();

    let upper = bus_ids
        .iter()
        .map(|e| f64::ceil(timestamp as f64 / *e as f64) as i32 * *e - timestamp)
        .collect::<Vec<_>>();

    let minutes_to_wait = upper.iter().min().unwrap();
    let busline_id = bus_ids[upper.iter().position(|e| e == minutes_to_wait).unwrap()];
    println!(
        "Part 1, should wait for {} min for bus line {} -> {}",
        minutes_to_wait,
        busline_id,
        minutes_to_wait * busline_id
    );
}

pub fn part2() {
    let offsets_and_busids = "19,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,523,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,17,13,x,x,x,x,x,x,x,x,x,x,29,x,853,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,23"
        .split(",")
        .enumerate()
        .filter(|(_, busid)| busid != &"x")
        .map(|(offset, token)| (offset as i64, token.parse::<i64>().unwrap()))
        .collect::<Vec<_>>();

    let mut timestamp = 0i64;
    let mut step = offsets_and_busids[0].1;

    for (offset, busid) in offsets_and_busids {
        while (timestamp + offset) % busid != 0 {
            timestamp += step;
        }
        step = lcm(step, busid);
    }
    println!("Part 2 buses aligned at timestamp {}", timestamp);
}
