use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char as char_, digit1, line_ending, space0, space1};
use nom::combinator::{map, opt};
use nom::multi::{count, many0, many1};
use nom::sequence::{delimited, pair, terminated, tuple};
use nom::IResult;

fn main() {
    let input = include_str!("../input.txt");

    let (mut s1, is) = parse_file(input).unwrap().1;
    let mut s2 = s1.clone();

    for ((f, t), c) in is.iter().cloned() {
        for _ in 0..c {
            let cr = s1[f as usize].pop().unwrap();
            s1[t as usize].push(cr);
        }
    }
    for ((f, t), c) in is.iter().cloned() {
        let o = s2[f as usize].len() - c;
        for _ in 0..c {
            let cr = s2[f as usize].remove(o);
            s2[t as usize].push(cr);
        }
    }

    for i in 0..s1.len() {
        if let Some(c) = s1[i].pop() {
            print!("{}", c);
        }
    }
    println!();

    for i in 0..s2.len() {
        if let Some(c) = s2[i].pop() {
            print!("{}", c);
        }
    }
    println!();
}

fn parse_file(i: &str) -> IResult<&str, (Vec<Vec<char>>, Vec<Inst>)> {
    let (i, s) = parse_stack(i)?;
    let (i, _) = line_ending(i)?;
    let (i, is) = parse_instructions(i)?;

    Ok((i, (s, is)))
}

fn parse_num(i: &str) -> IResult<&str, u32> {
    map(map(digit1, str::parse::<u32>), Result::unwrap)(i)
}

type Inst = ((u32, u32), usize);

fn parse_instructions(i: &str) -> IResult<&str, Vec<Inst>> {
    many1(terminated(parse_instruction, opt(line_ending)))(i)
}

fn parse_instruction(i: &str) -> IResult<&str, Inst> {
    let (i, _) = tag("move ")(i)?;
    let (i, c) = parse_num(i)?;
    let (i, _) = tag(" from ")(i)?;
    let (i, f) = parse_num(i)?;
    let (i, _) = tag(" to ")(i)?;
    let (i, t) = parse_num(i)?;

    Ok((i, ((f - 1, t - 1), c as usize)))
}

fn parse_stack(i: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (i, mut rows) = many1(parse_row)(i)?;
    let (i, cnt) = parse_row_index(i)?;

    rows.reverse();

    let mut acc: Vec<Vec<char>> = (0..cnt).map(|_| Vec::new()).collect();
    for row in rows {
        for (i, c) in row
            .into_iter()
            .enumerate()
            .filter_map(|(i, c)| c.map(|c| (i, c)))
        {
            acc[i].push(c);
        }
    }

    Ok((i, acc))
}

fn parse_row(i: &str) -> IResult<&str, Vec<Option<char>>> {
    let (i, acc) = many0(terminated(parse_crate, opt(char_(' '))))(i)?;
    let (i, _) = line_ending(i)?;

    Ok((i, acc))
}

fn parse_row_index(i: &str) -> IResult<&str, usize> {
    terminated(
        map(many1(tuple((opt(space1), digit1, opt(space1)))), |x| x.len()),
        line_ending,
    )(i)
}

fn parse_crate(i: &str) -> IResult<&str, Option<char>> {
    alt((
        map(delimited(char_('['), anychar, char_(']')), Some),
        map(tag("   "), |_| None),
    ))(i)
}
