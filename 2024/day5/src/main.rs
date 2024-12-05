use std::{cmp::Ordering, collections::HashMap};

use nom::{
    bytes::complete::tag, character::complete::{digit1, newline}, combinator::map_res, multi::{many1, many_till, separated_list1}, sequence::{terminated, tuple}, IResult
};

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

    let _ = part1(input);
    let _ = part2(input);

    Ok(())
}

type Rule = (u32, u32);
type Update = Vec<u32>;

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (remaining, (a, _, b)) = terminated(
            tuple((
            map_res(digit1, str::parse),
            tag("|"),
            map_res(digit1, str::parse),
        )),
        newline
        )(input)?;
    Ok((remaining, (a, b)))
}

fn parse_update(input: &str) -> IResult<&str, Update> {
    terminated(separated_list1(tag(","), map_res(digit1, str::parse)), newline)(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<Rule>, Vec<Update>)> {
    let (remaining, (rules, _)) = many_till(parse_rule, newline)(input)?;
    let (remaining, updates) = many1(parse_update)(remaining)?;
    // Ok((rules, updates))
    Ok((remaining, (rules, updates)))
}

fn part1(input: &str) -> Result<(), Error> {
    let (_, (rules, updates)) = parse(input).unwrap();

    // Build rules table for easy lookup
    let mut rule_table = HashMap::new();

    for (a, b) in rules {
        let rules = rule_table.entry(a).or_insert(vec![]);
        rules.push(b);
    }

    let mut good_updates = vec![];

    for update in updates {
        // To check that our rules are being respected we're going to loop over
        // each page number and then loop over any page numbers "forward" in the
        // list. For each future page we check to see if there is a rule that states
        // that the future number is required to come before our current page number.
        // if there is then we know that there's an error in this update.
        // Otherwise we stick our update in the list of good updates.
        let any_violations = update.iter()
            .enumerate()
            .any(|(i, page)| {
                let forward = &update[i..];

                // If we have any violations we can bail out and return false
                forward.iter().any(|f| {
                    // If there are no rules than we can return false
                    // otherwise if we have a violation we return true immediately.
                    match rule_table.get(f) {
                        Some(rules) => rules.contains(page),
                        None => false
                    }
                })
            });

        if !any_violations {
            good_updates.push(update.clone())
        }
    }

    let result = good_updates.iter()
        .map(|update| update[update.len()/2])
        .sum::<u32>();


    println!("Part 1: {}", result);
    Ok(())
}

type RulesTable = HashMap<u32, Vec<u32>>;

fn is_valid_update(update: Update, rule_table: RulesTable) -> bool {
    update.iter()
        .enumerate()
        .any(|(i, page)| {
            let forward = &update[i..];

            // If we have any violations we can bail out and return false
            forward.iter().any(|f| {
                // If there are no rules than we can return false
                // otherwise if we have a violation we return true immediately.
                match rule_table.get(f) {
                    Some(rules) => rules.contains(page),
                    None => false
                }
            })
        })
}

// Returns true if page 1 must come before page 2
fn page_must_be_before(page1: &u32, page2: &u32, table: &RulesTable) -> bool {
    match table.get(page1) {
        Some(rules) => rules.contains(page2),
        None => false,
    }
}

fn page_order(page1: &u32, page2: &u32, table: &RulesTable) -> Ordering {
    // If page 1 must be before page 2 we return Less
    // And if page 2 must be before page 1 we return Greater
    // otherwise we return equal
    if page_must_be_before(page1, page2, table) { return Ordering::Less }
    if page_must_be_before(page2, page1, table) { return Ordering::Greater }

    Ordering::Equal
}

fn part2(input: &str) -> Result<(), Error> {
    let (_, (rules, updates)) = parse(input).unwrap();

    // Build rules table for easy lookup
    let mut rule_table = HashMap::new();

    for (a, b) in rules {
        let rules = rule_table.entry(a).or_insert(vec![]);
        rules.push(b);
    }

    let rule_table = rule_table;

    let bad_updates:Vec<Update> = updates.into_iter().filter(|update| {
        // To check that our rules are being respected we're going to loop over
        // each page number and then loop over any page numbers "forward" in the
        // list. For each future page we check to see if there is a rule that states
        // that the future number is required to come before our current page number.
        // if there is then we know that there's an error in this update.
        // Otherwise we stick our update in the list of good updates.
        update.iter()
            .enumerate()
            .any(|(i, page)| {
                let forward = &update[i..];

                // If we have any violations we can bail out and return false
                forward.iter().any(|f| {
                    // If there are no rules than we can return false
                    // otherwise if we have a violation we return true immediately.
                    match rule_table.get(f) {
                        Some(rules) => rules.contains(page),
                        None => false
                    }
                })
            })
    })
    .collect::<Vec<Update>>();

    let result = bad_updates.iter()
        .map(|update| {
            let mut sorted = update.clone();
            sorted.sort_by(|a, b| page_order(a, b, &rule_table));
            sorted
        })
        .map(|update| update[update.len()/2])
        .sum::<u32>();

    println!("Part 2: {}", result);

    Ok(())
}
