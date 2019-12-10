use crate::computer;
use computer::{ComputerState, ComputerWord};

fn operation_stack(param1: i32, param2: i32) -> ComputerState {
    let modules_text = include_str!("resources/day02.txt");
    let mut operations = computer::parse_op_stack(modules_text);

    operations[1] = ComputerWord::from(param1);
    operations[2] = ComputerWord::from(param2);

    return operations;
}

fn solve_for(solution: i32) {
    let solution = ComputerWord::from(solution);
    let stack = operation_stack(0, 0);
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut stack = stack.clone();
            stack[1] = ComputerWord::from(noun);
            stack[2] = ComputerWord::from(verb);
            let mut computer = computer::Computer::new(stack);
            let result = computer.eval();
            if result == &solution {
                println!("Solved: {}", 100 * noun + verb);
                return;
            }
        }
    }
    panic!("Can't solve");
}

pub fn solve() {
    let stack = operation_stack(12, 2);
    let mut computer = computer::Computer::new(stack);
    println!("Part 1: {}", computer.eval());
    solve_for(19690720);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(
            computer::from("2,4,4,0,99,0").eval(),
            &ComputerWord::from(9801)
        );
        assert_eq!(
            computer::from("1,1,1,4,99,5,6,0,99").eval(),
            &ComputerWord::from(30)
        );
    }
}
