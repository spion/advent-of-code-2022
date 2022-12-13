use std::io::stdin;

use anyhow::{Context, Result};
use itertools::Itertools;

#[derive(Debug)]
pub struct Move {
  count: usize,
  from: usize,
  to: usize,
}

peg::parser! {
  grammar move_parser() for str {
    pub rule move_parse() -> Move
      = "move " count:number() " from " from:number() " to " to:number() {
        Move { count, from, to }
      }

    rule number() -> usize
      = n:$(['0'..='9']+) {? n.parse().or(Err("Could not parse i32")) }
  }
}

fn parse_stack(stack: &String) -> Vec<String> {
  let parsed = stack
    .as_bytes()
    .chunks(4)
    .filter(|c| c.len() > 1)
    .map(|c| (c[1] as char).to_string())
    .collect_vec();

  println!("{} -> {:?}", stack, parsed);

  parsed
}
fn parse(move_line: &String) -> Result<Move> {
  move_parser::move_parse(move_line).with_context(|| format!("Culd not parse move {}", move_line))
}

fn reverse_all(stacks: &mut Vec<Vec<String>>) {
  for s in stacks {
    s.reverse();
  }
}

fn solve(crane_model: u32) -> Result<()> {
  let lines: Vec<String> = stdin().lines().map(|l| l.unwrap()).collect();

  let (stack_lines, move_lines) = lines.split(|x| x.trim().len() == 0).next_tuple().unwrap();

  let mut stacks: Vec<Vec<String>> = vec![];

  for (ix, line) in stack_lines.into_iter().map(parse_stack).enumerate() {
    if ix == 0 {
      stacks = vec![vec![]; line.len()];
    }
    if ix == line.len() - 1 {
      break;
    }
    for (stack_ix, box_val) in line.into_iter().enumerate() {
      if box_val.trim().len() > 0 {
        stacks[stack_ix].push(box_val);
      }
    }
  }

  reverse_all(&mut stacks);
  println!("{:?}", stacks);

  for maybe_move in move_lines.into_iter().map(parse) {
    let move_data = maybe_move?;
    let mut move_vec = vec![];

    for _repeat in 0..move_data.count {
      let src = &mut stacks[move_data.from - 1];
      move_vec.push(src.pop().with_context(|| {
        format!(
          "Stack move impossible {:?}\n{:?}",
          move_data, stacks[move_data.from]
        )
      })?)
    }
    if crane_model == 9000 {
      move_vec.reverse();
    }
    for _repeat in 0..move_data.count {
      let dst = &mut stacks[move_data.to - 1];
      dst.push(move_vec.pop().unwrap());
    }
  }

  let res = stacks.into_iter().map(|mut s| s.pop().unwrap()).join("");
  println!("{}", res);

  Ok(())
}

fn main() -> Result<()> {
  solve(9001)
}
