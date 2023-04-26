use std::process::Command;

pub fn execute(string: &str) -> String {
    let mut executor = get_executor();

    let output = executor.arg(string)
        .output()
        .expect("Failed to execute");

    match String::from_utf8(output.stdout) {
        Ok(result) => {
            result
        }
        Err(e) => {
            e.to_string()
        }
    }
}

fn get_executor() -> Command {
    Command::new("powershell")
}