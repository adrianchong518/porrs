use crate::parse::IntrinsicType;
use crate::program::Program;

pub fn simulate_program(mut program: Program) {
    while let Some(tok) = program.lexer.next_token() {}

    todo!();
}

fn simulate_intrinsic(intrinsic: IntrinsicType, stack: &mut Vec<u64>) {
    match intrinsic {
        IntrinsicType::Plus => {
            let a = stack.pop().expect("Stack is empty when popping");
            let b = stack.pop().expect("Stack is empty when popping");
            stack.push(a + b);
        }
        IntrinsicType::Print => {
            println!("{}", stack.pop().expect("Stack is empty when popping"));
        }
    }
}
