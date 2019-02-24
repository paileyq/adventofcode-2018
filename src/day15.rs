use std::cmp;
use std::cmp::Ordering;
use std::fmt;
use std::collections::HashMap;
use std::collections::HashSet;
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
  Goblin,
}

impl Tile {
  pub fn from_char(c: char) -> Option<Tile> {
    use self::Tile::*;

    Some(match c {
      '.' => Empty,
      '#' => Wall,
      'E' => Elf,
      'G' => Goblin,
       _  => return None,
    })
  }

  pub fn to_char(self) -> char {
    use self::Tile::*;

    match self {
      Empty  => '.',
      Wall   => '#',
      Elf    => 'E',
      Goblin => 'G',
    }
  }
}

struct World {
  tiles: Vec<Tile>,
  width: usize,
  height: usize,
}

impl World {
  pub fn tile(&self, x: usize, y: usize) -> Option<Tile> {
    if x >= self.width || y >= self.height {
      return None;
    }

    Some(self.tiles[y * self.width + x])
  }

  pub fn distances_from(&self, from_x: usize, from_y: usize) -> HashMap<(usize, usize), usize> {
    let mut distances = HashMap::new();
    let mut unvisited = HashSet::new();

    for tile_x in 0..self.width {
      for tile_y in 0..self.height {
        if self.tile(tile_x, tile_y) == Some(Tile::Empty) {
          unvisited.insert((tile_x, tile_y));
        }
      }
    }

    distances.insert((from_x, from_y), 0);

    let mut current = (from_x, from_y);
    loop {
      let next_distance = distances[&current] + 1;

      for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let neighbor = (
          (current.0 as isize + dx) as usize,
          (current.1 as isize + dy) as usize,
        );

        if self.tile(neighbor.0, neighbor.1) == Some(Tile::Empty) {
          let neighbor_distance = distances.entry(neighbor).or_insert(next_distance);
          *neighbor_distance = cmp::min(*neighbor_distance, next_distance);
        }
      }

      unvisited.remove(&current);

      match unvisited.iter().min_by(|a, b| {
        // None = infinity!
        match (distances.get(a), distances.get(b)) {
          (Some(a), Some(b)) => a.cmp(b),
          (Some(_), None)    => Ordering::Less,
          (None, Some(_))    => Ordering::Greater,
          (None, None)       => Ordering::Equal,
        }
      }) {
        Some(&next) if distances.contains_key(&next) => {
          current = next;
        },
        _ => break,
      };
    }

    distances
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

  #[test]
  fn test_world_distances_from() {
    let map = "
#######
#.E...#
#.....#
#...G.#
#######
";

    let world: World = map.trim().parse().unwrap();

    let distances: HashMap<(usize, usize), usize> = world.distances_from(4, 2);

    assert_eq!(13, distances.len());

    assert_eq!(4, distances[&(1, 1)]);
    assert_eq!(2, distances[&(3, 1)]);
    assert_eq!(1, distances[&(4, 1)]);
    assert_eq!(2, distances[&(5, 1)]);
    assert_eq!(3, distances[&(1, 2)]);
    assert_eq!(2, distances[&(2, 2)]);
    assert_eq!(1, distances[&(3, 2)]);
    assert_eq!(0, distances[&(4, 2)]);
    assert_eq!(1, distances[&(5, 2)]);
    assert_eq!(4, distances[&(1, 3)]);
    assert_eq!(3, distances[&(2, 3)]);
    assert_eq!(2, distances[&(3, 3)]);
    assert_eq!(2, distances[&(5, 3)]);
  }

  #[test]
  fn test_world_distances_from_with_unreachable_tiles() {
    let map = "
#######
#E..G.#
#...#.#
#.G.#G#
#######
";

    let world: World = map.trim().parse().unwrap();

    let distances: HashMap<(usize, usize), usize> = world.distances_from(1, 1);

    assert_eq!(8, distances.len());

    assert_eq!(0, distances[&(1, 1)]);
    assert_eq!(1, distances[&(2, 1)]);
    assert_eq!(2, distances[&(3, 1)]);
    assert_eq!(1, distances[&(1, 2)]);
    assert_eq!(2, distances[&(2, 2)]);
    assert_eq!(3, distances[&(3, 2)]);
    assert_eq!(2, distances[&(1, 3)]);
    assert_eq!(4, distances[&(3, 3)]);
  }
}

