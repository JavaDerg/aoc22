#![feature(hash_drain_filter)]

use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");

    let mut acc = 0;

    for l in input.lines() {
        let s = l.len() / 2;

        let (d1, d2) = (
                &l[..s],
                &l[s..],
            );

        let mut hm = HashMap::new();
        for (k, _) in d1.chars().counts() {
            hm.insert(k, 1);
        }
        for (k, _) in d2.chars().counts() {
            *hm.entry(k).or_insert(0) += 1;
        }

        hm.drain_filter(|k, v| *v == 1);

        let shared = hm.iter().next().unwrap().0;

        acc += prio(*shared);
    }

    println!("{}", acc);
}

fn prio(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u8 - b'a' + 1) as u32,
        'A'..='Z' => (c as u8 - b'A' + 27) as u32,
        _ => unimplemented!(),
    }
}
