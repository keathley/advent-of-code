use std::env;
use std::fs;
use std::io;
use std::io::Read;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space0},
    combinator::map_res,
    error::VerboseError,
    multi::many0,
    sequence::tuple,
    IResult,
};

fn main() -> Result<(), std::io::Error> {
    let input = env::args().nth(1).unwrap();

    let mut rdr: Box<dyn io::Read> = match input.as_ref() {
        "-" => Box::new(io::stdin()),
        _ => Box::new(fs::File::open(input).expect("could not open file")),
    };
    let mut buf = String::new();
    rdr.read_to_string(&mut buf).unwrap();
    let input = parse(&buf);

    part1(input.clone());
    part2(input);

    Ok(())
}

#[derive(Clone, Debug)]
struct Game {
    id: u32,
    handfuls: Vec<Handful>,
}

#[derive(Clone, Debug)]
enum Color {
    Red(u32),
    Blue(u32),
    Green(u32),
}

#[derive(Clone, Debug)]
struct Handful {
    red: u32,
    blue: u32,
    green: u32,
}

type Res<T, U> = IResult<T, U, VerboseError<T>>;

fn num(input: &str) -> Res<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn pcolor(input: &str) -> Res<&str, Color> {
    let color = alt((tag("red"), tag("green"), tag("blue")));

    tuple((num, space0, color))(input).map(|(next, res)| {
        let c = match res.2 {
            "red" => Color::Red(res.0),
            "blue" => Color::Blue(res.0),
            "green" => Color::Green(res.0),
            _ => unreachable!()
        };

        (next, c)
    })
}

fn game(input: &str) -> Res<&str, u32> {
    tuple((tag("Game"), space0, num, tag(":"), space0))(input).map(|(next, res)| (next, res.2))
}

fn handful(input: &str) -> Res<&str, Handful> {
    tuple((pcolor, many0(tuple((tag(","), space0, pcolor)))))(input).map(|(next, res)| {
        let mut colors: Vec<Color> = vec![];
        colors.push(res.0);

        for color in res.1 {
            colors.push(color.2)
        }

        let mut h = Handful{
            red: 0,
            blue: 0,
            green: 0
        };

        for color in colors {
            match color {
                Color::Red(n) => h.red = n,
                Color::Blue(n) => h.blue = n,
                Color::Green(n) => h.green = n,
            }
        }

        (next, h)
    })
}


fn handfuls(input: &str) -> Res<&str, Vec<Handful>> {
    tuple((handful, many0(tuple((tag(";"), space0, handful)))))(input).map(|(next, res)| {
        let mut handfuls = vec![];
        handfuls.push(res.0);
        for hs in res.1 {
            handfuls.push(hs.2);
        }

        (next, handfuls)
    })
}

fn games(input: &str) -> Res<&str, Game> {
    tuple((
        game,
        handfuls,
        line_ending,
    ))(input)
    .map(|(next, res)| {
        let game = Game{
            id: res.0,
            handfuls: res.1
        };

        (next, game)
    })
}

fn parse(input: &str) -> Vec<Game> {
    many0(games)(input).unwrap().1
}

fn part1(games: Vec<Game>) {
    let sum = games.iter().filter(|game| game.handfuls.iter().all(|hf| hf.red <= 12 && hf.green <= 13 && hf.blue <= 14))
        .map(|game| game.id)
        .sum::<u32>();

    println!("Part 1: {}", sum);
}

fn part2(games: Vec<Game>) {
    let sum = games.iter()
        .map(|game| {
            (game.handfuls.iter().map(|hf| hf.red).max().unwrap(),
             game.handfuls.iter().map(|hf| hf.green).max().unwrap(),
             game.handfuls.iter().map(|hf| hf.blue).max().unwrap())
        })
        .map(|(r,g,b)| r*g*b)
        .sum::<u32>();

    println!("Part 2: {}", sum);
}
