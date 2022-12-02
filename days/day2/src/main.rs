use nom::branch::alt;
use nom::character::complete::{line_ending, one_of, space1};
use nom::combinator::{eof, map};
use nom::multi::many0;
use nom::IResult;
use std::cmp::Ordering;
use RpsLdw::*;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum RpsLdw {
    RL,
    PD,
    SW,
}

fn main() {
    let i = include_str!("../input.txt");
    let v = parse_file(i).unwrap().1;

    let score1 = v
        .iter()
        .cloned()
        .map(|(o, s)| win_score(s.cmp(&o)) + s.score())
        .sum::<u64>();
    let score2 = v
        .into_iter()
        .map(|(o, s)| (o, wl_tranform(o, s)))
        .map(|(o, s)| win_score(s.cmp(&o)) + s.score())
        .sum::<u64>();

    println!("{}", score1);
    println!("{}", score2);
}

fn win_score(ord: Ordering) -> u64 {
    match ord {
        Ordering::Less => 0,
        Ordering::Equal => 3,
        Ordering::Greater => 6,
    }
}

fn wl_tranform(op: RpsLdw, ldw: RpsLdw) -> RpsLdw {
    match (op, ldw) {
        (x, PD) => x,

        (RL, SW) => PD,
        (RL, RL) => SW,

        (PD, SW) => SW,
        (PD, RL) => RL,

        (SW, SW) => RL,
        (SW, RL) => PD,
    }
}

fn parse_file(i: &str) -> IResult<&str, Vec<(RpsLdw, RpsLdw)>> {
    many0(parse_line)(i)
}

fn parse_line(i: &str) -> IResult<&str, (RpsLdw, RpsLdw)> {
    let (i, f) = parse_rps(i)?;
    let (i, _) = space1(i)?;
    let (i, s) = parse_rps(i)?;
    let (i, _) = alt((line_ending, eof))(i)?;

    Ok((i, (f, s)))
}

fn parse_rps(i: &str) -> IResult<&str, RpsLdw> {
    alt((
        map(one_of("AX"), |_| RL),
        map(one_of("BY"), |_| PD),
        map(one_of("CZ"), |_| SW),
    ))(i)
}

impl RpsLdw {
    pub fn score(&self) -> u64 {
        match self {
            RL => 1,
            PD => 2,
            SW => 3,
        }
    }
}

impl Ord for RpsLdw {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for RpsLdw {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (PD, RL) => Ordering::Greater,
            (RL, PD) => Ordering::Less,

            (SW, PD) => Ordering::Greater,
            (PD, SW) => Ordering::Less,

            (RL, SW) => Ordering::Greater,
            (SW, RL) => Ordering::Less,

            _ => Ordering::Equal,
        })
    }
}
