use crate::cli::Cli;
use crate::errors::Result;

mod block;

mod errors;
mod blockchain;
mod cli;
mod transaction;
mod wallet;
mod tx;


fn main()->Result<()> {
    let mut cli = Cli::new()?;
    cli.run()?;

    Ok(())

}
