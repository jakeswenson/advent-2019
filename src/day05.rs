use crate::computer;

fn part1() {
    let day = include_str!("resources/day05.txt");
    let comp = computer::from(day).add_input(1);
    let outputs = comp.run();
    for (idx, output) in outputs.iter().enumerate() {
        println!("[{}] {}", idx, output)
    }
}

fn part2() {
    let day = include_str!("resources/day05.txt");
    let comp = computer::from(day).add_input(5);
    println!("States: {}", comp.len());
    let outputs = comp.run();
    for (idx, output) in outputs.iter().enumerate() {
        println!("[{}] {}", idx, output)
    }
}

pub fn solve() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use crate::computer::{from, ComputerWord};
    use num::ToPrimitive;

    #[test]
    fn test_part1_rules() {
        let mut computer = from("1002,4,3,4,33");
        assert_eq!(computer.eval_at(4).clone(), 99.into());
    }

    #[test]
    fn test_part2() {
        fn single_input(program: &str, input: i32, expected: i32) {
            let single_output = from(program).add_input(input).run().pop().unwrap();
            assert_eq!(
                single_output,
                ComputerWord::from(expected),
                "output: {}",
                single_output
            );
        }

        single_input("3,0,1008,0,8,0,4,0,99", 8, 1);

        single_input("3,9,8,9,10,9,4,9,99,-1,8", 8, 1);
        single_input("3,9,8,9,10,9,4,9,99,-1,8", 3, 0);

        single_input("3,9,7,9,10,9,4,9,99,-1,8", 3, 1);
        single_input("3,9,7,9,10,9,4,9,99,-1,8", 8, 0);
        single_input("3,9,7,9,10,9,4,9,99,-1,8", 9, 0);

        // immediate mode stuff
        single_input("3,3,1108,-1,8,3,4,3,99", 8, 1);
        single_input("3,3,1108,-1,8,3,4,3,99", 0, 0);

        single_input("3,3,1107,-1,8,3,4,3,99", 7, 1);
        single_input("3,3,1107,-1,8,3,4,3,99", 8, 0);

        // Jumps
        single_input("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 0, 0);
        single_input("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 1, 1);

        single_input("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 0, 0);
        single_input("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 1, 1);
    }

    #[test]
    fn test_part2_rules() {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let run = |i: i32| {
            from(input)
                .add_input(i)
                .run()
                .pop()
                .unwrap()
                .to_i32()
                .unwrap()
        };
        assert_eq!(run(5), 999);
        assert_eq!(run(8), 1000);
        assert_eq!(run(9), 1001);
    }
}
