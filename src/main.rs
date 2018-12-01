use std::env;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

mod day01;

fn main() -> io::Result<()> {
  let args: Vec<String> = env::args().collect();

  let file = File::open(&args[1])?;
  let reader = BufReader::new(file);

  let freq_changes: Vec<i32> = reader.lines().map(|line|
    line.unwrap().parse().unwrap()
  ).collect();

  println!(
    "Resulting frequency: {}",
    day01::resulting_frequency(&freq_changes)
  );
  println!(
    "First frequency reached twice: {}",
    day01::first_frequency_reached_twice(&freq_changes)
  );
  Ok(())
}

