use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "bft", about = "A bft program interpreter.")]
pub struct Opt {
    /// Input file with bft program
    #[structopt(name = "PROGRAM", parse(from_os_str))]
    pub input: PathBuf,

    /// Number of cells in memory
    #[structopt(short, long)]
    pub cells: Option<usize>,

    /// Activate debug mode
    #[structopt(short, long)]
    pub extensible: bool,
}
