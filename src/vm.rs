// vm.rs - Stack-based Bytecode Virtual Machine
use crate::{Chunk, Error, OpCode, Value};
use round::round;

const STACK_SIZE: usize = 256;

#[allow(clippy::upper_case_acronyms)]
pub struct VM {
    pub stack: Vec<Value>,
    pub positions: Vec<(usize, usize)>,
    pub result: Option<Value>,
    chunk: Chunk,
    verbose: bool,
}

impl VM {
    pub fn new(verbose: bool) -> Self {
        // Create a new virtual machine
        Self {
            stack: Vec::with_capacity(STACK_SIZE),
            positions: Vec::with_capacity(STACK_SIZE),
            chunk: Chunk::new(0),
            result: None,
            verbose,
        }
    }

    pub fn run(&mut self, chunk: Chunk) -> Result<(), Error> {
        // Execute a bytecode chunk
        self.chunk = chunk;
        for (col, len, instruction) in self.chunk.code.clone() {
            // Display stack if verbose option specified
            if self.verbose {
                for slot in &self.stack {
                    print!("[ {} ]", slot);
                }
                println!();
            }
            // Carry out instruction
            match instruction {
                OpCode::OpConstant(idx) => {
                    let constant = self.chunk.constants[idx as usize].clone();
                    self.stack.push(constant);
                    self.positions.push((col, len));
                }
                OpCode::OpNot => {
                    let pop = self.stack.pop().unwrap();
                    self.stack.push(if let Value::Boolean(b) = pop { 
                        Value::Boolean(!b)
                    } else { 
                        self.stack.push(pop);
                        return Err(Error::MismatchedTypes(
                            self.chunk.line,
                            self.get_col(0).0,
                            self.get_col(0).1,
                            "Operand must be a boolean".to_string()
                        ))
                    });
                }
                OpCode::OpNegate => if let Some(Value::Number(_)) = self.peek(0) {
                    let operand = -self.stack.pop().unwrap();
                    self.stack.push(operand);
                    self.positions.push((col, len));
                } else {
                    return Err(Error::MismatchedTypes(
                        self.chunk.line,
                        self.get_col(0).0,
                        self.get_col(0).1,
                        "Operand must be a number".to_string()
                    ));
                }
                OpCode::OpAdd => self.bin_op("+", col)?,
                OpCode::OpSub => self.bin_op("-", col)?,
                OpCode::OpMul => self.bin_op("*", col)?,
                OpCode::OpDiv => self.bin_op("/", col)?,
                OpCode::OpMod => self.bin_op("%", col)?,
                OpCode::OpPow => self.bin_op("^", col)?,
                OpCode::OpNil => {
                    self.stack.push(Value::Nil);
                    self.positions.push((col, len));
                }
                OpCode::OpTrue => {
                    self.stack.push(Value::Boolean(true));
                    self.positions.push((col, len));
                }
                OpCode::OpFalse => {
                    self.stack.push(Value::Boolean(false));
                    self.positions.push((col, len));
                }
                OpCode::OpEqual => {
                    let a = self.stack.pop().unwrap();
                    let c = self.positions.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    let d = self.positions.pop().unwrap();
                    self.stack.push(Value::Boolean(a == b));
                    self.positions.push((d.0, c.0 - d.0 + c.1));
                }
                OpCode::OpGreater => self.bin_op(">", col)?,
                OpCode::OpLess => self.bin_op("<", col)?,
                OpCode::OpReturn => {
                    self.positions.pop();
                    let popped = self.stack.pop();
                    let result = if let Some(Value::Number(f)) = popped {
                        Value::Number(round(f, 5))
                    } else {
                        popped.unwrap()
                    };
                    println!("{}", result);
                    self.result = Some(result);
                    break
                }
            }
        }
        Ok(())
    }
    
    fn bin_op(&mut self, op: &str, loc: usize) -> Result<(), Error> {
        // Execute a binary operation
        let (a, b) = (self.peek(0), self.peek(1));
        let (c, d) = (self.get_col(0), self.get_col(1));
        if let (Some(&Value::Number(_)), Some(&Value::Number(_))) = (a, b) {
            self.positions.pop();
            let b = self.stack.pop().unwrap();
            self.positions.pop();
            let a = self.stack.pop().unwrap();
            match op {
                "+" => self.stack.push(a + b),
                "-" => self.stack.push(a - b),
                "*" => self.stack.push(a * b),
                "/" => self.stack.push(a / b),
                "%" => self.stack.push(a % b),
                "^" => self.stack.push(a ^ b),
                ">" => self.stack.push(Value::Boolean(a > b)),
                "<" => self.stack.push(Value::Boolean(a < b)),
                _ => unreachable!(),
            }
            self.positions.push((d.0, c.0 - d.0 + c.1));
            Ok(())
        } else if let (Some(&Value::String(_)), Some(&Value::String(_))) = (a, b) {
            if op == "+" {
                self.positions.pop();
                let b = self.stack.pop().unwrap();
                self.positions.pop();
                let a = self.stack.pop().unwrap();
                self.stack.push(a + b);
                self.positions.push((d.0, c.0 - d.0 + c.1));
                Ok(())
            } else {
                return Err(Error::ImpossibleOperation(
                    self.chunk.line, 
                    loc,
                    1,
                    op.to_string(),
                ))
            }
        } else {
            return Err(Error::MismatchedTypes(
                self.chunk.line, 
                if let Some(&Value::Number(_)) = b { c.0 } else { d.0 },
                if let Some(&Value::Number(_)) = b { c.1 } else { d.1 },
                "Operands must be either numbers or strings".to_string()
            ))
        }
    }

    fn peek(&self, distance: usize) -> Option<&Value> {
        self.stack.get(self.stack.len() - 1 - distance)
    }

    pub fn get_col(&self, distance: usize) -> (usize, usize) {
        *self.positions.get(self.stack.len() - 1 - distance).unwrap()
    }

    pub fn reset(&mut self) {
        self.positions.clear();
        self.stack.clear();
    }
}

