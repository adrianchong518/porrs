use std::fmt;

use crate::lex::Token;
use crate::parse::{IntrinsicType, OpBlock, OpType};
use crate::program::Program;
use crate::Error;

#[derive(Debug)]
pub(crate) enum SimulationError {
    StackUnderflow,
}

impl fmt::Display for SimulationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use SimulationError::*;
        match self {
            StackUnderflow => write!(f, "Stack underflowed"),
        }
    }
}

struct Stack(Vec<u64>);

impl Stack {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, val: u64) {
        self.0.push(val);
    }

    fn pop(&mut self) -> Result<u64, Error> {
        match self.0.pop() {
            Some(val) => Ok(val),
            None => Err(Error::from(SimulationError::StackUnderflow)),
        }
    }
}

pub fn simulate(program: &Program) -> Result<(), Error> {
    let mut stack = Stack::new();

    simulate_op_block(&mut stack, &program.root_block)
}

fn simulate_op_block(stack: &mut Stack, op_block: &OpBlock) -> Result<(), Error> {
    for op in &op_block.ops {
        let result: Result<(), Error> = match &op.typ {
            OpType::PushInt(val) => Ok(stack.push(*val)),
            OpType::Intrinsic(intr) => simulate_intrinsic(stack, intr, &op.token),
            OpType::If(op_block) => simulate_if(stack, op_block, &op.token),
            OpType::End => Ok(()),
        };

        if let Err(err) = result {
            return Err(err.push_loc(&op.token.loc));
        }
    }

    Ok(())
}

fn simulate_intrinsic(
    stack: &mut Stack,
    intrinsic: &IntrinsicType,
    token: &Token,
) -> Result<(), Error> {
    match intrinsic {
        IntrinsicType::Plus => {
            let b = stack.pop()?;
            let a = stack.pop()?;
            let result = a.checked_add(b).unwrap_or_else(|| {
                log::warn!("--- {} --- Operation `+` overflowed", token.loc);
                a.wrapping_add(b)
            });
            stack.push(result);
        }

        IntrinsicType::Subtract => {
            let b = stack.pop()?;
            let a = stack.pop()?;
            let result = a.checked_sub(b).unwrap_or_else(|| {
                log::warn!("--- {} --- Operation `-` overflowed", token.loc);
                a.wrapping_sub(b)
            });
            stack.push(result);
        }

        IntrinsicType::Multiply => {
            let b = stack.pop()?;
            let a = stack.pop()?;
            stack.push(a * b);
        }

        IntrinsicType::DivMod => {
            let b = stack.pop()?;
            let a = stack.pop()?;
            stack.push(a / b);
            stack.push(a % b);
        }

        IntrinsicType::Print => {
            println!("{val} ({val:#018x})", val = stack.pop()?);
        }
    }

    Ok(())
}

fn simulate_if(stack: &mut Stack, op_block: &OpBlock, token: &Token) -> Result<(), Error> {
    let cond = stack.pop()?;

    if cond > 0 {
        if cond > 1 {
            log::warn!(
                "--- {} --- Used non-binary value ({}) as condition for `if`",
                token.loc,
                cond
            );
        }

        simulate_op_block(stack, op_block)
    } else {
        Ok(())
    }
}
