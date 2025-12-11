use std::{collections::{HashMap, HashSet}, io::{BufRead, Lines}};

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

pub fn part2(input: impl BufRead, verbose: bool) -> Result<String, Box<dyn std::error::Error>> {
        let mut lines = input.lines();

    let start: usize = get_start(&mut lines)?;
    // Map from beam index on current level to count of universes that beam could have originated from
    let mut beams_to_universes = HashMap::from([(start, 1u64)]);

    for line in lines {
        let line = line?;
        let splitters = find_splitters(line);

        let mut new_beams = HashMap::<usize, u64>::new();
        for splitter in splitters {
            if let Some(universes) = beams_to_universes.remove(&splitter) {
                let split_beams = [splitter - 1, splitter + 1];
                for beam in split_beams {
                    if let Some(new_beams_universes) = new_beams.get_mut(&beam) {
                        // Add possible prior universes from other side
                        *new_beams_universes += universes;
                    } else {
                        new_beams.insert(beam, universes);
                    }
                }
            }
        }

        if verbose {
            println!("Beams: {:?}", beams_to_universes);
            println!("New beams: {:?}", new_beams);
        }

        for (beam, universes) in new_beams {
            if let Some(old_universes) = beams_to_universes.get_mut(&beam) {
                // if a continuing beam could have also been produced by splitter, add those universes
                *old_universes += universes;
            } else {
                beams_to_universes.insert(beam, universes);
            }
        }
    }

    let universes: u64 = beams_to_universes.values().sum();
    Ok(universes.to_string())
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
