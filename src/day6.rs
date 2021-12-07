use std::{cell::Cell, fs};

pub fn part1() {
    let filename = "./input/day6.txt";
    let mut fish: Vec<Cell<u8>> = fs::read_to_string(filename)
        .unwrap()
        .trim()
        .split(',')
        .map(|s| {
            let n: u8 = s.parse().unwrap();
            Cell::new(n)
        })
        .collect();

    for _ in 0..80 {
        let mut spawned = Vec::new();
        for f in &fish {
            let mut countdown = f.get();
            if countdown == 0 {
                countdown = 6;
                spawned.push(Cell::new(8u8));
            } else {
                countdown = countdown - 1;
            }
            f.set(countdown);
        }
        fish.append(&mut spawned);
    }

    let result = fish.len();

    println!("day6part1: {}", result);
}

pub fn part2() {
    let filename = "./input/day6.txt";

    let result = simulate(filename, 256);

    println!("day6part2: {}", result);
}

fn simulate(filename: &str, days: i32) -> u64 {
    let mut age_counts = [0u64; 9];
    fs::read_to_string(filename)
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .for_each(|age| age_counts[age] += 1);

    for _ in 0..256 {
        age_counts.rotate_left(1);
        age_counts[6] += age_counts[8];
    }

    let result: u64 = age_counts.iter().sum();
    result
}
