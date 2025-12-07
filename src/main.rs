use clap::Parser;
use std::fs::File;
use std::io::BufReader;

mod day01;
mod day02;
mod day03;

#[derive(Parser)]
struct Cli {
    problem: String, // TODO: use enum instead
    input: String,

    #[arg(short, long)]
    verbose: bool,
}


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = Cli::parse();

    let file = File::open(cli.input)?;
    let reader = BufReader::new(file);

    let answer = match cli.problem.as_str() {
        "01.1" => day01::part1(reader),
        "01.2" => day01::part2(reader),
        "02.1" => day02::part1(reader, cli.verbose),
        "03.1" => day03::part1(reader, cli.verbose),
        _ => Err("Invalid problem".into()),
    }?;

    println!("{}", answer);
    Ok(())
}
