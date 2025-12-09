use std::{collections::HashMap, io::BufRead};

pub fn part1(input: impl BufRead, verbose: bool) -> Result<String, Box<dyn std::error::Error>> {
    let (filled, x_len, y_len) = build_filled(input, verbose)?;

    if verbose {
        println!("Filled:\n{:?}", filled);
    }

    let bounds = Some(Bounds {
        x_min: Some(0),
        y_min: Some(0),
        x_max: Some(x_len.try_into()?),
        y_max: Some(y_len.try_into()?),
    });

    let accessible_count = filled
        .keys()
        .filter(|(x, y)| removable(&filled, *x, *y, &bounds))
        .count();

    Ok(accessible_count.to_string())
}

pub fn part2(input: impl BufRead, verbose: bool) -> Result<String, Box<dyn std::error::Error>> {
    let (mut filled, x_len, y_len) = build_filled(input, verbose)?;

    let bounds = Some(Bounds {
        x_min: Some(0),
        y_min: Some(0),
        x_max: Some(x_len.try_into()?),
        y_max: Some(y_len.try_into()?),
    });

    let mut removeable = filled
        .keys()
        .filter(|(x, y)| removable(&filled, *x, *y, &bounds))
        .copied()
        .collect::<Vec<_>>();

    let mut removed_count = 0;
    while !removeable.is_empty() {
        removed_count += removeable.len();
        for x_y in removeable {
            filled.remove(&x_y);
        }
        removeable = filled
            .keys()
            .filter(|(x, y)| removable(&filled, *x, *y, &bounds))
            .copied()
            .collect::<Vec<_>>();
    }
    Ok(removed_count.to_string())
}

fn build_filled(
    input: impl BufRead,
    verbose: bool,
) -> Result<(HashMap<(i32, i32), char>, usize, usize), Box<dyn std::error::Error>> {
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
    Ok((filled, x_len, y_len))
}

fn removable(filled: &HashMap<(i32, i32), char>, x: i32, y: i32, bounds: &Option<Bounds>) -> bool {
    neighbouring_coords(x, y, bounds)
        .filter(|(other_x, other_y)| (&filled).contains_key(&(*other_x, *other_y)))
        .count()
        < 4
}

struct Bounds {
    x_min: Option<i32>,
    x_max: Option<i32>,
    y_min: Option<i32>,
    y_max: Option<i32>,
}

fn neighbouring_coords(
    x: i32,
    y: i32,
    bounds: &Option<Bounds>,
) -> impl Iterator<Item = (i32, i32)> {
    (x - 1..=x + 1)
        .flat_map(move |x| (y - 1..=y + 1).map(move |y: i32| (x, y)))
        .filter(move |(other_x, other_y)| !(*other_x == x && *other_y == y))
        .filter(move |(x, y)| {
            bounds.as_ref().is_none_or(|bounds| {
                let Bounds {
                    x_min,
                    x_max,
                    y_min,
                    y_max,
                } = bounds;
                x_min.is_none_or(|x_min| x_min <= *x)
                    && y_min.is_none_or(|y_min| y_min <= *y)
                    && x_max.is_none_or(|x_max| x_max >= *x)
                    && y_max.is_none_or(|y_max| y_max >= *y)
            })
        })
}
