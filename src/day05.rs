use std::io::BufRead;

pub fn part1(input: impl BufRead, verbose: bool) -> Result<String, Box<dyn std::error::Error>> {
    // TODO: consider merging adjacent ranges?
    let mut fresh_ranges = Vec::<(u64, u64)>::new();

    let mut lines = input.lines();
    for line in lines.by_ref() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let [start, end] = line.split('-').collect::<Vec<_>>()[..2] else {
            return Err("Invalid range".into());
        };
        fresh_ranges.push((start.parse::<u64>()?, end.parse::<u64>()?));
    }

    if verbose {
        println!("Fresh ranges: {:?}", fresh_ranges);
    }

    let mut fresh_count = 0;
    for line in lines.by_ref() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let id = line.parse::<u64>()?;
        if verbose {
            println!("ID: {}", id);
        }

        for (start, end) in &fresh_ranges {
            if id >= *start && id <= *end {
                if verbose {
                    println!("{} is fresh", id);
                }
                fresh_count += 1;
                break;
            }
        }
    }

    Ok(fresh_count.to_string())
}
