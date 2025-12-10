use std::{io::BufRead, iter::zip};

pub fn part1(input: impl BufRead, _verbose: bool) -> Result<String, Box<dyn std::error::Error>> {
    let mut columns = Vec::<Vec<u64>>::new();
    let lines = input.lines();
    for line in lines {
        let line = line?;
        if matches!(line.chars().nth(0).ok_or("Line empty")?, '+' | '*') {
            let line = line.split_whitespace();
            let ops = line.map(|op| op.chars().nth(0)).collect::<Option<Vec<_>>>();
            let ops = ops.ok_or("Failed to parse ops")?;
            return Ok(calculate(columns, ops)?.to_string());
        }
        let numbers = line.split_whitespace().map(|n| n.parse::<u64>());
        if columns.is_empty() {
            let numbers = numbers.collect::<Result<Vec<_>, _>>()?;
            columns = numbers.into_iter().map(|n| vec![n]).collect();
        } else {
            for (col, n) in zip(columns.iter_mut().by_ref(), numbers) {
                col.push(n?);
            }
        }
    }

    Err("Ops line not provided".into())
}

pub fn part2(input: impl BufRead, verbose: bool) -> Result<String, Box<dyn std::error::Error>> {
    let (columns, ops) = parse(input)?;
    if verbose {
        println!("Columns: {:?}", columns);
        println!("Ops: {:?}", ops);
    }
    Ok(calculate(columns, ops)?.to_string())
}

fn calculate(numbers: Vec<Vec<u64>>, ops: Vec<char>) -> Result<u64, Box<dyn std::error::Error>> {
    zip(numbers, ops)
        .map(|(n, op)| match op {
            '+' => Ok(n.into_iter().sum()),
            '*' => n
                .into_iter()
                .reduce(|a, b| a * b)
                .ok_or("Failed to reduce".into()),
            _ => Err("Invalid op".into()),
        })
        .sum()
}

fn parse(input: impl BufRead) -> Result<(Vec<Vec<u64>>, Vec<char>), Box<dyn std::error::Error>> {
    let mut columns = Vec::<Vec<u64>>::new();
    let lines: Vec<String> = input.lines().collect::<Result<Vec<_>, _>>()?;

    let numbers = &lines[..lines.len() - 1];
    let mut number_col = Vec::<u64>::new();
    for i in 0..numbers[0].len() {
        let col: String = numbers
            .into_iter()
            .map(|line| line.chars().nth(i))
            .filter_map(|c| match c {
                Some(' ') | None => None,
                Some(c) => Some(c),
            })
            .collect();
        if col.is_empty() {
            columns.push(number_col);
            number_col = Vec::<u64>::new();
            continue;
        }
        let n = col.parse::<u64>()?;
        number_col.push(n);
    }
    columns.push(number_col);

    let ops = lines[lines.len() - 1]
        .split_whitespace()
        .map(|op| op.chars().nth(0))
        .collect::<Option<Vec<_>>>()
        .ok_or("Failed to parse ops")?;

    Ok((columns, ops))
}
