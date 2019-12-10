use std::collections::VecDeque;
use std::fmt::{Debug, Display, Error, Formatter};
use std::str::FromStr;

use num::{pow::Pow, BigInt, One, ToPrimitive, Zero};

pub fn parse_op_stack(input: &str) -> Vec<ComputerWord> {
    input
        .split(',')
        .map(|s| s.trim())
        .map(|s| (s, ComputerWord::from_str(s)))
        .map(|(orig, result)| result.expect(&format!("Can't Parse '{}' to BigInt", orig)))
        .collect()
}

pub fn from(input: &str) -> Computer {
    Computer::new(parse_op_stack(input))
}

pub type ComputerWord = BigInt;
pub type ComputerState = Vec<ComputerWord>;

#[derive(Debug)]
pub struct Computer {
    input: VecDeque<ComputerWord>,
    output: Vec<ComputerWord>,
    state: ComputerState,
    instruction_pointer: usize,
}

impl Computer {
    pub fn new(state_vec: Vec<ComputerWord>) -> Self {
        Computer {
            input: VecDeque::new(),
            output: Vec::new(),
            state: state_vec,
            instruction_pointer: 0,
        }
    }

    pub fn add_input(mut self, input: i32) -> Self {
        self.input.push_back(ComputerWord::from(input));
        self
    }

    pub fn output(&mut self, value: &ComputerWord) {
        self.output.push(value.clone())
    }

    pub fn len(&self) -> usize {
        self.state.len()
    }

    pub fn resolve(&self, reference: usize) -> &ComputerWord {
        assert!(reference < self.state.len());
        &self.state[reference]
    }

    pub fn set(&mut self, destination: OpArg, value: ComputerWord) {
        match destination {
            OpArg::Reference(dest) => self.state[dest] = value,
            _ => unreachable!(),
        }
    }

    pub fn jump(&mut self, target: &OpArg) {
        let target = target.resolve(self).to_usize().unwrap();
        self.instruction_pointer = target;
    }

    const OP_CODE_SIZE: i32 = 100;
    pub fn op_code(&self) -> u8 {
        return (self.state[self.instruction_pointer].to_i32().unwrap() % Computer::OP_CODE_SIZE)
            as u8;
    }

    pub fn op_arg_factory(&self, arg: usize) -> impl Fn(ComputerWord) -> OpArg {
        let param_modes =
            self.state[self.instruction_pointer].to_i32().unwrap() / Computer::OP_CODE_SIZE;

        let arg_denominator = 10.pow(arg - 1);
        let arg_mode = (param_modes / arg_denominator) & 1;

        //        println!("Arg({}) mode: {}", arg, arg_mode);
        move |value| match arg_mode {
            0 => OpArg::Reference(value.to_usize().unwrap()),
            1 => OpArg::Literal(value),
            _ => unreachable!(),
        }
    }

    pub fn arg(&self, arg: usize) -> OpArg {
        assert!(arg > 0);
        let arg_value = self.state[self.instruction_pointer + arg].clone();
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

    pub fn next(&mut self) -> Option<OpCode> {
        if self.instruction_pointer >= self.state.len() {
            return None;
        }

        let op_code = OpCode::read_op(self);
        self.instruction_pointer += op_code.size();
        Some(op_code)
    }

    pub fn eval(&mut self) -> &ComputerWord {
        self.eval_at(0)
    }

    pub fn eval_at(&mut self, result_location: usize) -> &ComputerWord {
        self.interpret();
        return self.resolve(result_location);
    }

    pub fn run(mut self) -> Vec<ComputerWord> {
        self.interpret();
        self.output
    }

    fn interpret(&mut self) -> Vec<OpCode> {
        let mut result = Vec::new();

        while let Some(op_code) = self.next() {
            op_code.interpret(self);
            let is_done = op_code.is_done();
            result.push(op_code);
            if is_done {
                break;
            }
        }

        return result;
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum OpArg {
    Literal(ComputerWord),
    Reference(usize),
}

impl Display for OpArg {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            OpArg::Literal(literal) => write!(f, "{}", literal),
            OpArg::Reference(location) => write!(f, "@{}", location),
        }
    }
}

impl Debug for OpArg {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self)
    }
}

impl OpArg {
    pub fn resolve<'a>(&'a self, computer: &'a Computer) -> &'a ComputerWord {
        match self {
            OpArg::Literal(lit) => lit,
            OpArg::Reference(loc) => computer.resolve(*loc),
        }
    }

    pub fn unwrap_location(&self) -> usize {
        match self {
            OpArg::Reference(loc) => loc.clone(),
            _ => panic!("Location changed"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BinaryOp {
    op1: OpArg,
    op2: OpArg,
    destination: OpArg,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct JumpOp {
    test: OpArg,
    target: OpArg,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum OpCode {
    Add(BinaryOp),
    Mul(BinaryOp),
    ReadInput { to: OpArg },
    SaveOutput { from: OpArg },
    JumpIfNonZero(JumpOp),
    JumpIfZero(JumpOp),
    LessThan(BinaryOp),
    Equals(BinaryOp),
    Done,
}

trait InstructionSize {
    fn size(&self) -> usize;
}

impl InstructionSize for OpCode {
    fn size(&self) -> usize {
        match self {
            OpCode::Add(_) | OpCode::Mul(_) | OpCode::LessThan(_) | OpCode::Equals(_) => 4,
            OpCode::JumpIfZero(_) | OpCode::JumpIfNonZero(_) => 3,
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

    pub fn read_op(computer: &Computer) -> OpCode {
        match computer.op_code() {
            1 => computer.binary_op(OpCode::Add),
            2 => computer.binary_op(OpCode::Mul),
            3 => OpCode::ReadInput {
                to: computer.arg(1),
            },
            4 => OpCode::SaveOutput {
                from: computer.arg(1),
            },
            5 => OpCode::JumpIfNonZero(JumpOp {
                test: computer.arg(1),
                target: computer.arg(2),
            }),
            6 => OpCode::JumpIfZero(JumpOp {
                test: computer.arg(1),
                target: computer.arg(2),
            }),
            7 => computer.binary_op(OpCode::LessThan),
            8 => computer.binary_op(OpCode::Equals),
            99 => return OpCode::Done,
            _ => panic!(
                "Invalid opcode: {} @{}",
                computer.op_code(),
                computer.instruction_pointer
            ),
        }
    }

    fn binary_op(
        computer: &mut Computer,
        binary_op: &BinaryOp,
        _op: &str,
        operation: impl Fn(&ComputerWord, &ComputerWord) -> ComputerWord,
    ) {
        println!("Operation({op}): {:?}", binary_op, op = _op);
        let x = binary_op.op1.resolve(computer);
        let y = binary_op.op2.resolve(computer);
        let result = operation(&x, &y);
        println!(
            "[{dest}] = {} ({} {op} {})",
            result,
            x,
            y,
            op = _op,
            dest = binary_op.destination
        );
        computer.set(binary_op.destination.clone(), result);
    }

    fn bool_op(
        computer: &mut Computer,
        binary_op: &BinaryOp,
        _op: &str,
        condition: impl Fn(&ComputerWord, &ComputerWord) -> bool,
    ) {
        OpCode::binary_op(computer, binary_op, _op, |a, b| {
            if condition(a, b) {
                ComputerWord::one()
            } else {
                ComputerWord::zero()
            }
        })
    }

    fn jump(
        computer: &mut Computer,
        jump_op: &JumpOp,
        _op: &str,
        condition: impl Fn(&ComputerWord) -> bool,
    ) {
        println!("Operation({op}): {:?}", jump_op, op = _op);
        if condition(jump_op.test.resolve(computer)) {
            computer.jump(&jump_op.target)
        }
    }

    fn interpret(&self, computer: &mut Computer) {
        match self {
            OpCode::Add(bin_op) => OpCode::binary_op(computer, bin_op, "+", |x, y| x + y),
            OpCode::Mul(bin_op) => OpCode::binary_op(computer, bin_op, "*", |x, y| x * y),
            OpCode::LessThan(bin_op) => OpCode::bool_op(computer, bin_op, "<", |x, y| x < y),
            OpCode::Equals(bin_op) => OpCode::bool_op(computer, bin_op, "==", |x, y| x == y),
            OpCode::JumpIfNonZero(jump_op) => {
                OpCode::jump(computer, jump_op, "jnz", |i| i != &ComputerWord::zero())
            }
            OpCode::JumpIfZero(jump_op) => {
                OpCode::jump(computer, jump_op, "jz", |i| i == &ComputerWord::zero())
            }
            OpCode::ReadInput { to } => {
                let input = computer.input.pop_front().expect("No Input");
                computer.set(to.clone(), input)
            }
            OpCode::SaveOutput { from } => {
                let result = from.resolve(computer).clone();
                computer.output(&result);
            }
            OpCode::Done => {}
        }
    }
}
