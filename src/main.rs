use std::env;
use std::io;
use std::fs::File;

mod day01;

fn main() -> io::Result<()> {
  let args: Vec<String> = env::args().collect();

  let file = File::open(&args[1])?;

  day01::solve(file);

  Ok(())
}

