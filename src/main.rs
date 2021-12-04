mod util;

use std::ops::Index;

fn main() {
    day1prob1();
    day1prob2();
}

fn day1prob1() {
    let filename = "./input/day1.txt";
    let v = util::read_ints(filename);

    let mut count = 0;
    for i in 1..v.len() {
        if v.index(i) > v.index(i - 1) {
            count += 1;
        }
    }

    println!("day1prob1: {}", count);
}

fn day1prob2() {
    let filename = "./input/day1.txt";
    let v = util::read_ints(filename);

    let mut count = 0;
    for i in 3..v.len() {
        let sum_a = v.index(i - 3) + v.index(i - 2) + v.index(i - 1);
        let sum_b = v.index(i - 2) + v.index(i - 1) + v.index(i);
        if sum_b > sum_a {
            count += 1;
        }
    }

    println!("day1prob2: {}", count);
}
