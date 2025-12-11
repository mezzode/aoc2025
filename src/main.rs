use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

#[derive(Parser)]
/// Advent of Code 2025 Solver CLI
struct Cli {
    problem: String, // TODO: use enum instead

    /// Directly provide input as an argument.
    /// If neither this nor a --file is provided, stdin is used.
    input: Option<String>,

    /// File to read input from.
    /// If neither this nor an [INPUT] is provided, stdin is used.
    #[arg(short, long, conflicts_with = "input")]
    file: Option<String>,

    #[arg(short, long)]
    verbose: bool,
}


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = Cli::parse();

    let reader: Box<dyn BufRead> = if let Some(input) = &cli.input {
        Box::new(BufReader::new(input.as_bytes()))
    } else if let Some(file) = cli.file {
        let file = File::open(file)?;
        Box::new(BufReader::new(file))
    } else {
        Box::new(BufReader::new(std::io::stdin()))
    };

    let answer = match cli.problem.as_str() {
        "01.1" => day01::part1(reader),
        "01.2" => day01::part2(reader),
        "02.1" => day02::part1(reader, cli.verbose),
        "03.1" => day03::part1(reader, cli.verbose),
        "04.1" => day04::part1(reader, cli.verbose),
        "04.2" => day04::part2(reader, cli.verbose),
        "05.1" => day05::part1(reader, cli.verbose),
        "05.2" => day05::part2(reader, cli.verbose),
        "06.1" => day06::part1(reader, cli.verbose),
        "06.2" => day06::part2(reader, cli.verbose),
        "07.1" => day07::part1(reader, cli.verbose),
        _ => Err("Invalid problem".into()),
    }?;

    println!("{}", answer);
    Ok(())
}
