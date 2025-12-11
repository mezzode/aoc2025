use std::{collections::HashSet, io::{BufRead, Lines}};

pub fn part1(input: impl BufRead, verbose: bool) -> Result<String, Box<dyn std::error::Error>> {
    let mut lines = input.lines();

    let start: usize = get_start(&mut lines)?;
    let mut splits: u64 = 0;
    let mut beams = HashSet::from([start]);

    for line in lines {
        let line = line?;
        let splitters = find_splitters(line);

        let new_beams: HashSet<usize> = splitters
            .into_iter()
            .flat_map(|splitter| {
                if beams.remove(&splitter) {
                    splits += 1; // Ew side effect
                    Some([splitter - 1, splitter + 1])
                } else {
                    None
                }
            })
            .flatten()
            .collect();

        if verbose {
            println!("Splits: {}", splits);
            println!("Beams: {:?}", beams);
            println!("New beams: {:?}", new_beams);
        }

        beams.extend(new_beams);
    }

    Ok(splits.to_string())
}

fn get_start(lines: &mut Lines<impl BufRead>) -> Result<usize, Box<dyn std::error::Error>> {
    let start_line = lines.next().ok_or("No start line")??;
    start_line.find('S').ok_or("No start".into())
}

fn find_splitters(line: String) -> Vec<usize> {
    line
        .char_indices()
        .filter_map(|(i, c)| if c == '^' { Some(i) } else { None })
        .collect()
}
