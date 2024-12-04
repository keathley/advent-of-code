use std::{fs, collections::HashSet};

fn priority(item: &char) -> u32 {
    let val = *item as u32;

    if item.is_lowercase() {
        val - 96
    } else {
        val - 38
    }
}

struct Rucksack(Vec<char>, Vec<char>);

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");

    let mut rucksacks = vec![];

    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let half = line.len() / 2;

        let mut compartment1 = Vec::new();
        let mut compartment2 = Vec::new();

        for (i, char) in line.chars().enumerate() {
            if i < half {
                compartment1.push(char);
            } else {
                compartment2.push(char);
            }
        }

        assert!(compartment1.len() == compartment2.len());

        rucksacks.push(Rucksack(compartment1, compartment2))
    }

    let mut priority_sum: u32 = 0;

    for rucksack in &rucksacks {
        let mut matches = vec![];
        for x in &rucksack.0 {
            if matches.contains(x) {
                break;
            }

            for y in &rucksack.1 {
                if x == y {
                    matches.push(*x);
                    break;
                }
            }
        }
        assert!(matches.len() == 1);

        let priority = matches.iter()
            .map(priority)
            .sum::<u32>();

        priority_sum += priority;
    }

    println!("Part 1: {:?}", priority_sum);

    let mut rucksacks = vec![];
    for line in input.lines() {
        if !line.is_empty() {
            rucksacks.push(line)
        }
    }

    let mut badges = vec![];
    for group in rucksacks.chunks(3).map(|x| x.to_vec()) {
        assert!(group.len() == 3);

        let s1: HashSet<_> = group[0].chars().collect();
        let s2: HashSet<_> = group[1].chars().collect();
        let s3: HashSet<_> = group[2].chars().collect();

        for item in s1 {
            if s2.contains(&item) && s3.contains(&item) {
                badges.push(item);
                break;
            }
        }
    }

    let part2: u32 = badges.iter()
        .map(priority)
        .sum();

    println!("Part 2: {}", part2);
}
