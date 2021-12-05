use std::ops::{Deref, Index};

use crate::util;

pub fn part1() {
    let filename = "./input/day2.txt";
    let v = util::read_strings(filename);

    let mut horizontal = 0;
    let mut depth = 0;

    for s in v {
        let split: Vec<&str> = s.split(' ').collect();
        let command = split.index(0).deref();
        let unit: i32 = split.index(1).parse().unwrap();

        match command {
            "forward" => horizontal += unit,
            "down" => depth += unit,
            "up" => depth -= unit,
            _ => panic!("not a valid command"),
        }
    }

    let result = horizontal * depth;

    println!("day2part1: {}", result);
}

pub fn part2() {
    let filename = "./input/day2.txt";
    let v = util::read_strings(filename);

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for s in v {
        let split: Vec<&str> = s.split(' ').collect();
        let command = split.index(0).deref();
        let unit: i32 = split.index(1).parse().unwrap();

        match command {
            "forward" => {
                horizontal += unit;
                depth += aim * unit;
            }
            "down" => aim += unit,
            "up" => aim -= unit,
            _ => panic!("not a valid command"),
        }
    }

    let result = horizontal * depth;

    println!("day2part2: {}", result);
}
