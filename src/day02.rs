use std::io::BufRead;

/*
 * Part 1
 * Find all invalid IDs in the given ranges
 * No IDs have leading 0s
 * Invalid IDs consist of a string of digits that repeats twice
 */
pub fn part1(input: impl BufRead, verbose: bool) -> Result<String, Box<dyn std::error::Error>> {
    let mut sum = 0;
    let ranges = input.split(b',');
    for range in ranges {
        let range = range?;
        let range = String::from_utf8(range)?;
        let [start, end] = range.split("-").collect::<Vec<_>>()[..2] else {
            panic!("Invalid range");
        };
        let starting_digit_length = start.len();
        let ending_digit_length = end.len();
        for digit_length in starting_digit_length..=ending_digit_length {
            if digit_length % 2 == 1 {
                continue;
            }
            // For each even digit length, generate all invalid IDs
            // Invalid IDs consist of a set of digits that repeats twice
            let min_for_digit_length = if digit_length == starting_digit_length {
                // If this is the start, initialize at the given start ID
                Some(start.chars().collect())
            } else {
                None
            };

            let max_for_digit_length = if digit_length == ending_digit_length {
                Some(end.chars().collect())
            } else {
                None
            };

            sum += invalid_ids_for_length(
                digit_length,
                min_for_digit_length,
                max_for_digit_length,
                verbose,
            )?
            .into_iter()
            .map(|id| id.parse::<u64>())
            .sum::<Result<u64, _>>()?;
        }
    }
    Ok(sum.to_string())
}

pub fn invalid_ids_for_length(
    length: usize,
    min: Option<String>,
    max: Option<String>,
    verbose: bool,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // TODO: make into a generator/iterator instead of storing all
    if length % 2 == 1 {
        return Ok(Vec::new()); // Only IDs with even lengths can be invalid
    }

    let min = match min {
        Some(min) => {
            if min.len() != length {
                return Err("Invalid min length".into());
            }
            min
        }
        None => format!("1{}", "0".repeat(length - 1)),
    };
    let max = match max {
        Some(max) => {
            if max.len() != length {
                return Err("Invalid max length".into());
            }
            max
        }
        None => format!("9{}", "9".repeat(length - 1)),
    };

    let min_prefix = min[..length / 2].parse::<i32>()?;
    let max_prefix = max[..length / 2].parse::<i32>()?;
    if verbose {
        println!("Min: {}", min);
        println!("Max: {}", max);
        println!("Min prefix: {}", min_prefix);
        println!("Max prefix: {}", max_prefix);
    }

    let mut invalid_ids = Vec::<String>::new();

    for prefix in min_prefix..=max_prefix {
        if prefix == min_prefix && min[length / 2..].parse::<i32>()? > prefix {
            continue; // Only add the invalid ID for the min prefix if min is lower than it
        }
        if prefix == max_prefix && max[length / 2..].parse::<i32>()? < prefix {
            continue; // Only add the invalid ID for the max prefix if max is higher than it
        }
        invalid_ids.push(format!("{}{}", prefix, prefix));
    }

    if verbose {
        println!("Invalid IDs for length {}: {:?}", length, invalid_ids);
    }

    return Ok(invalid_ids);
}
