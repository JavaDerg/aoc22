use nom::branch::alt;
use nom::character::complete::{line_ending, one_of, space1};
use nom::combinator::{eof, map};
use nom::multi::many0;
use nom::IResult;
use std::cmp::Ordering;
use Rps::*;

#[derive(Eq, PartialEq, Debug)]
pub enum Rps {
    Rock,
    Paper,
    Scissors,
}

fn main() {
    let i = include_str!("../input.txt");
    let v = parse_file(i).unwrap().1;

    let score = v
        .into_iter()
        .map(|(o, s)| win_score(s.cmp(&o)) + s.score())
        .sum::<u64>();

    println!("{}", score);
}

fn win_score(ord: Ordering) -> u64 {
    match ord {
        Ordering::Less => 0,
        Ordering::Equal => 3,
        Ordering::Greater => 6,
    }
}

fn parse_file(i: &str) -> IResult<&str, Vec<(Rps, Rps)>> {
    many0(parse_line)(i)
}

fn parse_line(i: &str) -> IResult<&str, (Rps, Rps)> {
    let (i, f) = parse_rps(i)?;
    let (i, _) = space1(i)?;
    let (i, s) = parse_rps(i)?;
    let (i, _) = alt((line_ending, eof))(i)?;

    Ok((i, (f, s)))
}

fn parse_rps(i: &str) -> IResult<&str, Rps> {
    alt((
        map(one_of("AX"), |_| Rock),
        map(one_of("BY"), |_| Paper),
        map(one_of("CZ"), |_| Scissors),
    ))(i)
}

impl Rps {
    pub fn score(&self) -> u64 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

impl Ord for Rps {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Rps {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (Paper, Rock) => Ordering::Greater,
            (Rock, Paper) => Ordering::Less,

            (Scissors, Paper) => Ordering::Greater,
            (Paper, Scissors) => Ordering::Less,

            (Rock, Scissors) => Ordering::Greater,
            (Scissors, Rock) => Ordering::Less,

            _ => Ordering::Equal,
        })
    }
}
