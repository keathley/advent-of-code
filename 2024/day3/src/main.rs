use regex::Regex;
use nom::{
    branch::alt,
    combinator::value,
    character::complete::i32,
    character::complete::anychar,
    character::complete::alphanumeric0,
    character::complete::alphanumeric1,
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::Tuple,
    IResult,
    Parser,
};
use nom::bytes::complete::take_while;
use nom::sequence::{separated_pair, delimited};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

fn main() -> Result<(), Error> {
    // Get a path if there is one.
    let path = std::env::args_os().nth(1).map(std::path::PathBuf::from);

    // If we have a path we attempt to open the file or exit. Otherwise we assume
    // stdin
    let mut rdr: Box<dyn std::io::Read> = match path {
        Some(path) => Box::new(std::fs::File::open(path).expect("could not open file")),
        None => Box::new(std::io::stdin()),
    };

    let mut buf = String::new();
    rdr.read_to_string(&mut buf).unwrap();
    let input = &buf;
    // let input: Vec<&str> = buf.split('\n').filter(|val| !val.is_empty()).collect();

    part1(input);
    part2(input);

    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Mul(i32, i32),
    Do,
    Dont,
    Skip,
}

impl Op {
    fn keep(self) -> bool {
        match self {
            Op::Skip => false,
            _ => true,
        }
    }
}

fn part1(input: &str) {
    let re = Regex::new(r"mul\((\d{0,3}),(\d{0,3})\)").unwrap();
    let result = re.captures_iter(input)
        .map(|caps| {
            let (_, [x, y]) = caps.extract();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .map(|(x, y)| { x * y })
        .sum::<i32>();

    println!("Part 1: {}", result);
}

fn parse(input: &str) -> Vec<Op> {
    let (_, parsed) = nom::multi::many0(parse_op)(input).unwrap();
    // let ops = parsed.iter()
    //     .filter(|&op| {
    //         match op {
    //             Op::Skip => false,
    //             _ => true
    //         }
    //     })
    //     .collect();

    parsed.iter().filter_map(|&op| {
        match op {
            Op::Skip => None,
            _ => Some(op),
        }
    })
    .collect()
}

fn parse_op(input: &str) -> IResult<&str, Op> {
    alt((
        parse_mul,
        value(Op::Do, tag("do()")),
        value(Op::Dont, tag("don't()")),
        value(Op::Skip, anychar),
    ))(input)
}

fn parse_integer_pair(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        i32,
        tag(","),
        i32
    )(input)
}

fn parse_mul(input: &str) -> IResult<&str, Op> {
   let (remaining, (x, y)) = delimited(
        tag("mul("),
        parse_integer_pair,
        tag(")")
    )(input)?;

    Ok((remaining, Op::Mul(x, y)))
}

fn part2(input: &str) {
    let ops = parse(input);
    let mut enabled = true;

    let mut result = 0;

    for op in ops {
        match op {
            Op::Do => {
                enabled = true
            },
            Op::Dont => {
                enabled = false
            }
            Op::Mul(x, y) => {
                if enabled {
                    result += x * y
                }
            }
            _ => unreachable!("We've already removed skip opts")
        }
    }

    println!("Part 2: {}", result);
}
