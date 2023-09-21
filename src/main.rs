use std::{
    env,
    io::{
        Read,
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
    let args = env::args().collect::<Vec<String>>();
    match args.get(1) {
        Some(argument) => match args.get(2) {
            Some(ip) => match argument.as_str() {
                "serve" => serve(ip)?,
                "connect" => connect(ip)?,
                _ => print_usage("Unrecognized argument".into()),
            }
            None => print_usage("Missing IP argument".into()),
        }
        None => print_usage("Missing arguments".into()),
    }
    Ok(())
}

fn serve<A: ToSocketAddrs>(ip: A) -> Result<()> {
    let server_addr = get_scoket_addres(ip)?;
    let listener = TcpListener::bind(server_addr)?;
    println!("Listening on {server_addr}");
    for (client_id, possible_connection) in listener.incoming().enumerate() {
        let mut client = possible_connection?;
        println!("\nClient {} connected from {}", client_id, client.peer_addr()?);
        loop {
            let mut client_input = vec![0;1024];
            match client.read(&mut client_input) {
                Ok(0) => break,
                Ok(_bytes_read) => {},
                Err(_error) if _error.kind() == std::io::ErrorKind::ConnectionReset => break,
                Err(error) => return Err(error.into()),
            }
            // if &client_input == b"quit\n" { break }
            let server_output = client_input.clone();
            client.write_all(&server_output)?;
            print!("{}: {}", client_id, String::from_utf8(server_output)?);
        }
        println!("Client {client_id} disconected\nWaiting for new connection");
    }
    Ok(())
}

fn connect<A: ToSocketAddrs>(ip: A) -> Result<()> {
    let server_addr = get_scoket_addres(ip)?;
    let mut server = TcpStream::connect(server_addr)?;
    println!("Connected to {server_addr}");
    let mut server_input = vec![0;1024];
    loop {
        let console_input = get_console_input("> ")?;
        server.write_all(console_input.as_bytes())?;
        if &console_input == "quit\n" { return Ok(()) }
        match server.read(&mut server_input)? {
            0 => return Err("Nothing read from server, did it disconect?".into()),
            _bytes_read => {}
        }
        println!("$ {}", String::from_utf8(server_input.clone())?);
    }
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