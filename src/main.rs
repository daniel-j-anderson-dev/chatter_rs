use std::{
    env,
    io::{
        BufRead,
        BufReader,
        stdin,
        stdout,
        Write,
    },
    net::{
        SocketAddr,
        TcpListener,
        TcpStream,
        ToSocketAddrs,
    },
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    match env::args().nth(1) {
        Some(argument) => match env::args().nth(2) {
            Some(ip) => match argument.as_str() {
                "serve" => serve(ip)?,
                "connect" => connect(ip)?,
                _ => print_usage("Unrecognized argument".into()),
            }
            None => print_usage("Missing IP argument".into()),
        }
        None => print_usage("Too few arguments".into()),
    }
    Ok(())
}

fn serve<A: ToSocketAddrs>(ip: A) -> Result<()> {
    let server_addr = get_scoket_addres(ip)?;
    let listener = TcpListener::bind(server_addr)?;
    println!("Listening on {server_addr}");
    let mut client_input = String::new();
    for (connection_id, possible_connection) in listener.incoming().enumerate() {
        let client = possible_connection?;
        println!("Client {} connected from {}", connection_id, client.peer_addr()?);
        let mut reader = BufReader::new(client);
        loop {
            match reader.read_line(&mut client_input) {
                Ok(0) => return Ok(()),
                Err(error) => return Err(error.into()),
                Ok(_bytes_read) => {},
            }
            print!("{client_input}");
            if &client_input == "quit\n" { break }
            client_input.clear();
        }
        client_input.clear();
        println!("\nWaiting for new connection");
    }
    Ok(())
}

fn connect<A: ToSocketAddrs>(ip: A) -> Result<()> {
    let server_addr = get_scoket_addres(ip)?;
    let mut server = TcpStream::connect(server_addr)?;
    println!("Connected to {server_addr}");
    let mut console_input = String::new();
    while &console_input != "quit\n" {
        console_input = get_console_input("")?;
        server.write_all(console_input.as_bytes())?;
    }
    Ok(())
}

fn get_scoket_addres<A: ToSocketAddrs>(ip: A) -> Result<SocketAddr> {
    return ip.to_socket_addrs()?.next().ok_or_else(|| "Could not parse ip".into());
}

fn get_console_input(prompt: &str) -> Result<String> {
    let mut console_input = String::new();
    stdout().write_all(prompt.as_bytes())?;
    stdout().flush()?;
    return match stdin().read_line(&mut console_input)? {
        0 => Err("Nothing read from stdin".into()),
        _bytes_read => Ok(console_input), 
    }
}

fn print_usage(error: Box<dyn std::error::Error>) {
    eprintln!("{error}\nUsage: network_test <OPTION> <IP>\nOptions:\n - connect\n - serve\n\nBE SURE TO INCLUDE A  PORT\nIPv4: ip:port\nIPv6: [ip]:port");
}