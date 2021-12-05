use std::ops::{Deref, Index};

use crate::util;

pub fn part1() {
    let filename = "./input/day3.txt";
    let v = util::read_strings(filename);

    let size = v.index(0).len();
    let mut state = vec![0; size];

    for bin in v {
        for i in 0..size {
            let digit = bin.chars().nth(i).unwrap();
            let mut state_val = *state.index(i);
            if digit == '1' {
                state_val += 1;
            } else {
                state_val -= 1;
            }
            std::mem::replace(&mut state[i], state_val);
        }
    }

    let mut gamma_vec: Vec<&str> = Vec::new();
    let mut epsilon_vec: Vec<&str> = Vec::new();

    for s in state {
        if s > 0 {
            gamma_vec.push("1");
            epsilon_vec.push("0");
        } else {
            gamma_vec.push("0");
            epsilon_vec.push("1");
        }
    }

    let gamma_str = gamma_vec.join("");
    let gamma = isize::from_str_radix(gamma_str.as_str(), 2).unwrap();
    let epsilon_str = epsilon_vec.join("");
    let epsilon = isize::from_str_radix(epsilon_str.as_str(), 2).unwrap();

    let result = gamma * epsilon;

    println!("day3part1: {}", result);
}

pub fn part2() {
    let filename = "./input/day3.txt";
    let mut oxygen_input = util::read_strings(filename);
    let mut co2_input = util::read_strings(filename);

    let size = oxygen_input.index(0).len();

    for i in 0..size {
        let mut state = 0;
        for binary_string in &oxygen_input {
            let digit = binary_string.chars().nth(i).unwrap();
            if digit == '1' {
                state += 1;
            } else {
                state -= 1;
            }
        }

        let mut new_input: Vec<String> = Vec::new();
        for binary_string in &oxygen_input {
            let digit = binary_string.chars().nth(i).unwrap();
            let mut should_digit = '1'; // oxygen gen, so prefer 1s
            if state < 0 {
                should_digit = '0';
            }
            if digit == should_digit {
                new_input.push(binary_string.to_string());
            }
        }

        oxygen_input = new_input;
        if oxygen_input.len() == 1 {
            break;
        }
    }

    for i in 0..size {
        let mut state = 0;
        for binary_string in &co2_input {
            let digit = binary_string.chars().nth(i).unwrap();
            if digit == '1' {
                state += 1;
            } else {
                state -= 1;
            }
        }

        let mut new_input: Vec<String> = Vec::new();
        for binary_string in &co2_input {
            let digit = binary_string.chars().nth(i).unwrap();
            let mut should_digit = '0'; // co2, so prefer 0
            if state < 0 {
                should_digit = '1';
            }
            if digit == should_digit {
                new_input.push(binary_string.to_string());
            }
        }

        co2_input = new_input;
        if co2_input.len() == 1 {
            break;
        }
    }

    let oxygen_str = oxygen_input.index(0);
    let oxygen = isize::from_str_radix(oxygen_str.as_str(), 2).unwrap();
    let co2_str = co2_input.index(0);
    let co2 = isize::from_str_radix(co2_str.as_str(), 2).unwrap();

    let result = oxygen * co2;

    println!("day3part2: {}", result);
}
