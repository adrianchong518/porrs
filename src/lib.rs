enum OpType {
    PushInt(u64),
    Plus,
    Print,
}

struct Op {
    typ: OpType,
}

fn simulate_program(program: &Vec<Op>) {
    let mut stack: Vec<u64> = Vec::new();

    for op in program {
        match op.typ {
            OpType::PushInt(val) => stack.push(val),
            OpType::Plus => {
                let a = stack.pop().expect("Stack is empty when popping");
                let b = stack.pop().expect("Stack is empty when popping");
                stack.push(a + b);
            }
            OpType::Print => {
                println!("{}", stack.pop().expect("Stack is empty when popping"));
            }
        }
    }
}

pub fn main() {
    use OpType::*;
    let program = vec![PushInt(34), PushInt(35), Plus, Print]
        .into_iter()
        .map(|typ| Op { typ })
        .collect();
    simulate_program(&program);
}
