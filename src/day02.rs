use std::str::FromStr;

fn operation_stack(param1: i32, param2: i32) -> Vec<i32> {
    let modules_text = include_str!("resources/day02.txt");
    let mut operations = parse_op_stack(modules_text);

    operations[1] = param1;
    operations[2] = param2;

    return operations;
}

fn parse_op_stack(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(i32::from_str)
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect()
}

#[derive(Debug, Copy, Clone)]
enum OpCode {
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
    Done,
}

trait InstructionSize {
    fn size(&self) -> usize;
}

impl InstructionSize for OpCode {
    fn size(&self) -> usize {
        match self {
            OpCode::Add(_, _, _) | OpCode::Mul(_, _, _) => 4,
            OpCode::Done => 1,
        }
    }
}

fn interpret(op: OpCode, stack: &mut Vec<i32>) {
    match op {
        OpCode::Add(op1, op2, dest) => {
            let x = stack[op1];
            let y = stack[op2];
            let result = x + y;
            //println!("{dest} = {} ({} + {})", result, x, y, dest = dest);
            stack[dest] = result;
        }
        OpCode::Mul(op1, op2, dest) => {
            let x = stack[op1];
            let y = stack[op2];
            let result = x * y;
            //println!("{dest} = {} ({} * {})", result, x, y, dest = dest);
            stack[dest] = result;
        }
        OpCode::Done => {}
    }
}

impl OpCode {
    fn parse_eval(stack: &mut Vec<i32>) -> Vec<OpCode> {
        let mut ip = 0;

        let mut result = Vec::new();

        while ip < stack.len() {
            let op_code = OpCode::parse_op(ip, stack);
            result.push(op_code);
            interpret(op_code, stack);
            if op_code.is_done() {
                break;
            }
            ip += op_code.size();
        }

        return result;
    }

    fn parse_op(ip: usize, operations: &Vec<i32>) -> Self {
        match operations[ip] {
            1 => {
                let op1 = operations[ip + 1] as usize;
                let op2 = operations[ip + 2] as usize;
                let dest = operations[ip + 3] as usize;
                return OpCode::Add(op1, op2, dest);
            }
            2 => {
                let op1 = operations[ip + 1] as usize;
                let op2 = operations[ip + 2] as usize;
                let dest = operations[ip + 3] as usize;
                return OpCode::Mul(op1, op2, dest);
            }
            99 => return OpCode::Done,
            _ => panic!("Invalid opcode: {} @ {}", operations[ip], ip),
        }
    }

    fn is_done(&self) -> bool {
        match self {
            OpCode::Done => true,
            _ => false,
        }
    }
}

fn eval(stack: &mut Vec<i32>) -> i32 {
    OpCode::parse_eval(stack);
    return stack[0];
}

fn solve_for(solution: i32) {
    let stack = operation_stack(0, 0);
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut stack = stack.clone();
            stack[1] = noun;
            stack[2] = verb;
            let result = eval(&mut stack);
            if result == solution {
                println!("Solved: {}", 100 * noun + verb);
                return;
            }
        }
    }
    panic!("Can't solve");
}

pub fn solve() {
    println!("-------------\n  Day 02\n-------------");
    println!("Part 1: {}", eval(&mut operation_stack(12, 2)));
    solve_for(19690720);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(eval(&mut parse_op_stack("2,4,4,0,99,0")), 9801);
        assert_eq!(eval(&mut parse_op_stack("1,1,1,4,99,5,6,0,99")), 30);
    }
}
