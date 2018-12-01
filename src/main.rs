use std::env;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
  let args: Vec<String> = env::args().collect();

  let file = File::open(&args[1])?;
  let reader = BufReader::new(file);

  let mut total = 0;

  for line in reader.lines() {
    let freq: i32 = line?.parse().unwrap();
    total += freq;
  }

  println!("Resulting frequency: {}", total);
  Ok(())
}
