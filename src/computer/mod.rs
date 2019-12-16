use std::collections::VecDeque;
use std::fmt::{Debug, Display, Error, Formatter};
use std::str::FromStr;

use num::{pow::Pow, One, ToPrimitive, Zero};

mod ops;

pub use ops::{BinaryOp, JumpOp, OpArg, OpCode};

trait InstructionSize {
  fn size(&self) -> usize;
}

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

#[cfg(test)]
pub fn run_single_input(program: &str, input: i64) -> ComputerWord {
  from(program).add_input(input).run().pop().unwrap()
}

pub type ComputerWord = i64;
pub type ComputerState = Vec<ComputerWord>;

#[derive(Debug)]
pub struct Computer {
  input: VecDeque<ComputerWord>,
  output: Vec<ComputerWord>,
  state: ComputerState,
  instruction_pointer: usize,
  relative_base: usize,
}

impl Computer {
  pub fn new(state_vec: Vec<ComputerWord>) -> Self {
    let mut state = vec![0; 2048];
    state[..state_vec.len()].copy_from_slice(&state_vec);
    Computer {
      state,
      input: VecDeque::new(),
      output: Vec::new(),
      instruction_pointer: 0,
      relative_base: 0,
    }
  }

  pub fn add_input(mut self, input: i64) -> Self {
    self.input.push_back(ComputerWord::from(input));
    self
  }

  pub fn output(&mut self, value: &ComputerWord) {
    self.output.push(value.clone())
  }

  pub fn len(&self) -> usize {
    self.state.len()
  }

  pub fn resolve(&self, reference: usize) -> ComputerWord {
    assert!(reference < self.state.capacity());
    self
      .state
      .get(reference)
      .cloned()
      .unwrap_or(ComputerWord::zero())
  }

  fn calc_relative(&self, relative_location: i32) -> usize {
    let location = (self.relative_base as i32 + relative_location) as usize;
    return location;
  }

  pub fn resolve_relative(&self, relative_location: i32) -> ComputerWord {
    let location = self.calc_relative(relative_location);
    assert!(location < self.state.len());
    self.state[location].clone()
  }

  pub fn set(&mut self, destination: OpArg, value: ComputerWord) {
    match destination {
      OpArg::Reference(dest) => self.state[dest] = value,
      OpArg::Relative(dest) => {
        let location = self.calc_relative(dest);
        self.state[location] = value;
      }

      _ => unreachable!(),
    }
  }

  pub fn jump(&mut self, target: &OpArg) {
    let target = target.resolve(self).to_usize().unwrap();
    self.instruction_pointer = target;
  }

  const OP_CODE_SIZE: i32 = 100;
  pub fn op_code(&self) -> u8 {
    return (self.state[self.instruction_pointer].to_i32().unwrap() % Computer::OP_CODE_SIZE) as u8;
  }

  pub fn op_param_modes(&self) -> i32 {
    let value = self.state[self.instruction_pointer];
    value.to_i32().unwrap() / Computer::OP_CODE_SIZE
  }

  pub fn arg(&self, arg: usize) -> OpArg {
    assert!(arg > 0);
    let arg_value = self.state[self.instruction_pointer + arg].clone();
    OpArg::factory(self, arg)(arg_value)
  }

  pub fn binary_op(&self, op_code: impl Fn(BinaryOp) -> OpCode) -> OpCode {
    let op1 = self.arg(1);
    let op2 = self.arg(2);
    let dest = self.arg(3);

    let op_code = op_code(BinaryOp::new(op1, op2, dest));
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

  pub fn eval(&mut self) -> ComputerWord {
    self.eval_at(0)
  }

  pub fn eval_at(&mut self, result_location: usize) -> ComputerWord {
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
