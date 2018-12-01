use std::env;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;

fn main() -> io::Result<()> {
  let args: Vec<String> = env::args().collect();

  let file = File::open(&args[1])?;
  let reader = BufReader::new(file);

  let freq_changes: Vec<i32> = reader.lines().map(|line|
    line.unwrap().parse().unwrap()
  ).collect();

  println!(
    "Resulting frequency: {}",
    resulting_frequency(&freq_changes)
  );
  println!(
    "First frequency reached twice: {}",
    first_frequency_reached_twice(&freq_changes)
  );
  Ok(())
}

fn resulting_frequency(freq_changes: &[i32]) -> i32 {
  freq_changes.iter().sum()
}

fn first_frequency_reached_twice(freq_changes: &[i32]) -> i32 {
  let mut seen = HashSet::from([0]);
  freq_changes.iter()
    .cycle()
    .scan(0, |freq, &change| {
      *freq += change;
      Some(*freq)
    })
    .find(|freq| seen.replace(freq.clone()).is_some())
    .unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_first_frequency_reached_twice() {
    assert_eq!(0, first_frequency_reached_twice(&[1, -1]));
    assert_eq!(10, first_frequency_reached_twice(&[3, 3, 4, -2, -4]));
    assert_eq!(5, first_frequency_reached_twice(&[-6, 3, 8, 5, -6]));
    assert_eq!(14, first_frequency_reached_twice(&[7, 7, -2, -7, -4]));
  }
}
