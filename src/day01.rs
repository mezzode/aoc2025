use std::io::BufRead;

pub fn part1(input: impl BufRead) -> Result<String, Box<dyn std::error::Error>> {
    let mut position = 50;
    let mut zero_count = 0;

    let lines = input.lines();
    for line in lines {
        let line = line.unwrap();
        let (direction, n) = line.split_at(1);
        let n: i32 = n.parse().unwrap();
        eprintln!("{} {}", direction, n);
        let n = match direction {
            "L" => -n,
            "R" => n,
            _ => panic!("Invalid direction"),
        };
        let n = n % 100;
        position += n;
        position %= 100;
        eprintln!("{}", position);
        if position == 0 {
            zero_count += 1;
        }
    }
    Ok(zero_count.to_string())
}

pub fn part2(input: impl BufRead) -> Result<String, Box<dyn std::error::Error>> {
    let mut position = 50;
    let mut zero_count = 0;

    let lines = input.lines();
    for line in lines {
        let line = line.unwrap();
        let (direction, n) = line.split_at(1);
        let n: i32 = n.parse().expect("Invalid number");
        eprintln!("{} {}", direction, n);

        let n = match direction {
            "L" => -n,
            "R" => n,
            _ => panic!("Invalid direction"),
        };

        let old_position = position;
        position += n; // Rotate dial
        eprintln!("Pos: {}", position);
        let rotations = (position / 100).abs(); // Calculate full rotations
        eprintln!("Rotations: {}", rotations);
        zero_count += rotations;

        position %= 100; // Remove counted rotations. Now -99 < pos < 99
        if direction == "L" && position <= 0 && old_position > 0 {
            // If sign changed, crossed 0 an extra time not counted above
            zero_count += 1;
        }
        eprintln!("Zeros: {}", zero_count);
        eprintln!("Pre-Normalized Pos: {}", position);
        if position < 0 {
            position += 100; // Normalize to 0-99 range
        }
        eprintln!("Normalized Pos: {}", position);
    }
    Ok(zero_count.to_string())
}
