use std::any::Any;
use std::fmt::Debug;
use std::process::Command;
use powershell_script::{PsScript, PsScriptBuilder};

pub fn get_executor() -> Command {
    Command::new("powershell")
}

pub fn execute(command: &mut Command, string: &str) -> String {
    let output = command.arg(&string)
        .output()
        .expect("Failed to execute");

    match String::from_utf8(output.stdout) {
        Ok(result) => {
            result
        }
        Err(_) => {
            println!("None");
            String::new()
        }
    }
}
