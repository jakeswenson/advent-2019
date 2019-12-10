fn part1() {}

fn part2() {}

pub fn solve() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use crate::computer;
    #[test]
    fn test_large_number() {
        assert_eq!(
            computer::run_single_input("104,1125899906842624,99", 0),
            1125899906842624
        );
    }

    #[test]
    fn test_large_number_operations() {
        assert_eq!(
            computer::run_single_input("1102,34915192,34915192,7,4,7,99,0", 0),
            1219070632396864
        );
    }

    #[test]
    fn test_relative_adjustments() {
        assert_eq!(
            computer::run_single_input(
                "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
                0
            ),
            99
        );
    }

    #[test]
    fn test_part2_rules() {}
}
