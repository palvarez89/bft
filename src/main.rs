use structopt::StructOpt;
mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = cli::Opt::from_args();
    cli::entrypoint(opt)
}
