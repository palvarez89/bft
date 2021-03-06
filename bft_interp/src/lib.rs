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
    pub fn load_program(&self, program: &Program) {
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
    /// Read byte on memory cell pointed by head
    pub fn read_into_head<T: Read>(&mut self, reader: &mut T) -> Result<(), VMError> {
        let result = match self.memory.get_mut(self.head) {
            Some(cell) => cell.from_reader(reader),
            None => Ok(()),
        };
        match result {
            Err(e) => Err(VMError::IOError(
                e,
                self.program.instructions()[self.program_counter],
            )),
            Ok(_) => Ok(()),
        }
    }
    /// Write cell pointed by head into output
    pub fn write_from_head<T: Write>(&self, writer: &mut T) -> Result<(), VMError> {
        let result = match self.memory.get(self.head) {
            Some(cell) => cell.to_writer(writer),
            None => Ok(()),
        };
        match result {
            Err(e) => Err(VMError::IOError(
                e,
                self.program.instructions()[self.program_counter],
            )),
            Ok(_) => Ok(()),
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
    fn from_reader<T: Read>(&mut self, reader: &mut T) -> Result<(), std::io::Error>;
    /// Write cell contents
    fn to_writer<T: Write>(&self, writer: &mut T) -> Result<(), std::io::Error>;
}

impl CellKind for u8 {
    fn wrapping_increment(&mut self) {
        *self = self.wrapping_add(1);
    }
    fn wrapping_decrement(&mut self) {
        *self = self.wrapping_sub(1);
    }
    fn from_reader<T: Read>(&mut self, reader: &mut T) -> Result<(), std::io::Error> {
        let mut buffer = [0; 1];
        reader.read_exact(&mut buffer)?;
        *self = buffer[0];
        Ok(())
    }
    fn to_writer<T: Write>(&self, writer: &mut T) -> Result<(), std::io::Error> {
        Ok(writer.write_all(&[*self])?)
    }
}
/// Error definitions for VirtualMaachine
pub enum VMError {
    /// Error for when the head moves out of the memory
    HeadOutOfMemory(Instruction),
    /// IO error when reading or writing bytes
    IOError(std::io::Error, Instruction),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8_cellkind_decrement() {
        let mut cell = 1u8;
        cell.wrapping_decrement();
        assert_eq!(cell, 0);

        let mut cell = 0u8;
        cell.wrapping_decrement();
        assert_eq!(cell, std::u8::MAX);
    }
    #[test]
    fn test_u8_cellkind_increment() {
        let mut cell = 0u8;
        cell.wrapping_increment();
        assert_eq!(cell, 1u8);

        let mut cell = std::u8::MAX;
        cell.wrapping_increment();
        assert_eq!(cell, 0u8);
    }
    #[test]
    fn test_u8_cellkind_from_reader_success() {
        let input = "This string will be read";
        // Don't run as_bytes in the string literal because of clippy
        let mut b = input.as_bytes();
        let mut cell = 0u8;
        assert!(cell.from_reader(&mut b).is_ok());
        assert_eq!(cell, b'T');
    }
    #[test]
    fn test_u8_cellkind_from_reader_failure() {
        let input = "";
        // Don't run as_bytes in the string literal because of clippy
        let mut b = input.as_bytes();
        let mut cell = 0u8;
        assert!(cell.from_reader(&mut b).is_err());
    }
    #[test]
    fn test_u8_cellkind_to_writer() {
        use std::io::Cursor;
        let mut buff = Cursor::new(vec![0u8; 5]);
        assert!(105u8.to_writer(&mut buff).is_ok());
        assert!(114u8.to_writer(&mut buff).is_ok());
        assert!(111u8.to_writer(&mut buff).is_ok());
        assert!(110u8.to_writer(&mut buff).is_ok());
        assert_eq!(&buff.get_ref()[..], b"iron\0");
    }
}
