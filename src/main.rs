use std::path::Path;
use structopt::StructOpt;
mod cli;

fn main() {
    let opt = cli::Opt::from_args();
    let retcode = match run_bft(&opt) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!(
                "bft: Error in input file {filename}. {error}",
                filename = opt.input.display(),
                error = e
            );
            1
        }
    };
    std::process::exit(retcode);
}

fn run_bft(args: &cli::Opt) -> Result<(), Box<dyn std::error::Error>> {
    let mut the_program = bft_types::Program::from_file(Path::new(&args.input))?;
    the_program.check_syntax()?;

    let memory = bft_interp::VirtualMachine::<u8>::new(&the_program, args.cells, args.extensible);
    Ok(())
}
