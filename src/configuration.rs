use clap::{Arg, ArgAction, ArgMatches, Command};
use color_eyre::eyre::{eyre, Report, Result};
use std::net::SocketAddr;

#[derive(Debug)]
pub struct Configuration {
    is_host: bool,
    ip: SocketAddr,
}
impl Configuration {
    pub fn from_args() -> Result<Self> {
        let arg_matches = Command::new("one-on-one")
            .version("0.1.0")
            .author("Daniel Anderson")
            .about("A simple program for one on one communication")
            .args(&[
                Arg::new("host/join")
                    .required(true)
                    .help("Host a room or join a room"),
                Arg::new("ip")
                    .action(ArgAction::Set)
                    .default_value("[::1]:8080"),
            ])
            .get_matches();

        return arg_matches.try_into();
    }
}
impl TryFrom<ArgMatches> for Configuration {
    type Error = Report;
    fn try_from(arg_matches: ArgMatches) -> Result<Self> {
        let is_host = match arg_matches
            .get_one::<String>("host/join")
            .expect("host/join is a required value")
        {
            string if string == "host" => true,
            string if string == "join" => false,
            string => {
                return Err(eyre!(
                    "Unrecognized option: {}. Valid values are `host` and `join`",
                    string
                ))
            }
        };

        let ip = arg_matches
            .get_one::<String>("ip")
            .expect("ip arg has a default value");

        let ip = match ip.parse::<SocketAddr>() {
            Ok(socket_address) => socket_address,
            Err(ip_parse_error) => {
                return Err(eyre!("{} is an invalid ip address: {}", ip, ip_parse_error))
            }
        };

        return Ok(Configuration { is_host, ip });
    }
}
