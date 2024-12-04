use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("could not read input");
    let mut current_elf_calories = 0;
    let mut elves = vec![];

    for line in input.lines() {
        if line.is_empty() {
            elves.push(current_elf_calories);
            current_elf_calories = 0;
        } else {
            let calories = line.parse::<u32>().unwrap();
            current_elf_calories += calories;
        }
    }

    let max = elves.iter().max().unwrap();
    println!("Part 1: {}", max);

    // Reverse sort and take 3
    elves.sort_by(|a, b| b.cmp(a));
    let top = &elves[0..3];
    let answer: u32 = top.iter().sum();
    println!("Part 2: {:?}", answer);
}
