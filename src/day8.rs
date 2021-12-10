use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::Index,
    str::Chars,
};

use itertools::Itertools;

pub fn part1(input: &String) -> i64 {
    let result: usize = input
        .lines()
        .map(|line| {
            line.split(" | ")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|digit| match digit.len() {
                    2 | 4 | 3 | 7 => 1, // ie, the unique lengths of 1, 4, 7, 8
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum();

    result as i64
}

pub fn part2(input: &String) -> i64 {
    for line in input.lines() {
        let mut split = line.split(" | ");

        let mut observed: Vec<String> = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.chars().sorted().collect::<String>())
            .collect();
        observed.sort_by(|a, b| {
            if a.len() < b.len() {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        // FIXME: What you are about to witness is some witchcraft.
        let mut map = HashMap::new();
        map.insert(observed.index(0), '1'); // length 2 is "1"
        map.insert(observed.index(1), '7'); // length 3 is "7"
        map.insert(observed.index(2), '4'); // length 4 is "4"
        map.insert(observed.index(9), '8'); // length 7 is "8"
        let mut remaining_fives = Vec::new();
        'outer: for i in 3..=5 {
            for c in observed.index(0).chars() {
                if !observed.index(i).contains(c) {
                    continue 'outer;
                }
            }
            map.insert(observed.index(i), '3');
            for j in 3..=5 {
                if j != i {
                    remaining_fives.push(observed.index(j));
                }
            }
        }
        let mut remaining_sixes = Vec::new();
        let mut six_index = 0;
        for i in 6..=8 {
            for c in observed.index(0).chars() {
                if !observed.index(i).contains(c) {
                    map.insert(observed.index(i), '6');
                    six_index = i;
                    break;
                } else {
                    remaining_sixes.push(observed.index(i));
                }
            }
        }
        for rem in &remaining_sixes {
            for c in observed.index(2).chars() {
                if !rem.contains(c) {
                    map.insert(rem, '8');
                }
            }
        }
        for rem in &remaining_sixes {
            if !map.contains_key(rem) {
                map.insert(rem, '9');
            }
        }
        for rem in &remaining_fives {
            for c in a_not_b(observed.index(9), observed.index(six_index)).chars() {
                if !rem.contains(c) {
                    map.insert(rem, '5');
                }
            }
        }
        for rem in &remaining_fives {
            if !map.contains_key(rem) {
                map.insert(rem, '2');
            }
        }

        let output: Vec<char> = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.chars().sorted().collect::<String>())
            .map(|s| *map.get(&s).unwrap())
            .collect();

        let o: String = output.into_iter().collect();
    }

    println!("{:?}", "foo");
    -1
}

// If there is exactly one matching element, remove and return it. Else panic.
fn remove_only<T, F>(input: &mut Vec<T>, predicate: F) -> T
where
    T: Clone,
    F: Fn(&&T) -> bool + Copy,
{
    let mut results = input.iter().filter(predicate);
    let result = results.next().expect("no element found").clone();
    assert!(results.next().is_none(), "multiple elements found");

    // Vec::drain_filter would be useful here, but don't want to depend on nighly.
    input.retain(|x| !predicate(&x));

    result
}

fn decode(input: &mut Vec<HashSet<char>>) -> [HashSet<char>; 10] {
    // Easy cases.
    let n1 = remove_only(input, |x| x.len() == 2);
    let n4 = remove_only(input, |x| x.len() == 4);
    let n7 = remove_only(input, |x| x.len() == 3);
    let n8 = remove_only(input, |x| x.len() == 7);

    // 3 is the only 5-segment digit that shares 2 segments with digit 1.
    let n3 = remove_only(input, |x| x.len() == 5 && (*x & &n1).len() == 2);
    let n2 = remove_only(input, |x| x.len() == 5 && (*x & &n4).len() == 2);
    // 5 is the only remaining 5-segment digit.
    let n5 = remove_only(input, |x| x.len() == 5);

    // And so on.
    let n6 = remove_only(input, |x| x.len() == 6 && (*x & &n1).len() == 1);
    let n9 = remove_only(input, |x| x.len() == 6 && (*x & &n4).len() == 4);
    let n0 = remove_only(input, |x| x.len() == 6);

    assert!(input.is_empty());

    [n0, n1, n2, n3, n4, n5, n6, n7, n8, n9]
}
