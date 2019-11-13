extern crate colored;

use colored::*;
use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let unwrapped_lines = handle.lines().map(|line| line.unwrap());

    for line in unwrapped_lines {
        println!("{}", line.color("red"))
    }

    Ok(())
}
