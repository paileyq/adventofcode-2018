fn main() {
  let file = File::open("../input/input01")?;
  let reader = BufReader::new(file);

  for line in reader.lines() {
    println!("{}", line);
  }
}
