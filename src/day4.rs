use anyhow::Result;
use std::{
  cmp::{max, min},
  io::stdin,
};

fn parse_range(range: &str) -> Result<(i32, i32)> {
  let (start, end) = range.split_once('-').unwrap();

  Ok((start.parse()?, end.parse()?))
}

fn range_overlap(r1: &(i32, i32), r2: &(i32, i32)) -> i32 {
  min(r1.1, r2.1) - max(r1.0, r2.0)
}

fn range_size(r: &(i32, i32)) -> i32 {
  r.1 - r.0
}

fn print_overlaps(any_overlap: bool) -> Result<()> {
  let res = stdin()
    .lines()
    .map(|l| {
      let text = l.unwrap();
      let ranges = text.split_once(",").unwrap();
      (
        parse_range(ranges.0).unwrap(),
        parse_range(ranges.1).unwrap(),
      )
    })
    .filter(|(r1, r2)| {
      if any_overlap {
        range_overlap(r1, r2) >= 0
      } else {
        range_overlap(r1, r2) >= min(range_size(r1), range_size(r2))
      }
    })
    .count();

  println!("{}", res);
  Ok(())
}

fn main() -> Result<()> {
  print_overlaps(true)
}
