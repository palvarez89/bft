use bft_interp;
use bft_types;
use std::env::args;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fname = args().nth(1).ok_or("Expected filename")?;

    let the_program = bft_types::Program::from_file(Path::new(&fname))?;
    let memory = bft_interp::VirtualMachine::<u8>::new(0, false);
    memory.load_program(&the_program);
    Ok(())
}
