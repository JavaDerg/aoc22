use std::collections::BTreeMap;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_till};
use nom::character::complete::{char, digit1, line_ending};
use nom::combinator::{eof, map};
use nom::{IResult};
use nom::multi::many0;

fn main() {
    let input = include_str!("../input.txt");

    let cmds = parse_file(input).unwrap().1;

    let mut tree = BTreeMap::new();
    tree.insert("/".to_string(), None);
    let mut at = vec![];

    for cmd in cmds {
        match cmd {
            Cmd::Cd(ref cd) if cd == "." => continue,
            Cmd::Cd(ref cd) if cd == ".." => drop(at.pop()),
            Cmd::Cd(ref cd) if cd == "/" => at.clear(),
            Cmd::Cd(cd) => at.push(cd),
            Cmd::Ls(lsm) =>
                for (k, v) in lsm {
                    let mut p = at.clone();
                    p.push(k.clone());

                    match v {
                        None => {
                            let mut path = pathify(&p);
                            path.push('/');
                            tree.insert(path, None);
                        }
                        Some(size) => {
                            tree.insert(pathify(&p), Some(size));
                        }
                    }
                },
        }
    }

    let required = 40_000_000u64;
    let a_size = tree.iter().filter_map(|(_, v)| *v).sum::<u64>();

    println!("{}", a_size);

    let mut smallest_fit = ("/".to_string(), a_size);

    for (k, _) in tree.iter().filter(|(_, v)| v.is_none()) {
        let size = tree.range(k.clone()..=format!("{k}{}", char::MAX)).filter_map(|(_, x)| *x).sum::<u64>();
        // println!("{k:50}\t{}\t{}", a_size + size, required - size);
        if size < smallest_fit.1 && a_size - size <= required {
            smallest_fit = (k.clone(), size);
            println!("{:?}", smallest_fit);
        }
    }

    println!("{smallest_fit:?}");
}

fn pathify(v: &[String]) -> String {
    let mut buf = String::from("/");
    for e in v {
        buf.push_str(e);
        buf.push('/');
    }
    if buf.len() > 1 {
        buf.pop();
    }

    buf
}

#[derive(Debug)]
enum Cmd {
    Cd(String),
    Ls(BTreeMap<String, Option<u64>>)
}

fn parse_file(i: &str) -> IResult<&str, Vec<Cmd>> {
    many0(parse_cmd)(i)
}

fn parse_cmd(i: &str) -> IResult<&str, Cmd> {
    let (i, _) = tag("$ ")(i)?;
    alt((
        map(parse_cd, Cmd::Cd),
        map(parse_ls, Cmd::Ls),
    ))(i)
}

fn parse_cd(i: &str) -> IResult<&str, String> {
    let (i, _) = tag("cd ")(i)?;
    let (i, dir) = is_not("\n")(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, dir.to_string()))
}

fn parse_ls(i: &str) -> IResult<&str, BTreeMap<String, Option<u64>>> {
    let (i, _) = tag("ls\n")(i)?;
    map(many0(parse_ls_row), |v| BTreeMap::from_iter(v.into_iter()))(i)
}

fn parse_ls_row(i: &str) -> IResult<&str, (String, Option<u64>)> {
    let (i, size) = alt((
        map(tag("dir"), |_| None),
        map(digit1, |d| Some(str::parse::<u64>(d).unwrap()))
    ))(i)?;
    let (i, _) = char(' ')(i)?;
    let (i, name) = is_not("\n")(i)?;
    let (i, _) = alt((line_ending, eof))(i)?;

    Ok((i, (name.to_string(), size)))
}
