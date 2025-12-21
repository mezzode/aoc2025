use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use itertools::process_results;

pub fn part1(input: impl BufRead, _verbose: bool) -> Result<String, Box<dyn std::error::Error>> {
    let lines = input.lines();
    let graph = process_results(lines, |lines| {
        lines
            .map(
                |l| -> Result<(String, HashSet<String>), Box<dyn std::error::Error>> {
                    let (source, dests) = l.split_once(": ").ok_or("Invalid line")?;
                    let dests = dests.split(' ').map(String::from).collect::<HashSet<_>>();
                    Ok((source.to_string(), dests))
                },
            )
            .collect::<Result<HashMap<_, _>, _>>()
    })??;

    let mut paths = 0u64;
    let mut to_search = vec!["you".to_string()];

    while let Some(curr) = to_search.pop() {
        let mut goes_to = graph.get(&curr).ok_or("Dead end node")?.clone();
        if goes_to.remove("out") {
            paths += 1;
        }
        to_search.extend(goes_to.into_iter());
    }

    Ok(paths.to_string())
}
