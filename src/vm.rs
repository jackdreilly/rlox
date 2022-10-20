use super::chunk::*;
use strum_macros::Display;
pub struct VM {
    stack: Vec<Value>,
}

#[derive(Display)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl VM {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }
    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }
    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }
    fn free(&mut self) {
        self.stack.truncate(0);
    }
    pub fn interpret_chunk(&mut self, chunk: &Chunk) -> InterpretResult {
        let mut iter = chunk.code.iter().enumerate();
        let mut pair = iter.next();
        loop {
            let instruction = pair.map(|(_, instruction)| instruction);
            #[cfg(debug_assertions)]
            match pair {
                Some((offset, _)) => {
                    // TODO: Add back
                    // chunk.disassemble_instruction(offset);
                    println!("Offset {}", offset);
                    print!("          ");
                    println!("=====");
                    println!("stack");
                    println!("=====");
                    print!("          ");
                    for value in self.stack.iter() {
                        print!("[ {:} ]", *value);
                    }
                    println!("");
                }
                None => {}
            };
            let mut binary_op = |op: fn(f64, f64) -> f64| {
                let right = self.pop();
                let left = self.pop();
                self.push(op(left, right));
            };
            match instruction {
                Some(instruction) => match OpCode::try_from(*instruction) {
                    Ok(op_code) => match op_code {
                        OpCode::Negate => {
                            let offset = self.stack.len() - 1;
                            self.stack[offset] *= -1.0;
                        }
                        OpCode::Constant => {
                            let value = chunk.constants[*iter.next().unwrap().1 as usize];
                            self.push(value);
                            println!("{:}", value);
                        }
                        OpCode::ConstantLong => {
                            let byte_1 = *iter.next().unwrap().1 as usize;
                            let byte_2 = *iter.next().unwrap().1 as usize;
                            let byte_3 = *iter.next().unwrap().1 as usize;
                            let value = chunk.constants[(byte_1 << 16) + (byte_2 << 8) + byte_3];
                            println!("{:}", value);
                        }
                        OpCode::Return => {
                            println!("{:}", self.pop());
                            return InterpretResult::Ok;
                        }
                        OpCode::Multiply => binary_op(|a, b| a * b),
                        OpCode::Divide => binary_op(|a, b| a / b),
                        OpCode::Add => binary_op(|a, b| a + b),
                        OpCode::Subtract => binary_op(|a, b| a - b),
                    },
                    Err(_) => return InterpretResult::RuntimeError,
                },
                None => return InterpretResult::RuntimeError,
            }
            pair = iter.next();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn vm_test() {
        let mut my_vm = VM::new();
        let mut my_chunk = Chunk::new_chunk();
        for i in 0..260 {
            my_chunk.write_constant((i / 2) as f64, i / 2);
            if i % 2 == 0 {
                my_chunk.write_op_code(OpCode::Negate, i / 2);
            }
            if i % 4 == 3 {
                my_chunk.write_op_code(OpCode::Multiply, i / 2);
            }
            if i % 8 == 7 {
                my_chunk.write_op_code(OpCode::Add, i / 2);
            }
        }
        my_chunk.write_op_code(OpCode::Return, 1);
        println!("{:}", my_vm.interpret_chunk(&my_chunk));
    }
}
