use std::env;
use std::fs;
use std::io;
use std::io::Read;

fn distinct_bytes(bytes: &[u8], start: usize, count: usize) -> bool {
    for i in start..(start+count) {
        for j in i+1..(start+count) {
            if bytes[i] == bytes[j] {
                return false
            }
        }
    }

    true
}

fn find_marker(bytes: &[u8], buf_length: usize, marker_length: usize) -> Option<usize> {
    for i in 0..buf_length {
        if i+marker_length >= buf_length {
            return None;
        }

        if distinct_bytes(bytes, i, marker_length) {
            return Some(i+marker_length)
        }
    }

    None
}

fn main() -> io::Result<()> {
    let input = env::args().nth(1).unwrap();

    let mut rdr: Box<dyn io::Read> = match input.as_ref() {
        "-" => Box::new(io::stdin()),
        _   => Box::new(fs::File::open(input).expect("could not open file")),
    };
    let mut buf = vec![];
    let length = rdr.read_to_end(&mut buf).expect("could not read input");

    println!("Part 1: {}", find_marker(&buf, length, 4).expect("Could not find marker"));
    println!("Part 2: {}", find_marker(&buf, length, 14).expect("Could not find marker"));

    Ok(())
}
