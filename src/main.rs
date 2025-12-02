use clap::Parser;
use std::fs::File;
use std::io::BufReader;

mod day01;

#[derive(Parser)]
struct Cli {
    problem: String, // TODO: use enum instead
    input: String,
}


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = Cli::parse();

    let file = File::open(cli.input)?;
    let reader = BufReader::new(file);

    let answer = match cli.problem.as_str() {
        "01.1" => day01::part1(reader),
        "01.2" => day01::part2(reader),
        _ => Err("Invalid problem".into()),
    }?;

    println!("{}", answer);
    Ok(())
}
