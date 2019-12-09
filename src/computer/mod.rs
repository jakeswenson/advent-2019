use std::collections::VecDeque;
use std::str::FromStr;

use num::pow::Pow;

pub fn parse_op_stack(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(i32::from_str)
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect()
}

pub fn from(input: &str) -> Computer {
    Computer::new(parse_op_stack(input))
}

type ComputerState = Vec<i32>;

#[derive(Debug)]
pub struct Computer {
    input: VecDeque<i32>,
    output: Vec<i32>,
    state: ComputerState,
    instruction_pointer: usize,
}

impl Computer {
    pub fn add_input(mut self, input: i32) -> Self {
        self.input.push_back(input);
        self
    }
    pub fn state_mut(&mut self) -> &mut ComputerState {
        &mut self.state
    }

    pub fn new(state_vec: Vec<i32>) -> Self {
        Computer {
            input: VecDeque::new(),
            output: Vec::new(),
            state: state_vec,
            instruction_pointer: 0,
        }
    }

    pub fn resolve(&self, reference: usize) -> i32 {
        self.state[reference]
    }

    pub fn set(&mut self, destination: OpArg, value: i32) {
        match destination {
            OpArg::Reference(dest) => self.state[dest] = value,
            _ => unreachable!(),
        }
    }

    const OP_CODE_SIZE: i32 = 100;
    pub fn op_code(&self) -> i32 {
        return self.state[self.instruction_pointer] % Computer::OP_CODE_SIZE;
    }

    pub fn op_arg_factory(&self, arg: usize) -> impl Fn(i32) -> OpArg {
        let param_modes = self.state[self.instruction_pointer] / Computer::OP_CODE_SIZE;
        let arg_denominator = 10.pow(arg - 1);
        let arg_mode = (param_modes / arg_denominator) & 1;
        //        println!("Arg({}) mode: {}", arg, arg_mode);
        move |value| match arg_mode {
            0 => OpArg::Reference(value as usize),
            1 => OpArg::Literal(value),
            _ => unreachable!(),
        }
    }

    pub fn arg(&self, arg: usize) -> OpArg {
        assert!(arg > 0);
        let arg_value = self.state[self.instruction_pointer + arg];
        self.op_arg_factory(arg)(arg_value)
    }

    pub fn binary_op(&self, op_code: impl Fn(BinaryOp) -> OpCode) -> OpCode {
        let op1 = self.arg(1);
        let op2 = self.arg(2);
        let dest = self.arg(3);

        let op_code = op_code(BinaryOp {
            op1,
            op2,
            destination: dest,
        });
        //        println!("{:?}", op_code);
        op_code
    }

    pub fn read_op(&self) -> OpCode {
        match self.op_code() {
            1 => self.binary_op(OpCode::Add),
            2 => self.binary_op(OpCode::Mul),
            3 => OpCode::ReadInput { to: self.arg(1) },
            4 => OpCode::SaveOutput { from: self.arg(1) },
            99 => return OpCode::Done,
            _ => panic!(
                "Invalid opcode: {} @{}",
                self.op_code(),
                self.instruction_pointer
            ),
        }
    }

    pub fn next(&mut self) -> Option<OpCode> {
        if self.instruction_pointer >= self.state.len() {
            return None;
        }

        let op_code = self.read_op();
        self.instruction_pointer += op_code.size();
        Some(op_code)
    }

    pub fn eval(&mut self) -> i32 {
        self.run(0)
    }

    pub fn run(&mut self, result_location: usize) -> i32 {
        self.interpret();
        return self.resolve(result_location);
    }

    fn interpret(&mut self) -> Vec<OpCode> {
        let mut ip = 0;

        let mut result = Vec::new();

        while let Some(op_code) = self.next() {
            result.push(op_code);
            op_code.interpret(self);
            if op_code.is_done() {
                break;
            }
        }

        return result;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum OpArg {
    Literal(i32),
    Reference(usize),
}

impl OpArg {
    pub fn resolve(&self, computer: &Computer) -> i32 {
        match self {
            OpArg::Literal(lit) => lit.clone(),
            OpArg::Reference(loc) => computer.resolve(loc.clone()),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct BinaryOp {
    op1: OpArg,
    op2: OpArg,
    destination: OpArg,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum OpCode {
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

impl OpCode {
    fn is_done(&self) -> bool {
        match self {
            OpCode::Done => true,
            _ => false,
        }
    }

    fn interpret(&self, computer: &mut Computer) {
        let stack = computer.state_mut();
        fn binary_op(
            operation: impl Fn(i32, i32) -> i32,
            binary_op: &BinaryOp,
            computer: &mut Computer,
        ) {
            let x = binary_op.op1.resolve(computer);
            let y = binary_op.op2.resolve(computer);
            let result = operation(x, y);
            //            println!(
            //                "[@{dest:?}] = {} ({} +* {})",
            //                result,
            //                x,
            //                y,
            //                dest = binary_op.destination
            //            );
            computer.set(binary_op.destination, result);
        }
        match self {
            OpCode::Add(bin_op) => binary_op(|x, y| x + y, bin_op, computer),
            OpCode::Mul(bin_op) => binary_op(|x, y| x * y, bin_op, computer),
            OpCode::ReadInput { to } => {
                let input = computer.input.pop_front().expect("No Input");
                computer.set(to.clone(), input)
            }
            OpCode::SaveOutput { from } => {
                let result = from.resolve(computer);
                println!("{}", result);
            }
            OpCode::Done => {}
            _ => unimplemented!("Unsupported operation"),
        }
    }
}
