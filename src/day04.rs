use chrono::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Event {
  BeginShift(u32),
  FallAsleep,
  WakeUp
}

#[derive(Debug)]
struct LogEntry {
  timestamp: DateTime<Utc>,
  event: Event
}

impl FromStr for LogEntry {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, ()> {
    lazy_static! {
      static ref LINE_REGEX: Regex =
        Regex::new(r"^\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] (.+)$").unwrap();
      static ref BEGIN_SHIFT_REGEX: Regex =
        Regex::new(r"^Guard #(\d+) begins shift$").unwrap();
      static ref FALL_ASLEEP_REGEX: Regex =
        Regex::new(r"^falls asleep$").unwrap();
      static ref WAKE_UP_REGEX: Regex =
        Regex::new(r"^wakes up$").unwrap();
    }

    if let Some(caps) = LINE_REGEX.captures(s) {
      let year:   i32 = caps.get(1).unwrap().as_str().parse().unwrap();
      let month:  u32 = caps.get(2).unwrap().as_str().parse().unwrap();
      let day:    u32 = caps.get(3).unwrap().as_str().parse().unwrap();
      let hour:   u32 = caps.get(4).unwrap().as_str().parse().unwrap();
      let minute: u32 = caps.get(5).unwrap().as_str().parse().unwrap();

      let timestamp = Utc.ymd(year, month, day).and_hms(hour, minute, 0);

      let event_str = caps.get(6).unwrap().as_str();
      if let Some(caps) = BEGIN_SHIFT_REGEX.captures(event_str) {
        let guard_id: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
        Ok(LogEntry { timestamp, event: Event::BeginShift(guard_id) })
      } else if FALL_ASLEEP_REGEX.is_match(event_str) {
        Ok(LogEntry { timestamp, event: Event::FallAsleep })
      } else if WAKE_UP_REGEX.is_match(event_str) {
        Ok(LogEntry { timestamp, event: Event::WakeUp })
      } else {
        Err(())
      }
    } else {
      Err(())
    }
  }
}

#[derive(Debug)]
struct Nap {
  start: DateTime<Utc>,
  end: DateTime<Utc>
}

impl Nap {
  pub fn len(&self) -> chrono::Duration {
    self.end - self.start
  }
}

pub fn solve(input_file: File) {
  let reader = BufReader::new(input_file);

  let mut log_entries: Vec<LogEntry> = reader
    .lines()
    .flatten()
    .map(|line| line.parse())
    .flatten()
    .collect();

  log_entries.sort_by_key(|e| e.timestamp);

  let mut naps_by_guard_id: HashMap<u32, Vec<Nap>> = HashMap::new();
  let mut current_guard_id = None;
  let mut nap_start = None;

  for log_entry in log_entries.iter() {
    match log_entry.event {
      Event::BeginShift(guard_id) => {
        current_guard_id = Some(guard_id);
      },
      Event::FallAsleep => {
        nap_start = Some(log_entry.timestamp);
      },
      Event::WakeUp => {
        naps_by_guard_id
          .entry(current_guard_id.unwrap())
          .or_default()
          .push(Nap {
            start: nap_start.unwrap(),
            end: log_entry.timestamp
          });

        nap_start = None;
      }
    }
  }

  let guard_id = naps_by_guard_id.iter()
    .max_by_key(|(_, naps)| naps.iter()
      .map(|nap| nap.len().num_minutes())
      .sum::<i64>())
    .unwrap()
    .0;

  println!("{}", guard_id);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_log_entry_parse() {
    assert_eq!(
      "[1518-10-31 23:28] Guard #10 begins shift".parse::<LogEntry>().unwrap(),
      LogEntry {
        timestamp: Utc.ymd(1518, 10, 31).and_hms(23, 28, 0),
        event: Event::BeginShift(10)
      }
    );
    assert_eq!(
      "[1518-11-01 00:05] falls asleep".parse::<LogEntry>().unwrap(),
      LogEntry {
        timestamp: Utc.ymd(1518, 11, 1).and_hms(0, 5, 0),
        event: Event::FallAsleep
      }
    );
    assert_eq!(
      "[1518-11-01 00:25] wakes up".parse::<LogEntry>().unwrap(),
      LogEntry {
        timestamp: Utc.ymd(1518, 11, 1).and_hms(0, 25, 0),
        event: Event::WakeUp
      }
    );
  }
}
