use std::process::exit;

use crate::parse::OpType;
use crate::parse::{IntrinsicType, Op};
use crate::program::Program;

pub struct Simulation {
    program: Program,
    stack: Vec<u64>,

    current_op: Option<Op>,
}

impl Simulation {
    pub fn new(program: Program) -> Self {
        Self {
            program,
            stack: Vec::new(),
            current_op: None,
        }
    }

    pub fn simulate(&mut self) {
        while let Some(op) = self.program.next_op() {
            self.current_op = Some(op);
            match self.current_op.as_ref().unwrap().typ {
                OpType::PushInt(val) => self.stack.push(val),
                OpType::Intrinsic(intr) => self.simulate_intrinsic(intr),
            }
        }
    }

    fn pop_stack(&mut self) -> u64 {
        self.stack.pop().unwrap_or_else(|| {
            log::error!(
                "--- {} --- Stack underflow",
                self.current_op.as_ref().unwrap().token.loc
            );
            exit(1)
        })
    }

    fn simulate_intrinsic(&mut self, intrinsic: IntrinsicType) {
        match intrinsic {
            IntrinsicType::Plus => {
                let b = self.pop_stack();
                let a = self.pop_stack();
                let result = a.checked_add(b).unwrap_or_else(|| {
                    log::warn!(
                        "--- {} --- Operation `+` overflowed",
                        self.current_op.as_ref().unwrap().token.loc
                    );
                    a.wrapping_add(b)
                });
                self.stack.push(result);
            }

            IntrinsicType::Subtract => {
                let b = self.pop_stack();
                let a = self.pop_stack();
                let result = a.checked_sub(b).unwrap_or_else(|| {
                    log::warn!(
                        "--- {} --- Operation `-` overflowed",
                        self.current_op.as_ref().unwrap().token.loc
                    );
                    a.wrapping_sub(b)
                });
                self.stack.push(result);
            }

            IntrinsicType::Multiply => {
                let b = self.pop_stack();
                let a = self.pop_stack();
                self.stack.push(a * b);
            }

            IntrinsicType::DivMod => {
                let b = self.pop_stack();
                let a = self.pop_stack();
                self.stack.push(a / b);
                self.stack.push(a % b);
            }

            IntrinsicType::Print => {
                println!("{val} ({val:#018x})", val = self.pop_stack());
            }
        }
    }
}
