use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

pub fn send_data(address: &str, data: &[u8]) -> Result<(), std::io::Error> {
    let mut connection = TcpStream::connect(address)?;

    connection.write_all(data)?;

    return Ok(());
}

pub fn receive_data(address: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut listener = TcpListener::bind(address)?;

    let (mut connection, _connection_address) = listener.accept()?;

    let mut input = Vec::new();

    connection.read_to_end(&mut input)?;

    return Ok(input);
}
