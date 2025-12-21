use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use itertools::process_results;

pub fn part1(input: impl BufRead, _verbose: bool) -> Result<String, Box<dyn std::error::Error>> {
    let graph = build_graph(input)?;
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

fn build_graph(input: impl BufRead) -> Result<HashMap<String, HashSet<String>>, Box<dyn std::error::Error>> {
    let lines = input.lines();
    let graph = process_results(lines, |lines| {
        lines
            .map(parse_line)
            .collect::<Result<HashMap<_, _>, _>>()
    })?;
    graph
}

fn parse_line(line: String) -> Result<(String, HashSet<String>), Box<dyn std::error::Error>> {
    let (source, dests) = line.split_once(": ").ok_or("Invalid line")?;
    let dests = dests.split(' ').map(String::from).collect::<HashSet<_>>();
    Ok((source.to_string(), dests))
}