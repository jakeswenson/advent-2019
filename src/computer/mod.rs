use std::str::FromStr;
use std::collections::VecDeque;

pub fn parse_op_stack(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(i32::from_str)
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect()
}

type ComputerState = Vec<i32>;

struct Computer {
    input: VecDeque<i32>,
    output: Vec<i32>,
    state: ComputerState,
}

impl Computer {
    pub fn state_mut(&mut self) -> &mut ComputerState {
        &mut self.state
    }

    pub fn state(&self) -> &ComputerState {
        &self.state
    }

    pub fn new(state_vec: Vec<i32>) -> Self {
        Computer {
            input: VecDeque::new(),
            output: Vec::new(),
            state: state_vec,
        }
    }

    pub fn resolve(&self, reference: usize) -> i32 {
        self.state[reference]
    }

    pub fn op(&self, instruction_pointer: usize) -> OpReader {
        OpReader { instruction_pointer, state: &self.state}
    }
}

struct OpReader<'a> {
    instruction_pointer: usize,
    state: &'a ComputerState
}

impl OpReader {
    pub fn arg(&self, arg: usize) -> OpArg {

    }
}

enum OpArg {
    Literal(i32),
    Reference(usize),
}

impl OpArg {
    pub fn resolve(&self, computer: &Computer) -> i32 {
        match self {
            OpArg::Literal(lit) => lit,
            OpArg::Reference(loc) => computer.resolve(loc)
        }
    }
}

struct BinaryOp {
    op1: OpArg,
    op2: OpArg,
    destination: OpArg,
}

#[derive(Debug, Copy, Clone)]
enum OpCode {
    Add(BinaryOp),
    Mul(BinaryOp),
    ReadInput { to: OpArg },
    SaveOutput { from: OpArg },
    Done,
}

trait InstructionSize {
    fn size(&self) -> usize;
}

impl InstructionSize for OpCode {
    fn size(&self) -> usize {
        match self {
            OpCode::Add(_) | OpCode::Mul(_) => 4,
            OpCode::ReadInput { to: _ } | OpCode::SaveOutput { from: _ } => 2,
            OpCode::Done => 1,
        }
    }
}

fn interpret(op: OpCode, computer: &mut Computer) {
    let stack = computer.state_mut();
    fn binary_op() {}
    match op {
        OpCode::Add(BinaryOpArg { op1, op2, destination: dest }) => {
            let x = stack[op1];
            let y = stack[op2];
            let result = x + y;
            //println!("{dest} = {} ({} + {})", result, x, y, dest = dest);
            stack[dest] = result;
        }
        OpCode::Mul(BinaryOpArg { op1, op2, destination: dest }) => {
            let x = stack[op1];
            let y = stack[op2];
            let result = x * y;
            //println!("{dest} = {} ({} * {})", result, x, y, dest = dest);
            stack[dest] = result;
        }
        OpCode::ReadInput { to } => {}
        OpCode::SaveOutput { from } => {}
        OpCode::Done => {}
    }
}

impl OpCode {
    pub fn parse_eval(stack: &mut ComputerState) -> Vec<OpCode> {
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

    fn parse_op(ip: usize, computer: &Computer) -> Self {
        fn read_binary_op() {}
        match operations[ip % 100] {
            1 => {
                let op1 = operations[ip + 1] as usize;
                let op2 = operations[ip + 2] as usize;
                let dest = operations[ip + 3] as usize;
                return OpCode::Add(BinaryOp { op1, op2, destination: dest });
            }
            2 => {
                let op1 = operations[ip + 1] as usize;
                let op2 = operations[ip + 2] as usize;
                let dest = operations[ip + 3] as usize;
                return OpCode::Mul(BinaryOp { op1, op2, destination: dest });
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

pub fn eval(stack: &mut ComputerState) -> i32 {
    OpCode::parse_eval(stack);
    return stack[0];
}