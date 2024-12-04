use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::io::Read;
// use std::str::FromStr;

#[derive(Debug)]
enum Ast {
    Cd(String),
    Ls,
    Dir(String),
    File(String, u32),
}

fn parse(input: String) -> Vec<Ast> {
    let cd_re   = Regex::new(r"^\$ cd (.*)$").unwrap();
    let ls_re   = Regex::new(r"^\$ ls$").unwrap();
    let dir_re  = Regex::new(r"^dir (.*)$").unwrap();
    let file_re = Regex::new(r"^(\d+) (.*)$").unwrap();

    let mut entries: Vec<Ast> = vec![];

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if let Some(captures) = cd_re.captures(line) {
            entries.push(Ast::Cd(captures[1].to_string()));
            continue;
        }

        if let Some(_captures) = ls_re.captures(line) {
            entries.push(Ast::Ls);
            continue;
        }

        if let Some(captures) = dir_re.captures(line) {
            entries.push(Ast::Dir(captures[1].to_string()));
            continue;
        }

        if let Some(captures) = file_re.captures(line) {
            let size: u32 = captures[1].parse().unwrap();
            entries.push(Ast::File(captures[2].to_string(), size));
            continue;
        }

        println!("line: {}", line);

        unreachable!()
    }

    entries
}

#[derive(Debug)]
enum FileType {
    Dir(String, HashMap<String, FileType>, FileType),
    File(String, u32, FileType),
}

impl FileType {
    fn dir(name: &str) -> FileType {
        FileType::Dir(name.to_string(), HashMap::new())
    }

    fn file(name: &str, size: u32) -> FileType {
        FileType::File(name.to_string(), size)
    }

    fn add_child(&mut self, path: &Vec<String>, child: FileType) {
        match self {
            FileType::Dir(name, children) => {
                if path.len() == 1 && !children.contains_key(&path[0]) {
                    children.insert(path[0], child);
                }
            }

            FileType::File(_, _) => unreachable!(),
        }
    }
}

fn main() -> io::Result<()> {
    let input = env::args().nth(1).unwrap();

    let mut rdr: Box<dyn io::Read> = match input.as_ref() {
        "-" => Box::new(io::stdin()),
        _   => Box::new(fs::File::open(input).expect("could not open file")),
    };
    let mut buf = String::new();
    rdr.read_to_string(&mut buf).unwrap();

    let entries = parse(buf);

    let mut pointer = vec!["/".to_string()];
    let mut tree = Tree::new();
    tree.add_child(&pointer, FileType::dir("/"));

    for entry in entries {
        match entry {
            Ast::Cd(dir) => {
                match dir.as_ref() {
                    "/"  => {
                        pointer = vec!["/".to_string()];
                    },
                    ".." => {
                        pointer.pop();
                    },
                    name => {
                        pointer.push(name.to_string())
                    },
                }
            },

            Ast::Ls => {
            },

            Ast::Dir(name) => {
                tree.add_child(&pointer, FileType::dir(&name))
            }

            Ast::File(name, size) => {
                tree.add_child(&pointer, FileType::file(&name, size));
            }
        }
    }

    println!("{:?}", tree);

    Ok(())
}
