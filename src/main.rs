use std::env;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
  let args: Vec<String> = env::args().collect();

  let file = File::open(&args[1])?;
  let reader = BufReader::new(file);

  let freq_changes: Vec<i32> = reader.lines().map(|line|
    line.unwrap().parse().unwrap()
  ).collect();

  println!("Resulting frequency: {}", resulting_frequency(freq_changes));
  Ok(())
}

fn resulting_frequency(freq_changes: &[i32]) -> i32 {
  freq_changes.iter().sum()
}
