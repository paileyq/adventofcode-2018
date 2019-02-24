use std::cmp;
use std::cmp::Ordering;
use std::fmt;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::ops::Add;
use std::str::FromStr;

const DEFAULT_HEALTH: i32 = 200;
const DEFAULT_ATTACK_POWER: i32 = 3;

#[derive(PartialEq, Debug, Clone, Copy, Eq, Hash)]
struct Position(usize, usize);

impl Position {
  pub fn x(self) -> usize {
    return self.0;
  }

  pub fn y(self) -> usize {
    return self.1;
  }
}

impl Add<(isize, isize)> for Position {
  type Output = Position;

  fn add(self, (dx, dy): (isize, isize)) -> Position {
    Position(
      (self.x() as isize + dx) as usize,
      (self.y() as isize + dy) as usize,
    )
  }
}

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

#[derive(PartialEq, Debug, Clone, Copy)]
enum Team {
  Elf,
  Goblin,
}

#[derive(Debug)]
struct Unit {
  team: Team,
  position: Position,
  health: i32,
}

impl Unit {
  pub fn new(team: Team, position: Position) -> Unit {
    Unit { team, position, health: DEFAULT_HEALTH }
  }

  pub fn team(&self) -> Team {
    self.team
  }

  pub fn position(&self) -> Position {
    self.position
  }

  pub fn health(&self) -> i32 {
    self.health
  }
}

struct World {
  tiles: Vec<Tile>,
  width: usize,
  height: usize,
  units: Vec<Unit>,
}

impl World {
  pub fn tile(&self, position: Position) -> Option<Tile> {
    if position.x() >= self.width || position.y() >= self.height {
      return None;
    }

    Some(self.tiles[position.y() * self.width + position.x()])
  }

  pub fn distances_from(&self, source: Position) -> HashMap<Position, usize> {
    let mut distances = HashMap::new();
    let mut unvisited = HashSet::new();

    for tile_x in 0..self.width {
      for tile_y in 0..self.height {
        let pos = Position(tile_x, tile_y);
        if self.tile(pos) == Some(Tile::Empty) {
          unvisited.insert(pos);
        }
      }
    }

    distances.insert(source, 0);

    let mut current = source;
    loop {
      let next_distance = distances[&current] + 1;

      for &direction in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let neighbor = current + direction;

        if self.tile(neighbor) == Some(Tile::Empty) {
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
    let mut tiles = Vec::new();
    let mut units = Vec::new();

    let mut x = 0;
    let mut y = 0;
    for c in s.chars() {
      if let Some(tile) = Tile::from_char(c) {
        tiles.push(tile);

        match tile {
          Tile::Elf    => units.push(Unit::new(Team::Elf, Position(x, y))),
          Tile::Goblin => units.push(Unit::new(Team::Goblin, Position(x, y))),
          _ => (),
        };

        x += 1;
      } else if c == '\n' {
        y += 1;
        x = 0;
      }
    }

    let height = y + 1;

    if tiles.len() != width * height {
      return Err("invalid world map string");
    }

    Ok(World { tiles, width, height, units })
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

      let mut units = self.units.iter()
        .filter(|unit| unit.position().y() == y)
        .collect::<Vec<_>>();

      units.sort_by_key(|unit| unit.position().x());

      for (index, unit) in units.iter().enumerate() {
        write!(f, "{}{}({})",
          if index == 0 { "   " } else { ", " },
          match unit.team() { Team::Elf => "E", Team::Goblin => "G" },
          unit.health())?;
      }

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

    assert_eq!(world.tile(Position(1, 3)), Some(Tile::Empty));
    assert_eq!(world.tile(Position(0, 0)), Some(Tile::Wall));
    assert_eq!(world.tile(Position(6, 4)), Some(Tile::Wall));
    assert_eq!(world.tile(Position(1, 1)), Some(Tile::Elf));
    assert_eq!(world.tile(Position(5, 3)), Some(Tile::Goblin));
    assert_eq!(world.tile(Position(7, 3)), None);
    assert_eq!(world.tile(Position(0, 5)), None);

    let expected = "
#######
#E..G.#   E(200), G(200)
#...#.#
#.G.#G#   G(200), G(200)
#######
";

    assert_eq!(expected.trim(), format!("{}", world));
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

    let distances = world.distances_from(Position(4, 2));

    assert_eq!(13, distances.len());

    assert_eq!(4, distances[&Position(1, 1)]);
    assert_eq!(2, distances[&Position(3, 1)]);
    assert_eq!(1, distances[&Position(4, 1)]);
    assert_eq!(2, distances[&Position(5, 1)]);
    assert_eq!(3, distances[&Position(1, 2)]);
    assert_eq!(2, distances[&Position(2, 2)]);
    assert_eq!(1, distances[&Position(3, 2)]);
    assert_eq!(0, distances[&Position(4, 2)]);
    assert_eq!(1, distances[&Position(5, 2)]);
    assert_eq!(4, distances[&Position(1, 3)]);
    assert_eq!(3, distances[&Position(2, 3)]);
    assert_eq!(2, distances[&Position(3, 3)]);
    assert_eq!(2, distances[&Position(5, 3)]);
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

    let distances = world.distances_from(Position(1, 1));

    assert_eq!(8, distances.len());

    assert_eq!(0, distances[&Position(1, 1)]);
    assert_eq!(1, distances[&Position(2, 1)]);
    assert_eq!(2, distances[&Position(3, 1)]);
    assert_eq!(1, distances[&Position(1, 2)]);
    assert_eq!(2, distances[&Position(2, 2)]);
    assert_eq!(3, distances[&Position(3, 2)]);
    assert_eq!(2, distances[&Position(1, 3)]);
    assert_eq!(4, distances[&Position(3, 3)]);
  }
}

