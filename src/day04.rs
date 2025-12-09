use std::{collections::HashMap, io::BufRead};

pub fn part1(input: impl BufRead, verbose: bool) -> Result<String, Box<dyn std::error::Error>> {
    let mut filled = HashMap::<(i32, i32), _>::new();
    let mut x_len = 0;
    let mut y_len = 0;

    for (y, line) in input.lines().enumerate() {
        let line = line?;
        if verbose {
            println!("{}", line);
        }
        x_len = line.len();
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                filled.insert((x.try_into()?, y.try_into()?), c);
            }
        }
        y_len = y + 1;
    }

    if verbose {
        println!("Filled:\n{:?}", filled);
    }

    let mut accessible_count = 0u32;
    for (x, y) in filled.keys() {
        if neighbouring_coords(
            (*x).try_into()?,
            (*y).try_into()?,
            Some(0),
            Some(0),
            Some(x_len.try_into()?),
            Some(y_len.try_into()?),
        )
        .filter(|(other_x, other_y)| filled.contains_key(&(*other_x, *other_y)))
        .count()
            < 4
        {
            accessible_count += 1;
        }
    }

    Ok(accessible_count.to_string())
}

fn neighbouring_coords(
    x: i32,
    y: i32,
    min_x: Option<i32>,
    min_y: Option<i32>,
    max_x: Option<i32>,
    max_y: Option<i32>,
) -> impl Iterator<Item = (i32, i32)> {
    (x - 1..=x + 1)
        .flat_map(move |x| (y - 1..=y + 1).map(move |y: i32| (x, y)))
        .filter(move |(other_x, other_y)| !(*other_x == x && *other_y == y))
        .filter(move |(x, y)| {
            min_x.is_none_or(|min_x| min_x <= *x)
                && min_y.is_none_or(|min_y| min_y <= *y)
                && max_x.is_none_or(|max_x| max_x >= *x)
                && max_y.is_none_or(|max_y| max_y >= *y)
        })
}
