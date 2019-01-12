use std::fmt;
use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Tile {
  Empty,
  Wall,
  Elf,
  Goblin
}

impl Tile {
  pub fn from_char(c: char) -> Option<Tile> {
    use self::Tile::*;

    match c {
      '.' => Some(Empty),
      '#' => Some(Wall),
      'E' => Some(Elf),
      'G' => Some(Goblin),
       _  => None
    }
  }

  pub fn to_char(self) -> char {
    use self::Tile::*;

    match self {
      Empty  => '.',
      Wall   => '#',
      Elf    => 'E',
      Goblin => 'G'
    }
  }
}

struct World {
  tiles: Vec<Tile>,
  width: usize,
  height: usize
}

impl World {
  pub fn tile(&self, x: usize, y: usize) -> Option<Tile> {
    if x >= self.width || y >= self.height {
      return None;
    }

    Some(self.tiles[y * self.width + x])
  }
}

impl FromStr for World {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<World, Self::Err> {
    let width = s.lines().next().unwrap().len();
    let mut height = 1;
    let mut tiles = Vec::new();

    for c in s.chars() {
      if let Some(tile) = Tile::from_char(c) {
        tiles.push(tile);
      } else if c == '\n' {
        height += 1;
      }
    }

    if tiles.len() != width * height {
      return Err("invalid world map string");
    }

    Ok(World { tiles, width, height })
  }
}

impl Display for World {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for y in 0..self.height {
      let row = self.tiles[y * self.width .. (y+1) * self.width]
        .iter()
        .map(|tile| tile.to_char())
        .collect::<String>();

      write!(f, "{}", row)?;

      if y != self.height - 1 {
        write!(f, "\n")?;
      }
    }

    Ok(())
  }
}

pub fn solve(input_file: File) {
  let mut reader = BufReader::new(input_file);

  let mut map = String::new();
  reader.read_to_string(&mut map).unwrap();

  let world: World = map.trim().parse().unwrap();

  println!("{}", world);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_world() {
    let map = "
#######
#E..G.#
#...#.#
#.G.#G#
#######
";

    let world: World = map.trim().parse().unwrap();

    assert_eq!(world.tile(1, 3), Some(Tile::Empty));
    assert_eq!(world.tile(0, 0), Some(Tile::Wall));
    assert_eq!(world.tile(6, 4), Some(Tile::Wall));
    assert_eq!(world.tile(1, 1), Some(Tile::Elf));
    assert_eq!(world.tile(5, 3), Some(Tile::Goblin));
    assert_eq!(world.tile(7, 3), None);
    assert_eq!(world.tile(0, 5), None);

    assert_eq!(map.trim(), format!("{}", world));
  }
}

