mod configuration;
mod message;
mod network;
mod terminal;

use crate::{
    configuration::Configuration,
    message::Message,
    network::{receive_data, send_data},
};

use color_eyre::eyre::Result;

fn main() -> Result<()> {
    //set up panic/error handlers
    color_eyre::install()?;

    // load configuration from command line args
    let config = Configuration::from_args()?;

   dbg!(config); 

    return Ok(());
}
