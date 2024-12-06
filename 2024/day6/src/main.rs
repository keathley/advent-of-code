use std::{
    collections::HashSet, time::Instant}
;
use rayon::prelude::*;

use nom::{branch::alt, bytes::complete::tag, character::complete::newline, combinator::value, multi::many1, sequence::terminated, IResult};
use thiserror::Error;
use anyhow::Result;

#[derive(Error, Debug)]
pub enum PuzzleError {
}

#[derive(Clone, Copy, Debug)]
enum Cell {
    Empty,
    Obstacle,
    Guard(Dir),
}

type PuzzleInput = Vec<Vec<Cell>>;
type Steps = Vec<(Dir, usize, usize)>;

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
struct Guard(Dir, usize, usize);

impl Guard {
    fn new(dir: Dir, y: usize, x: usize) -> Guard {
        Guard(dir, y, x)
    }

    fn turn_right(&mut self) {
        let dir = match self.0 {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        };
        self.0 = dir;
    }

    fn step(&mut self, y: usize, x: usize) {
        self.1 = y;
        self.2 = x;
    }
}

#[derive(Clone, Debug)]
struct Grid {
    height: usize,
    width: usize,
    cells: Vec<Vec<Cell>>
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq, Ord, PartialOrd)]
enum Dir {
    North,
    East,
    South,
    West
}

fn parse_guard(input: &str) -> IResult<&str, Cell> {
    let (remaining, dir) = alt((
        value(Dir::North, tag("^")),
        value(Dir::East, tag(">")),
        value(Dir::South, tag("v")),
        value(Dir::West, tag("<")),
    ))(input)?;

    Ok((remaining, Cell::Guard(dir)))
}

fn parse(input: &str) -> IResult<&str, PuzzleInput> {
    many1(
        terminated(
            many1(
                alt((
                    value(Cell::Empty, tag(".")),
                    value(Cell::Obstacle, tag("#")),
                    parse_guard
                ))
            ),
            newline
        )
    )(input)
}

enum Sim {
    Cont,
    Fin,
}

fn next_step(guard: &mut Guard, grid: &Grid) -> Sim {
    let next = match *guard {
        Guard(Dir::North, y, x) if y > 0 => Some((y-1, x)),
        Guard(Dir::East, y, x) if x < grid.width-1 => Some((y, x+1)),
        Guard(Dir::South, y, x) if y < grid.height-1 => Some((y+1, x)),
        Guard(Dir::West, y, x) if x > 0 => Some((y, x-1)),
        _ => None
    };

    // If we have a next step then we check the grid to see what our guard does.
    // None means we're at the border so we're done.
    match next {
        Some((y, x)) => {
            match grid.cells[y][x] {
                Cell::Obstacle => guard.turn_right(),
                _ => guard.step(y, x)
            }
            Sim::Cont
        }
        None => {
            Sim::Fin
        }
    }
}

fn simulate(guard: Guard, grid: Grid) -> Steps {
    let mut steps = vec![];

    let mut guard = guard;

    loop {
        steps.push((guard.0, guard.1, guard.2));

        match next_step(&mut guard, &grid) {
            Sim::Fin => break,
            Sim::Cont => ()
        };
    }

    steps
}

fn find_loop(guard: Guard, grid: Grid) -> bool {
    let mut slow_guard = guard;
    let mut is_loop = false;
    let mut history = HashSet::new();

    loop {
        history.insert(slow_guard);

        // We can skip the check for the slow guard since our fast guard will find the exit much faster
        // let _ = next_step(&mut slow_guard, &grid);
        if let Sim::Fin = next_step(&mut slow_guard, &grid) {
            break
        }

        // If we've seen this exact same direction at this point we *have* to be
        // in a loop since the board never changes.
        if history.contains(&slow_guard) {
            is_loop = true;
            break
        }
    }

    is_loop
}

fn part1(guard: Guard, grid: Grid) -> Result<()> {
    let steps = simulate(guard, grid);

    let mut coords: Vec<(usize, usize)> = steps.iter().map(|(_, y, x)| (*y, *x)).collect();
    coords.sort();
    coords.dedup();

    println!("Part 1: {}", coords.len());
    Ok(())
}

fn part2(guard: Guard, grid: Grid) -> Result<()> {
    // Build permutations of the grid by looking at the actual path of the guard and
    // only adding obstacles in that path.
    let mut steps = simulate(guard, grid.clone());
    steps.sort();
    steps.dedup();

    let mut obstacles = HashSet::new();

    for step in steps {
        let result = match step {
            (Dir::North, y, x) if y > 0 => Some((y-1, x)),
            (Dir::East, y, x) if x < grid.width-1 => Some((y, x+1)),
            (Dir::South, y, x) if y < grid.height-1 => Some((y+1, x)),
            (Dir::West, y, x) if x > 0 => Some((y, x-1)),
            _ => None,
        };

        if let Some((y, x)) = result {
            obstacles.insert((y, x));
        }
    }

    let mut permutations: Vec<Grid> = vec![];

    for (y, x) in obstacles.iter() {
        if guard.1 == *y && guard.2 == *x {
            continue
        }

        match grid.cells[*y][*x] {
            Cell::Obstacle => { continue },
            _ => {
                let mut other = grid.clone();
                other.cells[*y][*x] = Cell::Obstacle;
                permutations.push(other)
            }
        }
    }

    let now = Instant::now();

    // How is this literally the best way to do pmap?
    let result: u32 = permutations.par_iter()
        .map(|grid| {
            if find_loop(guard, grid.clone()) { 1 } else { 0 }
        })
        .sum();

    println!("Elapsed time: {}", now.elapsed().as_millis());

    println!("Part 2: {}", result);

    Ok(())
}

fn main() -> Result<()> {
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

    let (_, parsed) = parse(input).unwrap();

    // We need to find the position of the guard and replace it with a empty cell
    let mut guard = None;

    let mut grid = parsed.clone();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if let Cell::Guard(dir) = grid[y][x] {
                guard = Some(Guard::new(dir, y, x));
                grid[y][x] = Cell::Empty;
            }
        }
    }

    let guard = guard.unwrap();

    // Mark the variable as immutable again
    let grid = Grid{
        height: grid.len(),
        width: grid[0].len(),
        cells: grid.clone()
    };

    let _ = part1(guard, grid.clone());
    let _ = part2(guard, grid);

    Ok(())
}
