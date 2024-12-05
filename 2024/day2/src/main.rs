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
    let input: Vec<&str> = buf.split('\n').filter(|val| !val.is_empty()).collect();

    part1(input.clone());
    part2(input);

    Ok(())
}

fn parse_input(input: Vec<&str>) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = vec![];

    for report in input {
        let levels = report.split(' ')
            .map(|level| level.parse::<i32>().unwrap())
            .collect();

        reports.push(levels)
    }

    reports
}

fn check_safety(report: Vec<i32>) -> bool {
    let levels = report;

    let a = levels[0];
    let b = levels[1];

    let increasing = a < b;

    for i in 0..(levels.len() - 1) {
        let a = levels[i];
        let b = levels[i+1];
        let diff = (a - b).abs();
        // println!("\tA - B -> {} - {}", a, b);

        if (diff < 1 || 3 < diff) || ((a < b) != increasing) {
            // println!("unsafe level {:?}", levels);
            return false
        }
    }

    true
}

fn part1(input: Vec<&str>) {
    let reports = parse_input(input);

    let result = reports.iter()
        .map(|report| { check_safety(report.to_vec()) })
        .filter(|x| *x)
        .count();

    println!("Part 1: {}", result);
}

fn part2(input: Vec<&str>) {
    let reports = parse_input(input);

    let result = reports.iter()
        .map(|report| {
            (0..report.len()).map(|i| {
                let mut new_report = report.clone();
                new_report.remove(i);
                check_safety(new_report)
            })
            .any(|result| result)
        })
        .filter(|result| *result)
        .count();

    println!("Part 2: {}", result);
}
