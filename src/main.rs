use bft_types;
use std::env::args;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let fname = args().nth(1).ok_or("Expected filename")?;

    let the_program = bft_types::Program::from_file(Path::new(&fname));
    println!("{:?}", the_program);
    Ok(())
}
