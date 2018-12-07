use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Rectangle {
  id: usize,
  x: usize,
  y: usize,
  w: usize,
  h: usize
}

impl FromStr for Rectangle {
  type Err = ParseIntError;

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    let nums: Vec<usize> = string
      .split(|c: char| !c.is_numeric())
      .filter(|n| !n.is_empty())
      .map(|n| n.parse())
      .collect::<Result<Vec<usize>, ParseIntError>>()?;

    Ok(Rectangle { id: nums[0], x: nums[1], y: nums[2], w: nums[3], h: nums[4] })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rectangle_from_str() {
    assert_eq!(
      "#1 @ 1,3: 4x4".parse(),
      Ok(Rectangle { id: 1, x: 1, y: 3, w: 4, h: 4 })
    );
    assert_eq!(
      "#34 @ 36,23: 41x37".parse(),
      Ok(Rectangle { id: 34, x: 36, y: 23, w: 41, h: 37 })
    );
  }
}
