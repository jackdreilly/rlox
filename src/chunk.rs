// Strum contains all the trait definitions
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryInto;
use strum_macros::Display;

#[derive(IntoPrimitive, TryFromPrimitive, Display, PartialEq)]
#[repr(u8)]
pub enum OpCode {
    Constant,
    Negate,
    Multiply,
    Divide,
    Add,
    Subtract,
    ConstantLong,
    Return,
}
pub type Value = f64;
type Code = u8;
type Line = usize;
#[derive(Clone)]
pub struct Chunk {
    pub code: Vec<Code>,
    pub constants: Vec<Value>,
    pub lines: LineEncoding,
}

type LineEncoding = Vec<Line>;
trait LineEncoder {
    fn put(&mut self, line: Line);
    fn get(&self, offset: Line) -> Line;
}
impl LineEncoder for LineEncoding {
    fn put(&mut self, line: Line) {
        if line >= self.len() {
            self.resize(1 + line, 0);
        }
        self[line] += 1;
    }

    fn get(&self, offset: Line) -> Line {
        let mut tracker = 0;
        let mut index = 0;
        while tracker <= offset {
            tracker += self[index];
            index += 1;
        }
        return index - 1;
    }
}

impl Chunk {
    pub fn new_chunk() -> Self {
        Chunk {
            code: vec![],
            constants: vec![],
            lines: vec![],
        }
    }
    fn disassemble(&self, description: &str) {
        println!("== {} ==", description);
        let mut offset: usize = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }
    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);
        if offset > 0 && self.get_line(offset) == self.get_line(offset - 1) {
            print!("   | ");
        } else {
            print!("{:4} ", self.get_line(offset));
        }
        let op_code = self.code[offset].try_into().unwrap();
        match op_code {
            OpCode::Constant => self.disassemble_constant(offset),
            OpCode::ConstantLong => self.disassemble_constant_long(offset),
            _ => self.simple_instruction(op_code.to_string().as_str(), offset),
        }
    }
    fn simple_instruction(&self, value: &str, offset: usize) -> usize {
        println!("{}", value);
        return offset + 1;
    }
    fn disassemble_constant(&self, offset: usize) -> usize {
        let constant_offset = self.code[offset + 1];
        print!("{:16} {:4} '", OpCode::Constant, constant_offset,);
        self.print_value(&self.constants[constant_offset as usize]);
        print!("'\n");
        return offset + 2;
    }
    fn disassemble_constant_long(&self, offset: usize) -> usize {
        let constant_offset =
            ((((self.code[offset + 1] as usize) << 8) + (self.code[offset + 2] as usize)) << 8)
                + self.code[offset + 3] as usize;
        print!("{:16} {:12} '", OpCode::ConstantLong, constant_offset);
        self.print_value(&self.constants[constant_offset]);
        print!("'\n");
        return offset + 4;
    }

    fn print_value(&self, value: &Value) {
        print!("{:?}", value);
    }

    pub fn write_op_code(&mut self, op_code: OpCode, line: Line) {
        self.write_code(op_code.into(), line);
    }
    fn write_code(&mut self, code: Code, line: Line) {
        self.code.push(code);
        self.lines.put(line);
    }
    fn get_line(&self, line: Line) -> Line {
        self.lines.get(line)
    }
    pub fn write_constant(&mut self, value: Value, line: Line) {
        let constant_offset = self.put_constant(value);
        if constant_offset < u8::MAX.into() {
            self.write_op_code(OpCode::Constant, line);
            self.write_operand(constant_offset as u8, line);
        } else {
            self.write_op_code(OpCode::ConstantLong, line);
            self.write_operand((constant_offset >> 16) as u8, line);
            self.write_operand((constant_offset >> 8) as u8, line);
            self.write_operand(constant_offset as u8, line);
        }
    }
    fn put_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        return self.constants.len() - 1;
    }
    fn free_chunk(&mut self) {
        self.lines.truncate(0);
        self.code.truncate(0);
        self.constants.truncate(0);
    }
    fn write_operand(&mut self, operand: Code, line: Line) {
        self.write_code(operand, line);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn new_chunk() {
        assert_eq!(Chunk::new_chunk().code.len(), 0);
    }
    #[test]
    fn line_encoder() {
        let mut le: LineEncoding = vec![];
        le.put(0);
        le.put(0);
        le.put(0);
        le.put(1);
        le.put(1);
        le.put(2);
        le.put(3);
        le.put(3);
        le.put(3);
        le.put(3);
        assert_eq!(le.get(0), 0);
        assert_eq!(le.get(1), 0);
        assert_eq!(le.get(2), 0);
        assert_eq!(le.get(3), 1);
        assert_eq!(le.get(4), 1);
        assert_eq!(le.get(5), 2);
        assert_eq!(le.get(6), 3);
        assert_eq!(le.len(), 4);
    }
    #[test]
    fn disassemble() {
        let mut chunk = Chunk::new_chunk();
        chunk.write_op_code(OpCode::Return, 0);
        for n in 0..260 {
            chunk.write_constant(n as f64 * 2.0, (n as f64).sqrt().floor() as usize);
        }
        chunk.disassemble("howdy");
    }
}
