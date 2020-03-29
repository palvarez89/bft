use std::fs;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
enum RawInstruction {
    MoveLeft,
    MoveRight,
    Increment,
    Decrement,
    Input,
    Output,
    BeginLoop,
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
pub struct Instruction {
    instruction: RawInstruction,
    row: usize,
    column: usize,
}

#[derive(Debug)]
pub struct Program {
    filename: PathBuf,
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

    pub fn from_file(filename: &Path) -> std::io::Result<Program> {
        let filename = filename.to_path_buf();
        let content = fs::read_to_string(&filename)?;
        Ok(Program::new(filename, Self::extract_instrunctions(content)))
    }
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
