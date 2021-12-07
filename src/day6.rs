use std::fs;

pub fn part1() {
    let filename = "./input/day6.txt";

    let result = simulate(filename, 80);

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

    for _ in 0..days {
        age_counts.rotate_left(1);
        age_counts[6] += age_counts[8];
    }

    let result: u64 = age_counts.iter().sum();
    result
}
