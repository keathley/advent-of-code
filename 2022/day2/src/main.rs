use std::fs;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Win,
    Loss,
    Draw,
}

type Score = u32;

impl Shape {
    fn score(self) -> Score {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn beats(self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn losses_to(self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }
}

struct Round(Shape, Shape);

impl Round {
    fn score(self) -> Score {
        let my_score = self.1.score();
        let result_score = match self.result() {
            Result::Loss => 0,
            Result::Draw => 3,
            Result::Win  => 6,
        };

        result_score + my_score
    }

    fn result(self) -> Result {
        if self.0 == self.1 {
            return Result::Draw;
        }

        match self {
            Round(Shape::Rock, Shape::Paper)     => Result::Win,
            Round(Shape::Paper, Shape::Scissors) => Result::Win,
            Round(Shape::Scissors, Shape::Rock)  => Result::Win,
            _                                    => Result::Loss
        }

    }
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("could not read input");

    let mut rounds: Vec<Round> = vec![];

    for line in input.lines() {
        let elf = match line.chars().next().expect("Incorrect line") {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            _ => unreachable!(),
        };

        let us = match line.chars().nth(2).expect("Incorrect line") {
            'X' => Shape::Rock,
            'Y' => Shape::Paper,
            'Z' => Shape::Scissors,
            _ => unreachable!(),
        };

        rounds.push(Round(elf, us))
    }

    let mut score: u32 = 0;

    for round in rounds {
        score += round.score()
    }

    println!("Part 1: {}", score);

    let mut rounds: Vec<Round> = vec![];

    for line in input.lines() {
        let elf = match line.chars().next().expect("Incorrect line") {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            _ => unreachable!(),
        };

        let us = match line.chars().nth(2).expect("Incorrect line") {
            'X' => {
                elf.beats()
            },
            'Y' => {
                elf
            },
            'Z' => {
                elf.losses_to()
            },
            _ => unreachable!(),
        };

        rounds.push(Round(elf, us))
    }

    let mut score = 0;
    for round in rounds {
        score += round.score();
    }

    println!("Part 2: {}", score);
}
