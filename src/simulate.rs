use std::{error, fmt};

use crate::lex::Token;
use crate::parse::{IntrinsicType, OpType};
use crate::program::{FileLocation, Program};

#[derive(Debug)]
enum ErrorKind {
    StackUnderflow,
}

impl ErrorKind {
    fn as_str(&self) -> &str {
        match self {
            Self::StackUnderflow => "Stack underflowed",
        }
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    loc: Option<FileLocation>,
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self { kind, loc: None }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "--- {} --- {}",
            if let Some(loc) = &self.loc {
                loc.to_string()
            } else {
                "Unknown Location".to_string()
            },
            self.kind.as_str()
        )
    }
}

impl error::Error for Error {}

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
            None => Err(ErrorKind::StackUnderflow.into()),
        }
    }
}

pub fn simulate(program: &Program) -> Result<(), Error> {
    let mut stack = Stack::new();

    for op in &program.root_block.ops {
        let result = match &op.typ {
            OpType::PushInt(val) => Ok(stack.push(*val)),
            OpType::Intrinsic(intr) => simulate_intrinsic(&mut stack, intr, &op.token),
        };

        if let Err(mut err) = result {
            err.loc = Some(op.token.loc.clone());
            return Err(err);
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
