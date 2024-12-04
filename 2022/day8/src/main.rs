use std::env;
use std::fs;
use std::io;
use std::io::Read;
// use std::fmt;
// use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Tree(u32),
    Edge,
}

impl Cell {
    fn height(self) -> u32 {
        match self {
            Cell::Tree(height) => height,
            Cell::Edge => 100_000,
        }
    }
}

type Grid = Vec<Vec<Cell>>;

fn display(grid: &Grid) {
    let mut str = String::new();
    for row in grid {
        for cell in row {
            match cell {
                Cell::Edge => str.push('@'),
                Cell::Tree(height) => str.push_str(&height.to_string()),
            }
        }

        str.push('\n');
    }

    println!("{}", str);
}

fn is_visible(grid: &Grid, x: usize, y: usize, width: usize, height: usize) -> bool {
    let cell = grid[y][x];

    // If we're on the edge then its already visible so don't bother checking the rest.
    if x == 1 || y == 1 || x == width-2 || y == height-2 {
        return true;
    }

    // above
    if grid[1..y].iter().all(|row| row[x].height() < cell.height()) {
        return true;
    }

    // below
    if grid[y+1..height-1].iter().all(|row| row[x].height() < cell.height()) {
        return true;
    }

    // left
    if grid[y][1..x].iter().all(|other| other.height() < cell.height()) {
        return true;
    }

    // right
    if grid[y][x+1..width-1].iter().all(|other| other.height() < cell.height()) {
        return true;
    }

    false
}

fn scenic_score(grid: &Grid, x: usize, y: usize, width: usize, height: usize) -> u32 {
    let mut score = 1;
    let cell = grid[y][x];

    // Above
    let mut count = 0;
    for dy in (1..y).rev(){
        count += 1;
        if grid[dy][x].height() >= cell.height() {
            break;
        }
    }
    score *= count;

    count = 0;
    for dy in y+1..height-1 {
        count += 1;
        if grid[dy][x].height() >= cell.height() {
            break;
        }
    }
    score *= count;

    count = 0;
    for dx in (1..x).rev() {
        count += 1;
        if grid[y][dx].height() >= cell.height() {
            break;
        }
    }
    score *= count;

    count = 0;
    for dx in x+1..width-1 {
        count += 1;
        if grid[y][dx].height() >= cell.height() {
            break;
        }
    }
    score *= count;

    score
}

fn best_scenic_score(grid: &Grid) -> u32 {
    let height = grid.len();
    let width = grid[0].len();

    let mut best_score: u32 = 0;

    for y in 1..height-1 {
        for x in 1..width-1 {
            let score = scenic_score(grid, x, y, width, height);
            if score > best_score {
                best_score = score;
            }
        }
    }

    best_score
}

fn find_visible_trees(grid: &Grid) -> Vec<Cell> {
    let mut trees = vec![];
    let ys = grid.len();
    let xs = grid[0].len();

    // Start 1 indexed since we already have the built in safety of the edges
    for y in 1..ys-1 {
        for x in 1..xs-1 {
            if is_visible(grid, x, y, xs, ys) {
                trees.push(grid[y][x]);
            }
        }
    }

    trees
}

fn main() {
    let input = env::args().nth(1).unwrap();

    let mut rdr: Box<dyn io::Read> = match input.as_ref() {
        "-" => Box::new(io::stdin()),
        _   => Box::new(fs::File::open(input).expect("could not open file")),
    };
    let mut buf = String::new();
    rdr.read_to_string(&mut buf).unwrap();

    let mut grid: Vec<Vec<Cell>> = vec![];

    let grid_size = buf.lines().next().unwrap().len();

    let top_edge = vec![Cell::Edge; grid_size+2];

    grid.push(top_edge.clone());

    for line in buf.lines() {
        let mut grid_line: Vec<Cell> = vec![Cell::Edge];
        for char in line.chars() {
            let height = char.to_digit(10).unwrap();
            grid_line.push(Cell::Tree(height));
        }
        grid_line.push(Cell::Edge);
        grid.push(grid_line);
    }
    grid.push(top_edge);

    display(&grid);

    let visible_trees = find_visible_trees(&grid);
    println!("Part 1: {}", visible_trees.len());

    println!("Part 2: {}", best_scenic_score(&grid));
}
