use crate::cli::*;
use crate::errors::Result;

mod constant;
mod cli;
mod block;
mod errors;
mod blockchain;
mod transaction;
mod test;

fn main() -> Result<()> {
    let mut cli = Cli::new()?;
    cli.run()?;

    Ok(())
}