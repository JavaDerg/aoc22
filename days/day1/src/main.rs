use itertools::Itertools;

fn main() {
    let mut items = include_str!("../input.txt")
        .lines()
        .map(|x| x.trim())
        .scan(0, |mut g, n| {
            if n.is_empty() {
                *g += 1;
                Some(None)
            } else {
                Some(Some((*g, n.parse::<u64>().unwrap())))
            }
        })
        .filter_map(|x| x)
        .group_by(|(g, _)| *g)
        .into_iter()
        .map(|(_, g)| g.into_iter().map(|(_, n)| n).sum::<u64>())
        .sorted()
        .collect::<Vec<u64>>();

    println!("p1: {}", items.last().unwrap());
    println!("p2: {}", items.iter().rev().take(3).sum::<u64>());
}
