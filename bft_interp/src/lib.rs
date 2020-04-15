use bft_types::Instruction;
use bft_types::Program;

#[derive(Debug)]
pub struct VirtualMachine<'a, Number> {
    memory: Vec<Number>,
    head: usize,
    elastic: bool,
    program_counter: usize,
    program: &'a Program,
}

impl<Number: Clone + num_traits::Num> VirtualMachine<'_, Number> {
    pub fn new(program: &Program, size: Option<usize>, elastic: bool) -> VirtualMachine<Number> {
        let size = match size {
            Some(0) => 30000,
            Some(n) => n,
            None => 30000,
        };

        VirtualMachine {
            program,
            memory: vec![Number::zero(); size],
            head: 0,
            elastic,
            program_counter: 0,
        }
    }
    pub fn load_program(self: &Self, program: &Program) {
        let instructions = program.get_instructions();
        for inst in instructions {
            println!("{:?}", inst);
        }
    }
    pub fn move_head_left(&mut self) -> Result<(), VMError> {
        if self.head == 0 {
            return Err(VMError::HeadOutOfMemory(
                self.program.get_instructions()[self.program_counter],
            ));
        }
        self.head -= 1;
        Ok(())
    }
    pub fn move_head_right(&mut self) -> Result<(), VMError> {
        // Check if we are at the end of the memory
        if self.head == (self.memory.len() - 1) {
            if self.elastic {
                self.memory.push(Number::zero());
            } else {
                return Err(VMError::HeadOutOfMemory(
                    self.program.get_instructions()[self.program_counter],
                ));
            }
        }
        self.head += 1;
        Ok(())
    }
}

pub enum VMError {
    HeadOutOfMemory(Instruction),
}

#[cfg(test)]
mod tests {}
