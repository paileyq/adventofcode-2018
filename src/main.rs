use std::env;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
  let args = env::args().collect();

  let file = File::open(&args[1])?;
  let reader = BufReader::new(file);

  for line in reader.lines() {
    println!("{}", line?);
  }
  Ok(())
}
