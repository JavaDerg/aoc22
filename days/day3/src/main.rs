#![feature(hash_drain_filter)]

use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    let p1 = input
        .lines()
        .map(|l| {
            let s = l.len() / 2;
            [&l[..s], &l[s..]]
        })
        .map(|p| find_shared(&p[..]))
        .map(prio)
        .sum::<u32>();

    let p2 = input
        .lines()
        .tuples()
        .map(|(l1, l2, l3)| [l1, l2, l3])
        .map(|p| find_shared(&p[..]))
        .map(prio)
        .sum::<u32>();

    println!("p1: {}", p1);
    println!("p2: {}", p2);
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
