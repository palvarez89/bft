use std::path::PathBuf;

#[derive(Debug)]
enum Instruction {
    MoveLeft,
    MoveRight,
    Increment,
    Decrement,
    Input,
    Output,
    BeginLoop,
    EndLoop,
}

#[derive(Debug)]
struct Program {
    filename: PathBuf,
    instructions: Vec<Instruction>,
}

#[cfg(test)]
mod tests {}
