//! Definition of types to be used in our bft program

#![deny(missing_docs)]

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

/// A raw instruction of a bft program
#[derive(Debug, PartialEq, Copy, Clone)]
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

#[derive(Debug, PartialEq, Copy, Clone)]
/// An instruction of a bft program with metadata
pub struct Instruction {
    /// The raw instructin itself
    instruction: RawInstruction,
    /// Row where the instruction is located
    row: usize,
    /// Column where the instruction is located
    column: usize,
}

/// Program error definitions
#[derive(Debug, PartialEq)]
pub enum ProgramError {
    /// Syntax error when a loop is left open
    UnclosedLoop(Instruction),
    /// Syntax error when a closing bracked dosen't have a matching opening one
    UnopenedLoop(Instruction),
}

impl Error for ProgramError {}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            ProgramError::UnclosedLoop(i) => format!(
                "Found unclosed bracket at row {row} column {column}",
                row = i.row,
                column = i.column
            ),
            ProgramError::UnopenedLoop(i) => format!(
                "Couldn't find matching opening bracket for bracket at row {row}, column {column}",
                row = i.row,
                column = i.column
            ),
        };
        write!(f, "{}", msg)
    }
}
#[derive(Debug)]
/// A bft program
pub struct Program {
    /// The path from where the program was loaded
    filename: PathBuf,
    /// A vector of the instructions of the program
    instructions: Vec<Instruction>,
    loop_gotos: HashMap<usize, usize>,
}

impl Program {
    fn new(filename: PathBuf, instructions: Vec<Instruction>) -> Program {
        Program {
            filename,
            instructions,
            loop_gotos: HashMap::new(),
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
    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    /// Getter for the maching bracket given an instuction index
    pub fn get_matching_bracket(&self, instruction_index: usize) -> std::option::Option<usize> {
        match self.loop_gotos.get(&instruction_index) {
            Some(n) => Some(n.to_owned()),
            None => None,
        }
    }

    /// Check if the syntax of the program is correct
    pub fn check_syntax(&mut self) -> Result<(), ProgramError> {
        let mut bracket_index = Vec::<usize>::new();
        for (index, current_inst) in self.instructions.iter().enumerate() {
            if let RawInstruction::BeginLoop = current_inst.instruction {
                bracket_index.push(index);
            } else if let RawInstruction::EndLoop = current_inst.instruction {
                let pop = bracket_index.pop();
                match pop {
                    Some(beginloop_index) => {
                        self.loop_gotos.insert(beginloop_index, index);
                        self.loop_gotos.insert(index, beginloop_index);
                    }
                    None => return Err(ProgramError::UnopenedLoop(*current_inst)),
                }
            }
        }

        match bracket_index.pop() {
            Some(index) => {
                let inst = self.instructions().get(index).unwrap();
                Err(ProgramError::UnclosedLoop(*inst))
            }
            None => Ok(()),
        }
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
        let instructions = Program::extract_instrunctions(a_program.to_owned());
        assert_eq!(
            instructions[0],
            Instruction {
                row: 1,
                column: 1,
                instruction: RawInstruction::MoveLeft
            }
        );
        assert_eq!(
            instructions[1],
            Instruction {
                row: 1,
                column: 5,
                instruction: RawInstruction::MoveRight
            }
        );
        assert_eq!(
            instructions[2],
            Instruction {
                row: 2,
                column: 1,
                instruction: RawInstruction::BeginLoop
            }
        );
        assert_eq!(
            instructions[3],
            Instruction {
                row: 2,
                column: 5,
                instruction: RawInstruction::EndLoop
            }
        );
    }
}
