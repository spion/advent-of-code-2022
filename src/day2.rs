use anyhow::{anyhow, Result};
use std::{cmp::Ordering, io::stdin, str::FromStr};

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Move {
  Rock,
  Paper,
  Scissors,
}

impl quickcheck::Arbitrary for Move {
  fn arbitrary(g: &mut quickcheck::Gen) -> Self {
    g.choose(&[Move::Rock, Move::Paper, Move::Scissors])
      .unwrap()
      .clone()
  }
}
const LESS_THAN: [(&Move, &Move); 3] = [
  (&Move::Rock, &Move::Paper),
  (&Move::Paper, &Move::Scissors),
  (&Move::Scissors, &Move::Rock),
];

impl Move {
  fn compare(&self, other: &Move) -> Ordering {
    if self == other {
      Ordering::Equal
    } else if LESS_THAN.contains(&(self, other)) {
      Ordering::Less
    } else {
      Ordering::Greater
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  quickcheck! {
      fn symmetry(a: Move, b: Move) -> bool {
        let a_to_b = a.compare(&b);
        let b_to_a = b.compare(&a);
        if a_to_b == b_to_a { a_to_b == Ordering::Equal }
        else if a_to_b == Ordering::Less { b_to_a == Ordering::Greater }
        else { a_to_b == Ordering::Greater && b_to_a == Ordering::Less }
      }
  }
}

impl FromStr for Move {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self> {
    match s {
      "A" => Ok(Move::Rock),
      "B" => Ok(Move::Paper),
      "C" => Ok(Move::Scissors),
      "X" => Ok(Move::Rock),
      "Y" => Ok(Move::Paper),
      "Z" => Ok(Move::Scissors),
      _ => Err(anyhow!("Failed to parse move")),
    }
  }
}

fn parse_game(line: &str) -> Result<(Move, Ordering)> {
  let v = line.split(" ").collect::<Vec<_>>();
  let other_move = v[0].parse::<Move>()?;

  let outcome = match v[1] {
    "X" => Ok(Ordering::Less),
    "Y" => Ok(Ordering::Equal),
    "Z" => Ok(Ordering::Greater),
    _ => Err(anyhow!("{} is not a valid game outcome", v[1])),
  }?;

  Ok((other_move, outcome))
}

fn invert_move(game: &(Move, Ordering)) -> Move {
  let (other_move, outcome) = game;

  if outcome == &Ordering::Equal {
    other_move.clone()
  } else if outcome == &Ordering::Less {
    LESS_THAN
      .into_iter()
      .find_map(|(m1, m2)| if m2 == other_move { Some(m1) } else { None })
      .unwrap()
      .clone()
  } else {
    LESS_THAN
      .into_iter()
      .find_map(|(m1, m2)| if m1 == other_move { Some(m2) } else { None })
      .unwrap()
      .clone()
  }
}

fn day2() -> Result<()> {
  let lines: Vec<String> = stdin().lines().map(|l| l.unwrap()).collect();
  let result: u32 = lines
    .iter()
    .map(|l| {
      l.split(" ")
        .map(|m| m.parse::<Move>().unwrap())
        .collect::<Vec<_>>()
    })
    .map(|v| {
      let game_score = match v[1].compare(&v[0]) {
        Ordering::Less => 0,
        Ordering::Equal => 3,
        Ordering::Greater => 6,
      };

      let move_score = match v[1] {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
      };

      // println!(
      //   "they({:?}) me({:?}) move_score={} game_score={}",
      //   v[0], v[1], move_score, game_score
      // );
      return game_score + move_score;
    })
    .sum();

  println!("{}", result);
  Ok(())
}

fn day2_pt2() -> Result<()> {
  let lines: Vec<String> = stdin().lines().map(|l| l.unwrap()).collect();
  let result: u32 = lines
    .iter()
    .map(|l| parse_game(l).unwrap())
    .map(|g| {
      let outcome = g.1;
      let my_move = invert_move(&g);
      // let game_score =
      let game_score = match outcome {
        Ordering::Less => 0,
        Ordering::Equal => 3,
        Ordering::Greater => 6,
      };

      let move_score = match my_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
      };

      return game_score + move_score;
    })
    .sum();

  println!("{}", result);
  Ok(())
}

fn main() -> Result<()> {
  day2_pt2()
}
