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

    /// Number of cells in memory
    #[structopt(short, long)]
    cells: Option<usize>,

    /// Activate debug mode
    #[structopt(short, long)]
    extensible: bool,
}

pub fn entrypoint(opt: Opt) -> Result<(), Box<dyn std::error::Error>> {
    let the_program = bft_types::Program::from_file(Path::new(&opt.input))?;
    let memory = bft_interp::VirtualMachine::<u8>::new(opt.cells, opt.extensible);
    memory.load_program(&the_program);
    Ok(())
}
