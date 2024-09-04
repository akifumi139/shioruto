use std::fs::OpenOptions;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("task.md")?;

    let mut input = String::new();
    println!("Please enter task text:");
    io::stdin().read_line(&mut input)?;

    let task = input.trim();
    writeln!(file, "- {}", task)?;

    Ok(())
}
