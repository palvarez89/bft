//! Definition of types to be used in our bft program

#![deny(missing_docs)]

use std::fs;
use std::path::Path;
use std::path::PathBuf;

/// A raw instruction of a bft program
#[derive(Debug, PartialEq)]
pub enum RawInstruction {
    /// decrement the data pointer (to point to the next cell to the left) `<`
    MoveLeft,
    /// increment the data pointer (to point to the next cell to the right) `>`
    MoveRight,
    /// increment (increase by one) the byte at the data pointer `+`
    Increment,
    /// decrement (decrease by one) the byte at the data pointer `-`
    Decrement,
    /// output the byte at the data pointer `.`
    Input,
    /// accept one byte of input, storing its value in the byte at the data pointer `,`
    Output,
    /// if the byte at the data pointer is zero, jump to end `[`
    BeginLoop,
    /// if the byte at the data pointer is nonzero, jump to beginning `]`
    EndLoop,
}

impl RawInstruction {
    fn from_char(c: char) -> Option<RawInstruction> {
        match c {
            '<' => Some(RawInstruction::MoveLeft),
            '>' => Some(RawInstruction::MoveRight),
            '+' => Some(RawInstruction::Increment),
            '-' => Some(RawInstruction::Decrement),
            ',' => Some(RawInstruction::Input),
            '.' => Some(RawInstruction::Output),
            '[' => Some(RawInstruction::BeginLoop),
            ']' => Some(RawInstruction::EndLoop),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
/// An instruction of a bft program with metadata
pub struct Instruction {
    /// The raw instructin itself
    instruction: RawInstruction,
    /// Row where the instruction is located
    row: usize,
    /// Column where the instruction is located
    column: usize,
}

#[derive(Debug)]
/// A bft program
pub struct Program {
    /// The path from where the program was loaded
    filename: PathBuf,
    /// A vector of the instructions of the program
    instructions: Vec<Instruction>,
}

impl Program {
    fn new(filename: PathBuf, instructions: Vec<Instruction>) -> Program {
        Program {
            filename,
            instructions,
        }
    }

    fn extract_instrunctions(program_string: String) -> Vec<Instruction> {
        let mut instructions: Vec<Instruction> = Vec::new();
        for (row, line) in program_string.lines().enumerate() {
            for (column, c) in line.chars().enumerate() {
                let inst = RawInstruction::from_char(c);
                if let Some(instruction) = inst {
                    instructions.push(Instruction {
                        instruction,
                        row: row + 1,
                        column: column + 1,
                    })
                }
            }
        }
        instructions
    }

    /// Function to load a program forom a Path
    /// ```
    /// # use bft_types::Program;
    /// use std::path::Path;
    /// let path = Path::new("program.bft");
    /// let program = Program::from_file(path);
    /// ```
    pub fn from_file(filename: &Path) -> std::io::Result<Program> {
        let filename = filename.to_path_buf();
        let content = fs::read_to_string(&filename)?;
        Ok(Program::new(filename, Self::extract_instrunctions(content)))
    }

    /// Getter of the instructions vector as a slice
    pub fn get_instructions(self: &Self) -> &[Instruction] {
        &self.instructions[..]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_char() {
        assert!(RawInstruction::from_char('o').is_none());
        assert_eq!(
            RawInstruction::from_char('<').unwrap(),
            RawInstruction::MoveLeft
        );
        assert_eq!(
            RawInstruction::from_char('>').unwrap(),
            RawInstruction::MoveRight
        );
        assert_eq!(
            RawInstruction::from_char('+').unwrap(),
            RawInstruction::Increment
        );
        assert_eq!(
            RawInstruction::from_char('-').unwrap(),
            RawInstruction::Decrement
        );
        assert_eq!(
            RawInstruction::from_char('[').unwrap(),
            RawInstruction::BeginLoop
        );
        assert_eq!(
            RawInstruction::from_char(']').unwrap(),
            RawInstruction::EndLoop
        );
        assert_eq!(
            RawInstruction::from_char(',').unwrap(),
            RawInstruction::Input
        );
        assert_eq!(
            RawInstruction::from_char('.').unwrap(),
            RawInstruction::Output
        );
    }
    #[test]
    fn test_extract_instructions() {
        let a_program = "<   >\n[foo]";
        let instsructions = Program::extract_instrunctions(a_program.to_owned());
        assert_eq!(
            instsructions[0],
            Instruction {
                row: 1,
                column: 1,
                instruction: RawInstruction::MoveLeft
            }
        );
        assert_eq!(
            instsructions[1],
            Instruction {
                row: 1,
                column: 5,
                instruction: RawInstruction::MoveRight
            }
        );
        assert_eq!(
            instsructions[2],
            Instruction {
                row: 2,
                column: 1,
                instruction: RawInstruction::BeginLoop
            }
        );
        assert_eq!(
            instsructions[3],
            Instruction {
                row: 2,
                column: 5,
                instruction: RawInstruction::EndLoop
            }
        );
    }
}
