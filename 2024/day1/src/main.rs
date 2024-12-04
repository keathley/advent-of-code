use std::env;
use std::fs;
use std::io;
use std::io::Read;

use regex::Regex;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

fn main() -> Result<(), Error> {
    // Get a path if there is one.
    let path = std::env::args_os().nth(1).map(std::path::PathBuf::from);

    // If we have a path we attempt to open the file or exit. Otherwise we assume
    // stdin
    let mut rdr: Box<dyn io::Read> = match path {
        Some(path) => Box::new(fs::File::open(path).expect("could not open file")),
        None => Box::new(io::stdin()),
    };

    let mut buf = String::new();
    rdr.read_to_string(&mut buf).unwrap();
    let input: Vec<&str> = buf.split('\n').filter(|val| !val.is_empty()).collect();

    part1(input.clone());
    part2(input);

    Ok(())
}

fn parse_lists(input: Vec<&str>) -> (Vec<i32>, Vec<i32>) {
    let mut list1 = vec![];
    let mut list2 = vec![];
    let re = Regex::new(r"^(\d+)\s+(\d+)$").unwrap();

    for line in input {
        if let Some(captures) = re.captures(line) {
            list1.push(captures[1].parse::<i32>().unwrap());
            list2.push(captures[2].parse::<i32>().unwrap());
        }
    }

    (list1, list2)
}

fn part1(input: Vec<&str>) {
    let (mut list1, mut list2) = parse_lists(input);

    list1.sort();
    list2.sort();


    let total_distance: i32 = std::iter::zip(list1, list2)
        .map(|(a, b)| { (a - b).abs() })
        .sum();
        // .collect();

    println!("part 1: {}", total_distance);
}

fn part2(input: Vec<&str>) {
    let (list1, list2) = parse_lists(input);

    let mut score = 0;

    for a in list1 {
        let mut count = 0;

        for b in &list2 {
            if a == *b {
                count+=1;
            }
        }
        score += a * count
    }

    println!("part 2: {}", score);
}
