pub use super::{Computer, ComputerWord};
use num::traits::{One, ToPrimitive, Zero};
use std::fmt::{Debug, Display, Error, Formatter};

use super::InstructionSize;

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum OpArg {
  Literal(ComputerWord),
  Reference(usize),
  Relative(i32),
}

impl Debug for OpArg {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    write!(f, "{}", self)
  }
}

impl Display for OpArg {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    match self {
      OpArg::Literal(literal) => write!(f, "{}", literal),
      OpArg::Reference(location) => write!(f, "@{}", location),
      OpArg::Relative(rel_location) => write!(f, "r{}", rel_location),
    }
  }
}

impl OpArg {
  pub fn resolve<'a>(&'a self, computer: &'a Computer) -> ComputerWord {
    match self {
      OpArg::Literal(lit) => *lit,
      OpArg::Reference(loc) => computer.resolve(*loc),
      OpArg::Relative(rel_loc) => computer.resolve_relative(*rel_loc),
    }
  }

  pub fn factory(computer: &Computer, arg: usize) -> impl Fn(ComputerWord) -> OpArg {
    assert!(arg > 0);

    let param_modes = computer.op_param_modes();

    let arg_denominator = 10i32.pow(arg.to_u32().unwrap() - 1);
    let arg_mode = (param_modes / arg_denominator) % 10;

    //        println!("Arg({}) mode: {}", arg, arg_mode);
    move |value| match arg_mode {
      0 => OpArg::Reference(value.to_usize().expect(&format!(
        "Trying to convert to i32: {} argmode={}",
        value, arg_mode
      ))),
      1 => OpArg::Literal(value),
      2 => OpArg::Relative(value.to_i32().expect(&format!(
        "Trying to convert to i32: {} argmode={}",
        value, arg_mode
      ))),
      _ => unimplemented!(),
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BinaryOp {
  op1: OpArg,
  op2: OpArg,
  destination: OpArg,
}

impl BinaryOp {
  pub fn new(operand1: OpArg, operand2: OpArg, destination: OpArg) -> Self {
    BinaryOp {
      op1: operand1,
      op2: operand2,
      destination,
    }
  }
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
  RelativeAdjustment(OpArg),
  Done,
}

impl InstructionSize for OpCode {
  fn size(&self) -> usize {
    match self {
      OpCode::Add(_) | OpCode::Mul(_) | OpCode::LessThan(_) | OpCode::Equals(_) => 4,
      OpCode::JumpIfZero(_) | OpCode::JumpIfNonZero(_) => 3,
      OpCode::ReadInput { to: _ }
      | OpCode::SaveOutput { from: _ }
      | OpCode::RelativeAdjustment(_) => 2,
      OpCode::Done => 1,
    }
  }
}

impl OpCode {
  pub fn is_done(&self) -> bool {
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
      9 => OpCode::RelativeAdjustment(computer.arg(1)),
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
    operation: impl Fn(ComputerWord, ComputerWord) -> ComputerWord,
  ) {
    //println!("Operation({op}): {:?}", binary_op, op = _op);
    let x = binary_op.op1.resolve(computer);
    let y = binary_op.op2.resolve(computer);
    let result = operation(x, y);
    //        println!(
    //            "[{dest}] = {} ({} {op} {})",
    //            result,
    //            x,
    //            y,
    //            op = _op,
    //            dest = binary_op.destination
    //        );
    computer.set(binary_op.destination.clone(), result);
  }

  fn bool_op(
    computer: &mut Computer,
    binary_op: &BinaryOp,
    _op: &str,
    condition: impl Fn(ComputerWord, ComputerWord) -> bool,
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
    condition: impl Fn(ComputerWord) -> bool,
  ) {
    //        println!("Operation({op}): {:?}", jump_op, op = _op);
    if condition(jump_op.test.resolve(computer)) {
      computer.jump(&jump_op.target)
    }
  }

  pub fn interpret(&self, computer: &mut Computer) {
    match self {
      OpCode::Add(bin_op) => {
        OpCode::binary_op(computer, bin_op, "+", |x, y| x.checked_add(y).unwrap())
      }
      OpCode::Mul(bin_op) => {
        OpCode::binary_op(computer, bin_op, "*", |x, y| x.checked_mul(y).unwrap())
      }
      OpCode::LessThan(bin_op) => OpCode::bool_op(computer, bin_op, "<", |x, y| x < y),
      OpCode::Equals(bin_op) => OpCode::bool_op(computer, bin_op, "==", |x, y| x == y),
      OpCode::JumpIfNonZero(jump_op) => {
        OpCode::jump(computer, jump_op, "jnz", |i| i != ComputerWord::zero())
      }
      OpCode::JumpIfZero(jump_op) => {
        OpCode::jump(computer, jump_op, "jz", |i| i == ComputerWord::zero())
      }
      OpCode::ReadInput { to } => {
        let input = computer.input.pop_front().expect("No Input");
        computer.set(to.clone(), input)
      }
      OpCode::SaveOutput { from } => {
        let result = from.resolve(computer).clone();
        computer.output(&result);
      }
      OpCode::RelativeAdjustment(adjustment) => {
        computer.relative_base = computer.calc_relative(adjustment.resolve(computer) as i32);
      }
      OpCode::Done => {}
    }
  }
}
