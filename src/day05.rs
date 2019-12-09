use crate::computer;
fn part1() {
    let day = include_str!("resources/day05.txt");
    let mut comp = computer::from(day).add_input(1);
    let i = comp.eval();
}

fn part2() {}

pub fn solve() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1_rules() {
        use crate::computer::{from, Computer};
        let mut computer = from("1002,4,3,4,33");
        assert_eq!(computer.run(4), 99);
    }

    #[test]
    fn test_part2_rules() {}
}
