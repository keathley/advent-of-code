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

#[derive(Debug)]
struct Coord(usize, usize);

pub struct Grid {
    height: usize,
    width: usize,
    cells: Vec<char>,
    grid_width: usize,
}

// We build the grid and add an empty 3 spaces around it that are always empty
// This allows our checking code to ignore any bounds checking since the
// values always fit into the bounds of the grid.
fn build_grid(input: &str) -> Grid {
    let lines: Vec<Vec<char>> = input.split('\n')
         .filter(|row| !row.is_empty())
         .map(|row| { row.chars().collect() })
         .collect();

    let height: usize = lines.len();
    let width: usize = lines[0].len();
    let grid_height = height + 6;
    let grid_width = width + 6;
    let size = grid_width * grid_height;

    let mut cells:Vec<char> = vec!['0'; size];
    for y in 0..height {
        for x in 0..width {
            let c = lines[y][x];
            // pad both down from the top and from the left
            let i = (3 + y) * grid_width + x + 3;
            cells[i] = c;
        }
    }

    Grid{
        height,
        width,
        grid_width,
        cells
    }
}

impl Grid {
    fn at(&self, x: usize, y: usize) -> char {
        // We can skip bounds checking since we've got our padding
        let i = ((y) * self.grid_width) + x;
        self.cells[i]
    }

    fn at_coord(&self, Coord(x, y): &Coord) -> char {
        self.at(*x, *y)
    }

    fn search(&self) -> u32 {
        let mut count = 0;

        for y in 3..self.height+3 {
            for x in 3..self.width+3 {
                if self.at(x, y) != 'X' {
                    continue
                }
                // Checks are done in cardinal directions
                // 0,0   N
                //     W   E
                //       S
                // Because of how we build the grid, to go north you *decrease* Y. To go west you *decrease* X. Obviously if
                // you want to go south or east you increase y and x.
                let checks = vec![
                    (Coord(x,   y-1), Coord(x,   y-2), Coord(x,   y-3)),   // north
                    (Coord(x+1, y-1), Coord(x+2, y-2), Coord(x+3, y-3)), // north-east
                    (Coord(x+1, y),   Coord(x+2, y),   Coord(x+3, y)),   // east
                    (Coord(x+1, y+1), Coord(x+2, y+2), Coord(x+3, y+3)), // south-east
                    (Coord(x,   y+1), Coord(x,   y+2), Coord(x,   y+3)),   // south
                    (Coord(x-1, y+1), Coord(x-2, y+2), Coord(x-3, y+3)), // south-west
                    (Coord(x-1, y),   Coord(x-2, y),   Coord(x-3, y)),   // west
                    (Coord(x-1, y-1), Coord(x-2, y-2), Coord(x-3, y-3)), // north-west
                ];

                for (m, a, s) in checks {
                    if self.at_coord(&m) == 'M' && self.at_coord(&a) == 'A' && self.at_coord(&s) == 'S' {
                        count += 1
                    }
                }
            }
        }

        count
    }

    fn search_x(&self) -> u32 {
        let mut count = 0;

        for y in 3..self.height+3 {
            for x in 3..self.width+3 {
                if self.at(x, y) != 'A' {
                    continue
                }

                let nw = self.at(x-1, y-1);
                let ne = self.at(x-1, y+1);
                let sw = self.at(x+1, y-1);
                let se = self.at(x+1, y+1);

                let live_mas = ((nw == 'M' && se == 'S') || (nw == 'S' && se == 'M')) &&
                               ((ne == 'M' && sw == 'S') || (ne == 'S' && sw == 'M'));

                if live_mas {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn print(&self) {
        println!("Grid:");
        for (i, char) in self.cells.iter().enumerate() {
            if i % self.grid_width == 0 {
                println!();
            }
            print!("{}", char);
        }
        println!();
    }
}

fn part1(input: &str) {
    let grid = build_grid(input);

    let results = grid.search();
    println!("Part 1: {}", results);
}

fn part2(input: &str) {
    let grid = build_grid(input);
    let results = grid.search_x();
    println!("Part 2: {}", results);
}
