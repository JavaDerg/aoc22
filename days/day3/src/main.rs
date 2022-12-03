#![feature(hash_drain_filter)]

use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");

    let mut acc = 0;

    for (l1, l2, l3) in input.lines().tuples() {
        acc += prio(find_shared(&[l1, l2, l3]));
    }

    println!("{}", acc);
}


fn find_shared(input: &[&str]) -> char {
    let mut hm = HashMap::new();
    for str in input {
        for (k, _) in str.chars().counts() {
            *hm.entry(k).or_insert(0) += 1;
        }
    }

    hm.drain_filter(|k, v| *v < input.len());

    hm.into_iter().next().unwrap().0
}

fn prio(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u8 - b'a' + 1) as u32,
        'A'..='Z' => (c as u8 - b'A' + 27) as u32,
        _ => unimplemented!(),
    }
}
