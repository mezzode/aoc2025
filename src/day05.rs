use std::io::BufRead;

pub fn part1(mut input: impl BufRead, verbose: bool) -> Result<String, Box<dyn std::error::Error>> {
    let fresh_ranges = get_fresh_ranges(input.by_ref())?;
    let merged_ranges = merge_ranges(fresh_ranges, verbose);

    if verbose {
        println!("Fresh ranges: {:?}", merged_ranges);
    }

    let mut fresh_count = 0;
    let mut lines = input.lines();
    for line in lines.by_ref() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let id = line.parse::<u64>()?;
        if verbose {
            println!("ID: {}", id);
        }

        for (start, end) in &merged_ranges {
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

pub fn part2(input: impl BufRead, verbose: bool) -> Result<String, Box<dyn std::error::Error>> {
    let fresh_ranges = get_fresh_ranges(input)?;
    let merged_ranges = merge_ranges(fresh_ranges, verbose);
    
    let total_fresh: u64 = merged_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum();

    Ok(total_fresh.to_string())
}

fn get_fresh_ranges(input: impl BufRead) -> Result<Vec<(u64, u64)>, Box<dyn std::error::Error>> {
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

    Ok(fresh_ranges)
}

fn merge_ranges(mut ranges: Vec<(u64, u64)>, verbose: bool) -> Vec<(u64, u64)> {
    let mut merged_ranges = Vec::<(u64, u64)>::new();

    if verbose {
        println!("Unsorted ranges: {:?}", ranges);
    }

    ranges.sort();

    if verbose {
        println!("Sorted ranges: {:?}", ranges);
    }

    let (mut curr_start, mut curr_end) = ranges[0];
    for (start, end) in ranges {
        if start > curr_end {
            merged_ranges.push((curr_start, curr_end));
            curr_start = start;
            curr_end = end;
            continue;
        }
        if end > curr_end {
            curr_end = end;
        }
    }
    merged_ranges.push((curr_start, curr_end));

    if verbose {
        println!("Merged ranges: {:?}", merged_ranges);
    }

    merged_ranges
}