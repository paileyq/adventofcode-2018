use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
  let file = File::open("../input/input99")?;
  let reader = BufReader::new(file);

  for line in reader.lines() {
    println!("{}", line?);
  }
  Ok(())
}
