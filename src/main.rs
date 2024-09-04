use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut file = File::create("task.md")?;

    let mut input = String::new();
    println!("Please enter task text:");
    io::stdin().read_line(&mut input)?;

    let task = input.trim();
    writeln!(file, "- {}", task)?;

    Ok(())
}
