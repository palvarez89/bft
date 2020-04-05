use bft_interp;
use bft_types;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "bft", about = "A bft program interpreter.")]
pub struct Opt {
    /// Input file with bft program
    #[structopt(name = "PROGRAM", parse(from_os_str))]
    input: PathBuf,
}

pub fn entrypoint(opt: Opt) -> Result<(), Box<dyn std::error::Error>> {
    let the_program = bft_types::Program::from_file(Path::new(&opt.input))?;
    let memory = bft_interp::VirtualMachine::<u8>::new(0, false);
    memory.load_program(&the_program);
    Ok(())
}
