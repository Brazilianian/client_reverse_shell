use std::io::{Error, Read, Write};
use std::net::TcpStream;
use std::str;


pub fn connect_to_ip_address(ip_address: &str) -> Result<TcpStream, Error> {
    println!("Trying to connect {}", &ip_address);
    let stream = TcpStream::connect(ip_address)?;

    Ok(stream)
}

pub fn read_from_server(mut stream: &TcpStream) -> Result<String, String> {
    let mut data = [0; 16384];
    match stream.read(&mut data) {
        Ok(bytes_read) => {
            if bytes_read == 0 {
                return Err(String::from("Disconnected"));
            }
            let command = str::from_utf8(&data[..bytes_read]).unwrap();
            Ok(String::from(command))
        }
        Err(_) => {
            return Err(String::from("Disconnected"));
        }
    }

}

pub fn send_to_server(mut stream: &TcpStream, message: &str) {
    if message.len() == 0 {
        let mut buf: [u8; 24] = [0; 24];
        stream.write(&mut buf).unwrap();
        return;
    }
    stream.write(message.as_bytes()).unwrap();
}
