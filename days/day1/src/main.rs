fn main() {
    let mut iter = include_str!("../input.txt")
        .lines()
        .map(|x| x.trim());

    let mut max = 0;
    while let x @ (1..) = total_cal_rec(&mut iter) {
        max = max.max(x);
    }

    println!("{max}");
}

fn total_cal_rec<'a>(mut i: impl Iterator<Item = &'a str>) -> u64 {
    if let Some(n) = i.next() {
        if n.is_empty() {
            0
        } else {
            n.parse::<u64>().unwrap()
                + total_cal_rec(i)
        }
    } else {
        0
    }
}