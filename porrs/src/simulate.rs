use std::fmt;

use crate::op::{If, IfStarBlock, Intrinsic, OpBlock, OpType, While};
use crate::program::{FileLocation, Program};
use crate::{Error, Result};

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

    fn pop(&mut self) -> Result<u64> {
        match self.0.pop() {
            Some(val) => Ok(val),
            None => Err(Error::from(SimulationError::StackUnderflow)),
        }
    }
}

pub fn simulate(program: &Program) -> Result<()> {
    let mut stack = Stack::new();

    simulate_op_block(&mut stack, &program.root_block)
}

fn simulate_op_block(stack: &mut Stack, op_block: &OpBlock) -> Result<()> {
    for op in op_block.iter() {
        let result: Result<()> = match &op.typ {
            OpType::PushInt(val) => Ok(stack.push(*val)),
            OpType::Intrinsic(intr) => simulate_intrinsic(stack, &intr, &op.loc),
            OpType::If(if_op) => simulate_if(stack, if_op, &op.loc),
            OpType::While(while_op) => simulate_while(stack, while_op),
        };

        if let Err(err) = result {
            if err.has_loc() {
                return Err(err);
            } else {
                return Err(err.add_loc(op.loc.clone()));
            }
        }
    }

    Ok(())
}

fn simulate_intrinsic(stack: &mut Stack, intrinsic: &Intrinsic, loc: &FileLocation) -> Result<()> {
    match intrinsic {
        Intrinsic::Dup => {
            let a = stack.pop()?;
            stack.push(a);
            stack.push(a);
        }

        Intrinsic::Swap => {
            let b = stack.pop()?;
            let a = stack.pop()?;
            stack.push(b);
            stack.push(a);
        }

        Intrinsic::Drop => {
            let _ = stack.pop()?;
        }

        Intrinsic::Print => {
            println!("{val} ({val:#018x})", val = stack.pop()?);
        }

        Intrinsic::Over => {
            let b = stack.pop()?;
            let a = stack.pop()?;
            stack.push(a);
            stack.push(b);
            stack.push(a);
        }

        Intrinsic::Rot => {
            let c = stack.pop()?;
            let b = stack.pop()?;
            let a = stack.pop()?;
            stack.push(b);
            stack.push(c);
            stack.push(a);
        }

        Intrinsic::Plus => {
            let b = stack.pop()?;
            let a = stack.pop()?;
            let result = a.checked_add(b).unwrap_or_else(|| {
                log::warn!("<-- {} --> Operation `+` overflowed", loc);
                a.wrapping_add(b)
            });
            stack.push(result);
        }

        Intrinsic::Subtract => {
            let b = stack.pop()?;
            let a = stack.pop()?;
            let result = a.checked_sub(b).unwrap_or_else(|| {
                log::warn!("<-- {} --> Operation `-` overflowed", loc);
                a.wrapping_sub(b)
            });
            stack.push(result);
        }

        Intrinsic::Multiply => {
            let b = stack.pop()?;
            let a = stack.pop()?;
            stack.push(a * b);
        }

        Intrinsic::DivMod => {
            let b = stack.pop()?;
            let a = stack.pop()?;
            stack.push(a / b);
            stack.push(a % b);
        }
    }

    Ok(())
}

fn is_condition_true(cond: u64, loc: &FileLocation) -> bool {
    if cond != 0 && cond != 1 {
        log::warn!(
            "<-- {} --> Non-binary value ({}) used as a boolean condition",
            loc,
            cond
        );
    }

    cond > 0
}

fn simulate_if(stack: &mut Stack, if_op: &If, if_loc: &FileLocation) -> Result<()> {
    if is_condition_true(stack.pop()?, if_loc) {
        simulate_op_block(stack, &if_op.if_block)
    } else {
        for IfStarBlock { loc, cond, inner } in &if_op.if_star_blocks {
            simulate_op_block(stack, cond)?;

            if is_condition_true(stack.pop()?, loc) {
                return simulate_op_block(stack, inner);
            }
        }

        if let Some(else_block) = &if_op.else_block {
            simulate_op_block(stack, else_block)
        } else {
            Ok(())
        }
    }
}

fn simulate_while(stack: &mut Stack, while_op: &While) -> Result<()> {
    let do_loc = while_op.do_loc.as_ref().unwrap();

    simulate_op_block(stack, &while_op.cond_block)?;

    while is_condition_true(
        stack.pop().map_err(|err| err.add_loc(do_loc.clone()))?,
        do_loc,
    ) {
        simulate_op_block(stack, &while_op.do_block)?;
        simulate_op_block(stack, &while_op.cond_block)?;
    }

    Ok(())
}
