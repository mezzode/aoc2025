use std::io;

fn main() -> io::Result<()> {
    let mut position = 50;
    let mut zero_count = 0;

    let lines = io::stdin().lines();

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

    println!("{}", zero_count);
    Ok(())
}
