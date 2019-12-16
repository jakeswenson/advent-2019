mod computer;
mod coordinates;
mod days;
mod debug;

pub use days::*;

fn main() {
  println!("-------------\n  Day 01\n-------------");
  day01::solve();

  println!("-------------\n  Day 02\n-------------");

  day02::solve();

  println!("-------------\n  Day 03\n-------------");
  day03::solve();

  println!("-------------\n  Day 04\n-------------");
  day04::solve();

  println!("-------------\n  Day 05\n-------------");
  day05::solve();

  println!("-------------\n  Day 06\n-------------");
  day06::solve();

  println!("-------------\n  Day 07\n-------------");
  day07::solve();

  println!("-------------\n  Day 08\n-------------");
  day08::solve();

  println!("-------------\n  Day 09\n-------------");
  day09::solve();

  println!("-------------\n  Day 10\n-------------");
  day10::solve();
}
