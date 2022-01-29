use crate::parse::IntrinsicType;
use crate::parse::OpType;
use crate::program::Program;

pub fn simulate_program(mut program: Program) {
    let mut stack: Vec<u64> = Vec::new();

    while let Some(op) = program.next_op() {
        match op.typ {
            OpType::PushInt(val) => stack.push(val),
            OpType::Intrinsic(intr) => simulate_intrinsic(intr, &mut stack),
        }
    }
}

fn simulate_intrinsic(intrinsic: IntrinsicType, stack: &mut Vec<u64>) {
    match intrinsic {
        IntrinsicType::Plus => {
            let a = stack.pop().expect("Stack is empty when popping");
            let b = stack.pop().expect("Stack is empty when popping");
            stack.push(a + b);
        }
        IntrinsicType::Multiply => {
            let a = stack.pop().expect("Stack is empty when popping");
            let b = stack.pop().expect("Stack is empty when popping");
            stack.push(a * b);
        }
        IntrinsicType::Print => {
            println!("{}", stack.pop().expect("Stack is empty when popping"));
        }
    }
}
