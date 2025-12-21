use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use itertools::process_results;

struct SearchNode {
    node: String,
    dac_visited: bool,
    fft_visited: bool,
}

pub fn part1(input: impl BufRead, verbose: bool) -> Result<String, Box<dyn std::error::Error>> {
    let graph = build_graph(input)?;
    let mut paths = 0u64;
    let mut to_search = vec!["you".to_string()];

    while let Some(curr) = to_search.pop() {
        let mut goes_to = graph.get(&curr).ok_or("Dead end node")?.clone();
        if goes_to.remove("out") {
            paths += 1;
        }
        to_search.extend(goes_to.into_iter());
        if verbose {
            println!("{:?}", to_search.len());
        }
    }

    Ok(paths.to_string())
}

pub fn part2(input: impl BufRead, verbose: bool) -> Result<String, Box<dyn std::error::Error>> {
    let graph = build_graph(input)?;
    let mut paths = 0u64;
    let mut to_search = vec![SearchNode {
        node: "svr".to_string(),
        dac_visited: false,
        fft_visited: false,
    }];

    while let Some(curr) = to_search.pop() {
        let SearchNode {
            node,
            mut dac_visited,
            mut fft_visited,
        } = curr;
        match node.as_str() {
            "dac" => {
                dac_visited = true;
            }
            "fft" => {
                fft_visited = true;
            }
            _ => {}
        }
        let mut goes_to = graph.get(&node).ok_or("Dead end node")?.clone();
        if goes_to.remove("out") && dac_visited && fft_visited {
            paths += 1;
        }
        to_search.extend(goes_to.into_iter().map(|dest| SearchNode {
            node: dest,
            dac_visited,
            fft_visited,
        }));
        if verbose {
            println!("{:?}", to_search.len());
        }
    }

    Ok(paths.to_string())
}

fn build_graph(
    input: impl BufRead,
) -> Result<HashMap<String, HashSet<String>>, Box<dyn std::error::Error>> {
    let lines = input.lines();
    let graph = process_results(lines, |lines| {
        lines.map(parse_line).collect::<Result<HashMap<_, _>, _>>()
    })?;
    graph
}

fn parse_line(line: String) -> Result<(String, HashSet<String>), Box<dyn std::error::Error>> {
    let (source, dests) = line.split_once(": ").ok_or("Invalid line")?;
    let dests = dests.split(' ').map(String::from).collect::<HashSet<_>>();
    Ok((source.to_string(), dests))
}
