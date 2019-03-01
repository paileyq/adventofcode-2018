use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::error::Error;
use std::io::BufReader;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
struct State(i32, i32, i32, i32);

impl State {
  pub fn reg(self, index: i32) -> i32 {
    match index {
      0 => self.0,
      1 => self.1,
      2 => self.2,
      3 => self.3,
      _ => unreachable!(),
    }
  }

  pub fn set_reg(self, index: i32, value: i32) -> State {
    let mut state = self;

    match index {
      0 => state.0 = value,
      1 => state.1 = value,
      2 => state.2 = value,
      3 => state.3 = value,
      _ => unreachable!(),
    }

    state
  }

  pub fn addr(self, reg_a: i32, reg_b: i32, reg_out: i32) -> State {
    let result = self.reg(reg_a) + self.reg(reg_b);
    self.set_reg(reg_out, result)
  }

  pub fn addi(self, reg_a: i32, val_b: i32, reg_out: i32) -> State {
    let result = self.reg(reg_a) + val_b;
    self.set_reg(reg_out, result)
  }

  pub fn mulr(self, reg_a: i32, reg_b: i32, reg_out: i32) -> State {
    let result = self.reg(reg_a) * self.reg(reg_b);
    self.set_reg(reg_out, result)
  }

  pub fn muli(self, reg_a: i32, val_b: i32, reg_out: i32) -> State {
    let result = self.reg(reg_a) * val_b;
    self.set_reg(reg_out, result)
  }

  pub fn banr(self, reg_a: i32, reg_b: i32, reg_out: i32) -> State {
    let result = self.reg(reg_a) & self.reg(reg_b);
    self.set_reg(reg_out, result)
  }

  pub fn bani(self, reg_a: i32, val_b: i32, reg_out: i32) -> State {
    let result = self.reg(reg_a) & val_b;
    self.set_reg(reg_out, result)
  }

  pub fn borr(self, reg_a: i32, reg_b: i32, reg_out: i32) -> State {
    let result = self.reg(reg_a) | self.reg(reg_b);
    self.set_reg(reg_out, result)
  }

  pub fn bori(self, reg_a: i32, val_b: i32, reg_out: i32) -> State {
    let result = self.reg(reg_a) | val_b;
    self.set_reg(reg_out, result)
  }

  pub fn setr(self, reg_a: i32, _b: i32, reg_out: i32) -> State {
    self.set_reg(reg_out, self.reg(reg_a))
  }

  pub fn seti(self, val_a: i32, _b: i32, reg_out: i32) -> State {
    self.set_reg(reg_out, val_a)
  }

  pub fn gtir(self, val_a: i32, reg_b: i32, reg_out: i32) -> State {
    let result = if val_a > self.reg(reg_b) { 1 } else { 0 };
    self.set_reg(reg_out, result)
  }

  pub fn gtri(self, reg_a: i32, val_b: i32, reg_out: i32) -> State {
    let result = if self.reg(reg_a) > val_b { 1 } else { 0 };
    self.set_reg(reg_out, result)
  }

  pub fn gtrr(self, reg_a: i32, reg_b: i32, reg_out: i32) -> State {
    let result = if self.reg(reg_a) > self.reg(reg_b) { 1 } else { 0 };
    self.set_reg(reg_out, result)
  }

  pub fn eqir(self, val_a: i32, reg_b: i32, reg_out: i32) -> State {
    let result = if val_a == self.reg(reg_b) { 1 } else { 0 };
    self.set_reg(reg_out, result)
  }

  pub fn eqri(self, reg_a: i32, val_b: i32, reg_out: i32) -> State {
    let result = if self.reg(reg_a) == val_b { 1 } else { 0 };
    self.set_reg(reg_out, result)
  }

  pub fn eqrr(self, reg_a: i32, reg_b: i32, reg_out: i32) -> State {
    let result = if self.reg(reg_a) == self.reg(reg_b) { 1 } else { 0 };
    self.set_reg(reg_out, result)
  }
}

impl FromStr for State {
  type Err = Box<dyn Error>;

  fn from_str(string: &str) -> Result<State, Self::Err> {
    lazy_static! {
      static ref STATE_REGEX: Regex =
        Regex::new(r"^(Before|After): *\[(\d+), (\d+), (\d+), (\d+)\]$").unwrap();
    }

    let caps = STATE_REGEX.captures(string)
      .ok_or("Invalid state string")?;

    Ok(State(
      caps.get(2).unwrap().as_str().parse()?,
      caps.get(3).unwrap().as_str().parse()?,
      caps.get(4).unwrap().as_str().parse()?,
      caps.get(5).unwrap().as_str().parse()?,
    ))
  }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Instruction {
  opcode: i32,
  a: i32,
  b: i32,
  c: i32,
}

impl Instruction {
  pub fn possible_opcodes(self, before: State, after: State) -> i32 {
    let Instruction { a, b, c, .. } = self;
    let mut count = 0;

    if before.addr(a, b, c) == after { count += 1; }
    if before.addi(a, b, c) == after { count += 1; }
    if before.mulr(a, b, c) == after { count += 1; }
    if before.muli(a, b, c) == after { count += 1; }
    if before.banr(a, b, c) == after { count += 1; }
    if before.bani(a, b, c) == after { count += 1; }
    if before.borr(a, b, c) == after { count += 1; }
    if before.bori(a, b, c) == after { count += 1; }
    if before.setr(a, b, c) == after { count += 1; }
    if before.seti(a, b, c) == after { count += 1; }
    if before.gtir(a, b, c) == after { count += 1; }
    if before.gtri(a, b, c) == after { count += 1; }
    if before.gtrr(a, b, c) == after { count += 1; }
    if before.eqir(a, b, c) == after { count += 1; }
    if before.eqri(a, b, c) == after { count += 1; }
    if before.eqrr(a, b, c) == after { count += 1; }

    count
  }
}

impl FromStr for Instruction {
  type Err = Box<dyn Error>;

  fn from_str(string: &str) -> Result<Instruction, Self::Err> {
    lazy_static! {
      static ref STATE_REGEX: Regex =
        Regex::new(r"^(\d+) (\d+) (\d+) (\d+)$").unwrap();
    }

    let caps = STATE_REGEX.captures(string)
      .ok_or("Invalid instruction string")?;

    Ok(Instruction {
      opcode: caps.get(1).unwrap().as_str().parse()?,
      a: caps.get(2).unwrap().as_str().parse()?,
      b: caps.get(3).unwrap().as_str().parse()?,
      c: caps.get(4).unwrap().as_str().parse()?,
    })
  }
}

pub fn solve(input_file: File) {
  let reader = BufReader::new(input_file);
  let mut lines = reader.lines();

  let mut num_samples_behaving_like_three_or_more_opcodes = 0;
  loop {
    if let Some(Ok(line)) = lines.next() {
      if line.is_empty() { continue }
      if let Ok(before_state) = line.parse::<State>() {
        let instruction = lines.next().unwrap().unwrap().parse::<Instruction>().unwrap();
        let after_state = lines.next().unwrap().unwrap().parse::<State>().unwrap();

        if instruction.possible_opcodes(before_state, after_state) >= 3 {
          num_samples_behaving_like_three_or_more_opcodes += 1;
        }
      } else {
        break;
      }
    } else {
      break;
    }
  }

  println!("Number of samples behaving like 3 or more opcodes: {}", num_samples_behaving_like_three_or_more_opcodes);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_reg() {
    let state = State(2, 3, 6, 7);

    assert_eq!(state.reg(0), 2);
    assert_eq!(state.reg(1), 3);
    assert_eq!(state.reg(2), 6);
    assert_eq!(state.reg(3), 7);
  }

  #[test]
  fn test_set_reg() {
    let state = State(0, 0, 0, 0);

    assert_eq!(state.set_reg(0, 42), State(42, 0, 0, 0));
    assert_eq!(state.set_reg(1, 42), State(0, 42, 0, 0));
    assert_eq!(state.set_reg(2, 42), State(0, 0, 42, 0));
    assert_eq!(state.set_reg(3, 42), State(0, 0, 0, 42));
  }

  #[test]
  fn test_addr() {
    let state = State(2, 3, 6, 7);

    assert_eq!(state.addr(2, 3, 1), State(2, 13, 6, 7));
    assert_eq!(state.addr(1, 1, 1), State(2, 6, 6, 7));
    assert_eq!(state.addr(0, 2, 2), State(2, 3, 8, 7));
  }

  #[test]
  fn test_addi() {
    let state = State(2, 3, 6, 7);

    assert_eq!(state.addi(2, 3, 1), State(2, 9, 6, 7));
    assert_eq!(state.addi(1, 1, 1), State(2, 4, 6, 7));
    assert_eq!(state.addi(3, 0, 0), State(7, 3, 6, 7));
  }

  #[test]
  fn test_mulr() {
    let state = State(2, 3, 6, 7);

    assert_eq!(state.mulr(2, 3, 1), State(2, 42, 6, 7));
    assert_eq!(state.mulr(1, 1, 1), State(2, 9, 6, 7));
    assert_eq!(state.mulr(0, 2, 2), State(2, 3, 12, 7));
  }

  #[test]
  fn test_muli() {
    let state = State(2, 3, 6, 7);

    assert_eq!(state.muli(2, 3, 1), State(2, 18, 6, 7));
    assert_eq!(state.muli(1, 1, 1), State(2, 3, 6, 7));
    assert_eq!(state.muli(3, 0, 0), State(0, 3, 6, 7));
  }

  #[test]
  fn test_banr() {
    let state = State(2, 3, 6, 7);

    assert_eq!(state.banr(2, 3, 1), State(2, 6, 6, 7));
    assert_eq!(state.banr(1, 1, 1), State(2, 3, 6, 7));
    assert_eq!(state.banr(0, 2, 2), State(2, 3, 2, 7));
  }

  #[test]
  fn test_bani() {
    let state = State(2, 3, 6, 7);

    assert_eq!(state.bani(2, 3, 1), State(2, 2, 6, 7));
    assert_eq!(state.bani(1, 1, 1), State(2, 1, 6, 7));
    assert_eq!(state.bani(3, 0, 0), State(0, 3, 6, 7));
  }

  #[test]
  fn test_borr() {
    let state = State(2, 3, 6, 7);

    assert_eq!(state.borr(2, 3, 1), State(2, 7, 6, 7));
    assert_eq!(state.borr(1, 1, 1), State(2, 3, 6, 7));
    assert_eq!(state.borr(0, 2, 2), State(2, 3, 6, 7));
  }

  #[test]
  fn test_bori() {
    let state = State(2, 3, 6, 7);

    assert_eq!(state.bori(2, 3, 1), State(2, 7, 6, 7));
    assert_eq!(state.bori(1, 1, 1), State(2, 3, 6, 7));
    assert_eq!(state.bori(3, 0, 0), State(7, 3, 6, 7));
  }

  #[test]
  fn test_setr() {
    let state = State(2, 3, 6, 7);

    assert_eq!(state.setr(2, 0, 1), State(2, 6, 6, 7));
    assert_eq!(state.setr(1, 0, 1), State(2, 3, 6, 7));
    assert_eq!(state.setr(3, 0, 0), State(7, 3, 6, 7));
  }

  #[test]
  fn test_seti() {
    let state = State(2, 3, 6, 7);

    assert_eq!(state.seti(0, 0, 0), State(0, 3, 6, 7));
    assert_eq!(state.seti(5, 0, 1), State(2, 5, 6, 7));
    assert_eq!(state.seti(8, 0, 2), State(2, 3, 8, 7));
    assert_eq!(state.seti(9, 0, 3), State(2, 3, 6, 9));
  }

  #[test]
  fn test_gtir() {
    let state = State(2, 3, 6, 7);

    assert_eq!(state.gtir(5, 2, 0), State(0, 3, 6, 7));
    assert_eq!(state.gtir(4, 1, 0), State(1, 3, 6, 7));
    assert_eq!(state.gtir(7, 3, 3), State(2, 3, 6, 0));
  }

  #[test]
  fn test_gtri() {
    let state = State(2, 3, 6, 7);

    assert_eq!(state.gtri(2, 5, 0), State(1, 3, 6, 7));
    assert_eq!(state.gtri(1, 4, 0), State(0, 3, 6, 7));
    assert_eq!(state.gtri(3, 7, 3), State(2, 3, 6, 0));
  }

  #[test]
  fn test_gtrr() {
    let state = State(2, 3, 6, 7);

    assert_eq!(state.gtrr(1, 2, 0), State(0, 3, 6, 7));
    assert_eq!(state.gtrr(2, 1, 0), State(1, 3, 6, 7));
    assert_eq!(state.gtrr(3, 3, 3), State(2, 3, 6, 0));
  }

  #[test]
  fn test_eqir() {
    let state = State(2, 3, 6, 7);

    assert_eq!(state.eqir(5, 2, 0), State(0, 3, 6, 7));
    assert_eq!(state.eqir(3, 1, 0), State(1, 3, 6, 7));
    assert_eq!(state.eqir(7, 3, 3), State(2, 3, 6, 1));
  }

  #[test]
  fn test_eqri() {
    let state = State(2, 3, 6, 7);

    assert_eq!(state.eqri(2, 5, 0), State(0, 3, 6, 7));
    assert_eq!(state.eqri(1, 3, 0), State(1, 3, 6, 7));
    assert_eq!(state.eqri(3, 7, 3), State(2, 3, 6, 1));
  }

  #[test]
  fn test_eqrr() {
    let state = State(7, 3, 6, 7);

    assert_eq!(state.eqrr(1, 2, 0), State(0, 3, 6, 7));
    assert_eq!(state.eqrr(0, 3, 0), State(1, 3, 6, 7));
    assert_eq!(state.eqrr(3, 3, 3), State(7, 3, 6, 1));
  }

  #[test]
  fn test_state_parse() {
    assert_eq!(
      "Before: [2, 3, 6, 78]".parse::<State>().unwrap(),
      State(2, 3, 6, 78)
    );
    assert_eq!(
      "After:  [0, 32, 68, 1]".parse::<State>().unwrap(),
      State(0, 32, 68, 1)
    );
  }

  #[test]
  fn test_instruction_parse() {
    assert_eq!(
      "14 26 1 0".parse::<Instruction>().unwrap(),
      Instruction { opcode: 14, a: 26, b: 1, c: 0 }
    );
  }

  #[test]
  fn test_possible_opcodes() {
    let instruction = Instruction { opcode: 9, a: 2, b: 1, c: 2 };
    let before = State(3, 2, 1, 1);
    let after = State(3, 2, 2, 1);

    assert_eq!(instruction.possible_opcodes(before, after), 3);
  }
}

