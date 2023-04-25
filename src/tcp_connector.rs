use std::io::{Error, Read, Write};
use std::net::TcpStream;
use std::str;


pub fn connect_to_ip_address(ip_address: &str) -> Result<TcpStream, Error> {
    let stream = TcpStream::connect(ip_address)?;

    Ok(stream)
}

pub fn read_from_server(mut stream: &TcpStream) -> Result<String, Error> {
    let mut data = [0 as u8; 1024];
    let bytes_read = stream.read(&mut data)?;
    if bytes_read == 0 {
        panic!("Disconnected")
    }
    let command = str::from_utf8(&data[..bytes_read]).unwrap();
    Ok(String::from(command))
}

pub fn send_to_server(mut stream: &TcpStream, message: &str) {
    stream.write(message.as_bytes());
}
