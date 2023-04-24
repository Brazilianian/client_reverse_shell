use powershell_script::PsScriptBuilder;

pub fn execute(command: &str) -> &str {
    let ps = PsScriptBuilder::new()
        .no_profile(true)
        .non_interactive(true)
        .hidden(false)
        .print_commands(false)
        .build();

    ps.run(command).unwrap().stdout().unwrap().as_str()
}
