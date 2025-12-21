use itertools::Itertools;
use std::io::BufRead;

pub fn part1(input: impl BufRead, _verbose: bool) -> Result<String, Box<dyn std::error::Error>> {
    let lines = input.lines();

    let total_presses = lines.map(|line| solve(line?)).sum::<Result<usize, _>>()?;
    Ok(total_presses.to_string())
}

fn solve(line: String) -> Result<usize, Box<dyn std::error::Error>> {
    let (target, buttons) = parse_line(line)?;

    // Every time a button is pressed more than once can just be expressed as not pressing it or pressing it once
    for num_buttons in 1..=buttons.len() {
        for buttons_active in buttons.iter().combinations(num_buttons) {
            let toggles = buttons_active
                .iter()
                .fold(vec![0; target.len()], |mut acc, button| {
                    for i in *button {
                        // Counting toggles instead of just toggling a bool in case Part 2 needs it
                        acc[*i] += 1;
                    }
                    acc
                });

            let state = toggles
                .iter()
                .map(|light| light % 2 == 1)
                .collect::<Vec<_>>();
            if state == target {
                return Ok(buttons_active.len());
            }
        }
    }

    Err("No solution found".into())
}

fn parse_line(line: String) -> Result<(Vec<bool>, Vec<Vec<usize>>), Box<dyn std::error::Error>> {
    let components = line.split(' ').collect::<Vec<&str>>();
    let [target, buttons @ .., _joltage] = components.as_slice() else {
        return Err("Invalid line".into());
    };
    Ok((parse_target(target)?, parse_buttons(buttons)?))
}

fn parse_target(target: &&str) -> Result<Vec<bool>, Box<dyn std::error::Error>> {
    (&target[1..target.len() - 1])
        .chars()
        .map(|c| -> Result<bool, Box<dyn std::error::Error>> {
            match c {
                '.' => Ok(false),
                '#' => Ok(true),
                _ => Err("Invalid char".into()),
            }
        })
        .collect::<Result<Vec<_>, _>>()
}

fn parse_buttons(buttons: &[&str]) -> Result<Vec<Vec<usize>>, Box<dyn std::error::Error>> {
    buttons
        .iter()
        .map(parse_button)
        .collect::<Result<Vec<_>, _>>()
}

fn parse_button(button: &&str) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    button[1..button.len() - 1]
        .split(',')
        .map(|b| b.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.into())
}
