use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
  let file = File::open("../input/input01")?;
  let reader = BufReader::new(file);

  for line in reader.lines() {
    println!("{}", line);
  }
}
