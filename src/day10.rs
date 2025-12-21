use itertools::Itertools;
use std::{collections::HashMap, io::BufRead};

pub fn part1(input: impl BufRead, _verbose: bool) -> Result<String, Box<dyn std::error::Error>> {
    let lines = input.lines();

    let total_presses = lines
        .map(|line| solve_part1(line?))
        .sum::<Result<usize, _>>()?;
    Ok(total_presses.to_string())
}

pub fn part2(input: impl BufRead, _verbose: bool) -> Result<String, Box<dyn std::error::Error>> {
    let lines = input.lines();

    let total_presses = lines
        .map(|line| solve_part2(line?))
        .sum::<Result<usize, _>>()?;
    Ok(total_presses.to_string())
}

fn solve_part1(line: String) -> Result<usize, Box<dyn std::error::Error>> {
    let (indicator_target, buttons, _joltage_target) = parse_line(line)?;

    // Every time a button is pressed more than once can just be expressed as not pressing it or pressing it once
    // TODO: For Part 2 this is not true, can press buttons multiple times to reach target joltages
    for num_buttons in 1..=buttons.len() {
        for buttons_active in buttons.iter().combinations(num_buttons) {
            let toggles =
                buttons_active
                    .iter()
                    .fold(vec![0; indicator_target.len()], |mut acc, button| {
                        for i in *button {
                            acc[*i] += 1;
                        }
                        acc
                    });

            let state = toggles
                .iter()
                .map(|light| light % 2 == 1)
                .collect::<Vec<_>>();
            if state == indicator_target {
                return Ok(buttons_active.len());
            }
        }
    }

    Err("No solution found".into())
}

fn solve_part2(line: String) -> Result<usize, Box<dyn std::error::Error>> {
    let (_, buttons, joltage_target) = parse_line(line)?;

    // Memoize results based on Vec of button indices pressed to resulting joltages
    let mut memo: HashMap<Vec<usize>, Vec<i32>> = HashMap::new();

    let mut num_buttons = 1;
    loop {
        let mut new_memo = HashMap::new();
        for (button_index, button) in buttons.iter().enumerate() {
            if num_buttons == 1 {
                let mut joltages = vec![0; joltage_target.len()];
                for i in button {
                    joltages[*i] += 1;
                }

                if joltages == joltage_target {
                    return Ok(num_buttons);
                }
                new_memo.insert(vec![button_index], joltages);
                continue;
            }

            for (mut buttons_active, mut joltages) in memo.clone().drain() {
                for i in button {
                    joltages[*i] += 1;
                }
                if joltages == joltage_target {
                    return Ok(num_buttons);
                }

                // Cull invalid search branches
                let too_high = (0..joltages.len()).any(|i| joltages[i] > joltage_target[i]);
                if !too_high {
                    buttons_active.push(button_index);
                    new_memo.insert(buttons_active, joltages);
                }
            }
        }
        num_buttons += 1;
        memo = new_memo;
    }
}

fn parse_line(
    line: String,
) -> Result<(Vec<bool>, Vec<Vec<usize>>, Vec<i32>), Box<dyn std::error::Error>> {
    let components = line.split(' ').collect::<Vec<&str>>();
    let [indicators, buttons @ .., joltage] = components.as_slice() else {
        return Err("Invalid line".into());
    };
    Ok((
        parse_indicators(indicators)?,
        parse_buttons(buttons)?,
        parse_joltages(joltage)?,
    ))
}

fn parse_indicators(s: &&str) -> Result<Vec<bool>, Box<dyn std::error::Error>> {
    (&s[1..s.len() - 1])
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

fn parse_joltages(s: &&str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    (&s[1..s.len() - 1])
        .split(',')
        .map(|c| -> Result<i32, Box<dyn std::error::Error>> {
            c.parse::<i32>().map_err(|e| e.into())
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
