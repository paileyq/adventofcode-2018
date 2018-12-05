use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

pub fn solve(input_file: File) {
  let reader = BufReader::new(input_file);

  let box_ids: Vec<String> = reader.lines().flatten().collect();

  println!("Checksum: {}", get_checksum(&box_ids));
}

fn get_checksum<T: AsRef<str>>(box_ids: &[T]) -> usize {
  let mut num_containing_two = 0;
  let mut num_containing_three = 0;

  for box_id in box_ids.iter() {
    let freq = char_frequency(box_id.as_ref());

    if freq.values().any(|&count| count == 2) {
      num_containing_two += 1;
    }
    if freq.values().any(|&count| count == 3) {
      num_containing_three += 1;
    }
  }

  return num_containing_two * num_containing_three;
}

fn char_frequency(string: &str) -> HashMap<char, usize> {
  string.chars().fold(HashMap::new(), |mut freq, letter| {
    *freq.entry(letter).or_insert(0) += 1;
    freq
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_checksum() {
    let box_ids = &[
      "abcdef",
      "bababc",
      "abbcde",
      "abcccd",
      "aabcdd",
      "abcdee",
      "ababab"
    ];

    assert_eq!(12, get_checksum(box_ids));
  }

  #[test]
  fn test_char_frequency() {
    let freq = char_frequency("bababc");

    assert_eq!(2, freq[&'a']);
    assert_eq!(3, freq[&'b']);
    assert_eq!(1, freq[&'c']);
    assert_eq!(None, freq.get(&'d'));
  }
}
