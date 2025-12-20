use std::io::BufRead;

type Coord = (i64, i64);

pub fn part1(input: impl BufRead, _verbose: bool) -> Result<String, Box<dyn std::error::Error>> {
    let lines = input.lines();
    let tiles = lines.collect::<Result<Vec<_>, _>>()?;
    let tiles = tiles
        .into_iter()
        .map(parse_line)
        .collect::<Result<Vec<_>, _>>()?;

    let max_area = tiles
        .iter()
        .flat_map(|(x1, y1)| {
            tiles.iter().filter_map(move |(x2, y2)| {
                if x1 == x2 && y1 == y2 {
                    return None;
                }
                let area = ((x2 - x1 + 1) * (y2 - y1 + 1)).abs();
                Some(area)
            })
        })
        .max()
        .ok_or("No max area")?;

    Ok(max_area.to_string())
}

fn parse_line(line: String) -> Result<Coord, Box<dyn std::error::Error>> {
    let coords = line
        .split(',')
        .map(|s: &str| s.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;
    let [x, y] = coords[..2] else {
        return Err("Invalid coords".into());
    };
    Ok((x, y))
}
