use std::io::BufRead;

pub fn part1(input: impl BufRead, verbose: bool) -> Result<String, Box<dyn std::error::Error>> {
    let total_joltage = input
        .lines()
        .map(|line| max_bank_joltage(line?, verbose))
        .sum::<Result<u32, _>>()?;
    Ok(total_joltage.to_string())
}

fn max_bank_joltage(bank: String, verbose: bool) -> Result<u32, Box<dyn std::error::Error>> {
    let mut joltage = 0u32;
    let mut biggest_prev_batt = None;

    if verbose {
        println!("Bank: {}", bank);
    }

    for battery in bank.chars() {
        if verbose {
            println!(
                "Biggest prev batt: {:?}, Joltage: {}, Battery: {}",
                biggest_prev_batt, joltage, battery
            );
        }
        if biggest_prev_batt == None {
            biggest_prev_batt = Some(battery);
            continue;
        }

        let potential_joltage = format!(
            "{}{}",
            biggest_prev_batt.ok_or("Invalid")?,
            battery,
        )
        .parse()?;
        if potential_joltage > joltage {
            joltage = potential_joltage;
        }
        if battery > biggest_prev_batt.ok_or("Invalid")? {
            biggest_prev_batt = Some(battery);
        }
    }
    if verbose {
        println!("Joltage: {}", joltage);
    }
    return Ok(joltage);
}
