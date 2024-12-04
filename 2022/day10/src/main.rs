use std::env;
use std::fs;
use std::io;
use std::io::Read;
use regex::Regex;

#[derive(Debug)]
enum Op {
    Noop,
    Addx(i32),
}

fn parse(buf: String) -> Vec<Op> {
    let mut ops = vec![];
    let noop_re = Regex::new(r"^noop$").unwrap();
    let addx_re = Regex::new(r"^addx (-?\d+)$").unwrap();

    for line in buf.lines() {
        if line.is_empty() {
            continue;
        }

        if noop_re.is_match(line) {
            ops.push(Op::Noop);
            continue;
        }

        if let Some(captures) = addx_re.captures(line) {
            let num: i32 = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
            ops.push(Op::Noop);
            ops.push(Op::Addx(num));
            continue;
        }
    }

    ops
}

fn display_crt(crt: &Vec<Vec<&str>>) {
    for row in crt {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

fn run(ops: &Vec<Op>) {
    let mut cycle: usize = 0;
    let mut x: i32 = 1;
    let mut total_signal_strength = 0;
    let mut target_cycle = 20;
    let mut crtx: usize = 0;
    let mut crty: usize = 0;

    let mut crt = vec![vec!["."; 40]; 6];

    for op in ops {
        cycle += 1;

        let pixel = vec![x-1, x, x+1];

        for i in pixel {
            if crtx == i as usize {
                crt[crty][crtx] = "#";
            }
        }

        crtx += 1;
        if crtx == 40 {
            crtx = 0;
            crty += 1;
        }

        if crty == 6 {
            crty = 0;
        }

        if target_cycle == cycle {
            target_cycle += 40;
            let signal_strength = cycle * x as usize;
            // println!("Cycle: {} x: {} SS: {}", cycle, x, signal_strength);
            total_signal_strength += signal_strength;
        }

        match op {
            Op::Noop => {},
            Op::Addx(val) => {
                x += val;
            }
        }
    }
    display_crt(&crt);

    println!("Total Signal Strength: {}", total_signal_strength);
}

fn main() {
    let input = env::args().nth(1).unwrap();

    let mut rdr: Box<dyn io::Read> = match input.as_ref() {
        "-" => Box::new(io::stdin()),
        _   => Box::new(fs::File::open(input).expect("could not open file")),
    };
    let mut buf = String::new();
    rdr.read_to_string(&mut buf).unwrap();

    let instructions = parse(buf);
    run(&instructions);
}
