use std::env;
use std::io;
use std::fs::File;
use std::process;

mod day01;

fn main() -> io::Result<()> {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 || args.len() > 3 {
    println!("Usage: {} <day number> [input file]", &args[0]);
    process::exit(1);
  }

  let day_number: u8 = args[1].parse()
    .expect("first argument must be a number");

  let file = match args.len() {
    3 => File::open(&args[2]),
    _ => File::open(format!("input/input{:02}", day_number))
  }.expect("input file doesn't exist");

  match day_number {
    1 => day01::solve(file),
    _ => panic!("Day {} not implemented yet", day_number)
  };

  Ok(())
}

