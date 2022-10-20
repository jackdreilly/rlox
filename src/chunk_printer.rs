use std::fs::File;
use std::io::{Write, Result};

use crate::chunk::{Chunk, OpCode, Value};

pub fn print_chunk(chunk: &Chunk, file: &mut File, description: &str) -> Result<()> {
    ChunkPrinter::new(chunk, file).disassemble(description)?;
    OK
}

struct ChunkPrinter<'a> {
    chunk: &'a Chunk,
    file: &'a mut File,
}

type R = Result<()>;
type RU = Result<usize>;
const OK: R = Ok(());

impl<'a> ChunkPrinter<'a> {
    fn disassemble(&mut self, description: &str) -> R {
        writeln!(self.file, "== {} ==", description)?;
        let mut offset: usize = 0;
        while offset < self.chunk.code.len() {
            offset = self.disassemble_instruction(offset)?;
        }
        OK
    }
    pub fn disassemble_instruction(&mut self, offset: usize) -> RU {
        write!(self.file, "{:04} ", offset)?;
        if offset > 0 && self.chunk.get_line(offset) == self.chunk.get_line(offset - 1) {
            write!(self.file, "   | ")?;
        } else {
            write!(self.file, "{:4} ", self.chunk.get_line(offset))?;
        }
        let op_code = self.chunk.code[offset].try_into().unwrap();
        Ok(match op_code {
            OpCode::Constant => self.disassemble_constant(offset)?,
            OpCode::ConstantLong => self.disassemble_constant_long(offset)?,
            _ => self.simple_instruction(op_code.to_string().as_str(), offset)?,
        })
    }
    fn simple_instruction(&mut self, value: &str, offset: usize) -> RU {
        writeln!(self.file, "{}", value)?;
        return Ok(offset + 1);
    }
    fn disassemble_constant(&mut self, offset: usize) -> RU {
        let constant_offset = self.chunk.code[offset + 1];
        write!(self.file, "{:16} {:4} '", OpCode::Constant, constant_offset,)?;
        self.print_value(&self.chunk.constants[constant_offset as usize])?;
        write!(self.file, "'\n")?;
        return Ok(offset + 2);
    }
    fn disassemble_constant_long(&mut self, offset: usize) -> RU {
        let constant_offset = ((((self.chunk.code[offset + 1] as usize) << 8)
            + (self.chunk.code[offset + 2] as usize))
            << 8)
            + self.chunk.code[offset + 3] as usize;
        write!(
            self.file,
            "{:16} {:12} '",
            OpCode::ConstantLong,
            constant_offset
        )?;
        self.print_value(&self.chunk.constants[constant_offset])?;
        write!(self.file, "'\n")?;
        return Ok(offset + 4);
    }

    fn print_value(&mut self, value: &Value) -> R {
        write!(self.file, "{:?}", value)?;
        OK
    }

    fn new(chunk: &'a Chunk, file: &'a mut File) -> Self {
        Self { chunk, file }
    }
}
