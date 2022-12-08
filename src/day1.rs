use anyhow::Result;
use std::io::stdin;

// fn first() -> Result<()> {
//   let mut max = 0;
//   let mut sum = 0;
//   for maybe_line in stdin().lines() {
//     let line = maybe_line?;
//     if line.len() > 0 {
//       let num: u32 = line.parse()?;
//       sum += num;
//     } else {
//       if sum > max {
//         max = sum;
//       }
//       sum = 0;
//     }
//   }
//   println!("{}", max);

//   Ok(())
// }

fn second() -> Result<()> {
  let lines: Vec<String> = stdin().lines().map(|l| l.unwrap()).collect();

  let mut elf_lines: Vec<u32> = lines
    .split(|line| line.is_empty())
    .into_iter()
    .map(|elf| elf.iter().map(|item| item.parse::<u32>().unwrap()).sum())
    .collect();

  elf_lines.sort();

  elf_lines.reverse();

  println!("{:?}", elf_lines.iter().take(3).sum::<u32>());

  Ok(())
}

fn main() -> Result<()> {
  second()
}
