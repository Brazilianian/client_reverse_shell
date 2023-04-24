use std::io::Write;
use std::net::TcpStream;

pub fn connect_to_ip_address(ip_address: &str) -> Result<TcpStream, E> {
    TcpStream::connect(ip_address)
}

pub fn send_to_server(mut stream: TcpStream, message: &str) -> Result<u8, E> {
    stream.write(message.as_bytes())
}
