//! Definition of types to be used in our bft program

#![deny(missing_docs)]

use bft_types::Instruction;
use bft_types::Program;
use std::fmt;
use std::io::{Read, Write};

/// The virtual machine (Memory, PC, etc) of the interpreter
#[derive(Debug)]
pub struct VirtualMachine<'a, Number> {
    memory: Vec<Number>,
    head: usize,
    elastic: bool,
    program_counter: usize,
    program: &'a Program,
}

impl<'a, Number> VirtualMachine<'a, Number>
where
    Number: PartialEq + Clone + CellKind + num_traits::Num,
{
    /// Constructor of the VirtualMachine
    pub fn new(program: &'a Program, size: Option<usize>, elastic: bool) -> Self {
        let size = match size {
            Some(0) => 30000,
            Some(n) => n,
            None => 30000,
        };

        Self {
            program,
            memory: vec![Number::zero(); size],
            head: 0,
            elastic,
            program_counter: 0,
        }
    }
    /// Load program (not implemented yet)
    pub fn load_program(self: &Self, program: &Program) {
        let instructions = program.instructions();
        for inst in instructions {
            println!("{:?}", inst);
        }
    }
    /// Move head to the left
    pub fn move_head_left(&mut self) -> Result<(), VMError> {
        if self.head == 0 {
            Err(VMError::HeadOutOfMemory(
                self.program.instructions()[self.program_counter],
            ))
        } else {
            self.head -= 1;
            Ok(())
        }
    }
    /// Move head to the right
    pub fn move_head_right(&mut self) -> Result<(), VMError> {
        // Check if we can increase the head
        if self.elastic || self.head != (self.memory.len() - 1) {
            // If head is at the end of the memory, considering
            // memory is elasic, increase the memory.
            if self.head == self.memory.len() - 1 {
                self.memory.push(Number::zero())
            }
            self.head += 1;
            Ok(())
        } else {
            Err(VMError::HeadOutOfMemory(
                self.program.instructions()[self.program_counter],
            ))
        }
    }
}

/// Define functions that our VirtualMachine cells
/// should implement
pub trait CellKind: fmt::Debug {
    /// Increment by one the value of the cell
    fn wrapping_increment(&mut self);
    /// Decrement by one the value of the cell
    fn wrapping_decrement(&mut self);
    /// Load cell from reader
    fn from_reader(&mut self, reader: Box<dyn Read>) -> Result<(), std::io::Error>;
    /// Write cell contents
    fn to_writer(&self, writer: Box<dyn Write>) -> Result<(), std::io::Error>;
}

impl CellKind for u8 {
    fn wrapping_increment(&mut self) {
        *self = self.wrapping_add(1);
    }
    fn wrapping_decrement(&mut self) {
        *self = self.wrapping_sub(1);
    }
    fn from_reader(&mut self, mut reader: Box<dyn Read>) -> Result<(), std::io::Error> {
        let mut buffer = [0; 1];
        Ok(reader.read_exact(&mut buffer)?)
    }
    fn to_writer(&self, mut writer: Box<dyn Write>) -> Result<(), std::io::Error> {
        Ok(writer.write_all(&[*self])?)
    }
}
/// Error definitions for VirtualMaachine
pub enum VMError {
    /// Error for when the head moves out of the memory
    HeadOutOfMemory(Instruction),
}

#[cfg(test)]
mod tests {}
