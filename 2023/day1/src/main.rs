use std::env;
use std::fs;
use std::io;
use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let input = env::args().nth(1).unwrap();

    let mut rdr: Box<dyn io::Read> = match input.as_ref() {
        "-" => Box::new(io::stdin()),
        _   => Box::new(fs::File::open(input).expect("could not open file")),
    };
    let mut buf = String::new();
    rdr.read_to_string(&mut buf).unwrap();
    let input: Vec<&str> = buf.split('\n').filter(|val| !val.is_empty()).collect();

    part1(input.clone());
    part2(input);

    Ok(())
}

fn part1(input: Vec<&str>) {
    let mut nums = vec![];

    for value in input {
        if value.is_empty() {
            continue;
        }

        let mut digits = vec![];

        for char in value.chars() {
            if char.is_ascii_digit() {
                digits.push(char.to_digit(10).unwrap())
            }
        }

        if !digits.is_empty() {
            let num = digits.first().unwrap() * 10 + digits.last().unwrap();
            nums.push(num);
        }
    }

    let sum = nums.iter().sum::<u32>();

    println!("Part 1: {}", sum);
}

fn part2(input: Vec<&str>) {
    let mut nums = vec![];

    for value in input {
        let mut digits = vec![];
        let mut i = 0;
        let chars: Vec<char> = value.chars().collect();
        let replacements = vec![
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];

        loop {
            if i >= chars.len() {
                break;
            }

            if chars[i].is_ascii_digit() {
                digits.push(chars[i].to_digit(10).unwrap());
                i += 1;
                continue;
            }

            if let Some((digit, inc)) = match_words(&replacements, &chars[i..]) {
                i += inc;
                digits.push(digit);
                continue;
            }

            i += 1;
        }

        if !digits.is_empty() {
            let num = digits.first().unwrap() * 10 + digits.last().unwrap();
            nums.push(num);
            println!("{} -> {}", value, num);
        }
    }

    let sum = nums.iter().sum::<u32>();

    println!("Part 2: {}", sum);
}

fn match_words(words: &Vec<(&str, u32)>, chars: &[char]) -> Option<(u32, usize)> {
    for (word, digit) in words {
        if is_match(word, chars) {
            return Some((*digit, word.len()-1));
        }
    }

    None
}

fn is_match(word: &str, chars: &[char]) -> bool {
    let word_chars: Vec<char> = word.chars().collect();
    let mut i = 0;

    while i < word_chars.len() {
        // If we're about to exceed the amount of chars we have don't bother
        // continuing
        if i >= chars.len() {
            return false;
        }

        // If the characters don't match, return a false immediately.
        if word_chars[i] != chars[i] {
            return false
        }

        i += 1;
    }

    // If we got here it means that all of the characters matched.
    true
}
