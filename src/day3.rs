use anyhow::Result;
use std::collections::HashSet;
use std::io::stdin;

fn item_to_code(cr: &char) -> u32 {
  let c = cr.clone();

  if c >= 'a' && c <= 'z' {
    c as u32 - 'a' as u32 + 1
  } else {
    c as u32 - 'A' as u32 + 27
  }
}

fn part1() -> Result<()> {
  let lines: Vec<String> = stdin().lines().map(|l| l.unwrap()).collect();
  let mut sum = 0;

  for line in lines {
    let half = line.len() / 2;
    let (left, right) = line.split_at(half);

    // println!("{}, {}", left.len(), right.len());
    let left_set: HashSet<_> = left.chars().collect();
    let right_set: HashSet<_> = right.chars().collect();

    sum += item_to_code(left_set.intersection(&right_set).next().unwrap());
  }

  println!("{}", sum);
  Ok(())
}

fn part2() -> Result<()> {
  let lines: Vec<String> = stdin().lines().map(|l| l.unwrap()).collect();
  let mut sum = 0;

  for line_chunks in lines.chunks(3) {
    let res = line_chunks
      .into_iter()
      .map(|l| l.chars().collect::<HashSet<_>>())
      .reduce(|a, b| a.intersection(&b).cloned().collect::<HashSet<_>>())
      .unwrap()
      .into_iter()
      .next()
      .unwrap();

    sum += item_to_code(&res);
  }

  println!("{}", sum);
  Ok(())
}

fn main() -> Result<()> {
  if false {
    part1()
  } else {
    part2()
  }
}
