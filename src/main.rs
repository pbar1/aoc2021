mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

use std::{fs, ops::Index};

use stopwatch::Stopwatch;

fn main() {
    let parts: Vec<(fn(&String) -> i64, fn(&String) -> i64)> = vec![
        (day1::part1, day1::part2),
        (day2::part1, day2::part2),
        (day3::part1, day3::part2),
        (day4::part1, day4::part2),
        (day5::part1, day5::part2),
        (day6::part1, day6::part2),
        (day7::part1, day7::part2),
    ];

    for i in 0..parts.len() {
        let (part1, part2) = *parts.index(i);

        let filename = format!("./input/day{}.txt", i + 1);
        let input = fs::read_to_string(filename).unwrap();

        let mut sw1 = Stopwatch::start_new();
        let part1_result = part1(&input);
        let part1_elapsed = sw1.elapsed();
        println!(
            "Day {}, Part 1: {}   ({:?})",
            i + 1,
            part1_result,
            part1_elapsed
        );

        let mut sw2 = Stopwatch::start_new();
        let part2_result = part2(&input);
        let part2_elapsed = sw2.elapsed();
        println!(
            "Day {}, Part 2: {}   ({:?})",
            i + 1,
            part2_result,
            part2_elapsed
        );
    }
}
