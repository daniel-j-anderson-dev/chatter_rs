mod configuration;
mod network;
mod terminal;

use crate::configuration::Configuration;

use color_eyre::eyre::Result;

fn main() -> Result<()> {
    //set up panic/error handlers
    color_eyre::install()?;

    // load configuration from command line args
    let config = Configuration::from_args()?;

    dbg!(config);

    return Ok(());
}
