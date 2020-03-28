use std::fs;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
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

#[derive(Debug)]
struct Instruction {
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
    pub fn from_file(filename: &Path) -> std::io::Result<Program> {
        let filename = filename.to_path_buf();
        let content = fs::read_to_string(&filename)?;
        let mut instructions: Vec<Instruction> = Vec::new();

        for (row, line) in content.lines().enumerate() {
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
        Ok(Program::new(filename, instructions))
    }
}
#[cfg(test)]
mod tests {}
