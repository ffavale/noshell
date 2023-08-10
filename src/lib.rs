pub fn about() -> String {
    let abt_str = "A simple library to run shell commands in rust programs";
    String::from(abt_str)
}

pub struct ShellCommand {
    stdin: Option<String>,
    argv: Vec<String>,
    env_var: Vec<(String, String)>,
}

impl ShellCommand {
    pub fn new(arg0: impl Into<String> + Copy) -> ShellCommand {
        ShellCommand {
            stdin: None,
            argv: vec![arg0.into()],
            env_var: Vec::new(),
        }
    }
}

mod tests;
