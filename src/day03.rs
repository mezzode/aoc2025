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
    let mut curr_first = None;
    let mut curr_second = None;

    if verbose {
        println!("Bank: {}", bank);
    }

    for battery in bank.chars() {
        // let battery = battery.to_digit(10)?;
        if verbose {
            println!(
                "First: {:?}, Second: {:?}, Joltage: {}, Battery: {}",
                curr_first, curr_second, joltage, battery
            );
        }
        if curr_first == None {
            curr_first = Some(battery);
            continue;
        }
        if curr_second == None {
            curr_second = Some(battery);
            let potential_joltage =
                format!("{}{}", curr_first.ok_or("Invalid")?, battery,).parse()?;
            if potential_joltage > joltage {
                joltage = potential_joltage;
            }
            continue;
        }
        if battery > curr_second.ok_or("Invalid")? {
            curr_second = Some(battery);
        }
        let potential_joltage = format!(
            "{}{}",
            curr_first.ok_or("Invalid")?,
            curr_second.ok_or("Invalid")?
        )
        .parse()?;
        if potential_joltage > joltage {
            joltage = potential_joltage;
        }
        if battery > curr_first.ok_or("Invalid")? {
            curr_first = Some(battery);
            curr_second = None;
        }
    }
    if verbose {
        println!("Joltage: {}", joltage);
    }
    return Ok(joltage);
}
