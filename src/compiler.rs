use crate::{
    chunk::{Chunk, OpCode, Value},
    scanner::{Scannable, Scanner},
    token::{LineNo, Token},
    token_type::TokenType,
};

struct Compiler<'a> {
    scanner: Scanner<'a>,
    chunk: Chunk,
    previous: Option<Token<'a>>,
    current: Option<Token<'a>>,
}

const OK: R = Ok(());

type R = Result<(), CompilerError>;

#[derive(Debug)]
pub struct CompilerError {
    pub message: String,
}

impl<'a> Compiler<'a> {
    fn new(scanner: Scanner<'a>) -> Self {
        Self {
            scanner,
            chunk: Chunk::new_chunk(),
            previous: None,
            current: None,
        }
    }
    fn compile(mut self) -> Result<Chunk, CompilerError> {
        self.advance()?;
        self.expression()?;
        self.consume(TokenType::EndOfFile, "Expect end of expression.")?;
        self.chunk
            .write_op_code(OpCode::Return, self.previous.map_or(0, |t| t.line as usize));
        Ok(self.chunk)
    }

    fn line(&self) -> Result<usize, CompilerError> {
        self.current
            .map(|t| t.line as usize)
            .ok_or("No line currently".into())
    }

    fn expression(&mut self) -> Result<(), CompilerError> {
        let token = self.current.ok_or("Expected token for expression".into())?;
        match token.token_type {
            TokenType::Number => {
                let value: Value = token.content.parse().unwrap();
                let line = token.line as usize;
                self.chunk.write_constant(value, line);
                self.advance()?;
            }
            x => self.report_error(&format!("Don't know token yet: {:#?}", x))?,
        }
        Ok(())
    }

    fn consume(&mut self, expected: TokenType, error_message_input: &str) -> R {
        let mut error_message = format!(
            "{} - Expected token type {:#?}",
            error_message_input, expected
        );
        if self.current.is_none() {
            error_message += ": No tokens left";
            self.report_error(&error_message)?;
        }
        let token = self.current.unwrap();
        if token.token_type != expected {
            error_message += format!(": Got {:#?}", token.token_type).as_str();
            self.report_error(&error_message)?;
        }
        self.advance()?;
        OK
    }

    fn report_error(&mut self, format: &String) -> R {
        Err(format.as_str().into())
    }

    fn advance(&mut self) -> R {
        self.previous = self.current;
        self.current = self.scanner.next();
        OK
    }
}

impl Into<CompilerError> for &str {
    fn into(self) -> CompilerError {
        CompilerError {
            message: self.to_string(),
        }
    }
}

pub trait Compiled {
    fn compile(&self) -> Result<Chunk, CompilerError>;
}

impl Compiled for &str {
    fn compile(&self) -> Result<Chunk, CompilerError> {
        Compiler::new(self.scanner()).compile()
    }
}
