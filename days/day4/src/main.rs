use nom::branch::alt;
use nom::character::complete::{char, digit1, line_ending};
use nom::combinator::{eof, map};
use nom::multi::many0;
use nom::IResult;
use std::ops::Range;

fn main() {
    let n = parse_file(include_str!("../input.txt"))
        .unwrap()
        .1
        .into_iter()
        .map(|(a, b)| overlaps(a, b))
        .filter(|x| *x)
        .count();

    println!("{}", n);
}

fn overlaps(a: Range<u32>, b: Range<u32>) -> bool {
    let (a, al) = bits(a);
    let (b, bl) = bits(b);

    let r = a & b;

    r.count_ones() == al || r.count_ones() == bl
}

fn bits(r: Range<u32>) -> (u128, u32) {
    let len = r.end - r.start + 1;
    let mask = u128::MAX >> (128 - len);
    (mask << r.start, len)
}

fn parse_file(i: &str) -> IResult<&str, Vec<(Range<u32>, Range<u32>)>> {
    many0(parse_line)(i)
}

fn parse_line(i: &str) -> IResult<&str, (Range<u32>, Range<u32>)> {
    let (i, a) = parse_range(i)?;
    let (i, _) = char(',')(i)?;
    let (i, b) = parse_range(i)?;
    let (i, _) = alt((line_ending, eof))(i)?;
    Ok((i, (a, b)))
}

fn parse_range(i: &str) -> IResult<&str, Range<u32>> {
    let (i, s) = map(map(digit1, str::parse::<u32>), Result::unwrap)(i)?;
    let (i, _) = char('-')(i)?;
    let (i, e) = map(map(digit1, str::parse::<u32>), Result::unwrap)(i)?;

    Ok((i, s..e))
}
