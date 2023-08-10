use std::collections::HashMap;

pub fn about() -> String {
    let abt_str = "A simple library to run shell commands in rust programs";
    String::from(abt_str)
}

/// Shell command structure
/// Holds all the information needed to have the OS dispatch a process
pub struct ShellCommand {
    env_var: Option<HashMap<String, String>>,
    stdin: Option<String>,
    argv: Vec<String>,
}

impl ShellCommand {
    /// Create a new ShellCommand
    pub fn new(arg0: impl Into<String> + Copy) -> ShellCommand {
        ShellCommand {
            env_var: None,
            stdin: None,
            argv: vec![arg0.into()],
        }
    }

    /// Add arguments to a ShellCommand
    pub fn args<I, S>(self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let arg_addition: Vec<String> = args.into_iter().map(|item: S| item.into()).collect();

        let mut argv = self.argv;
        argv.extend(arg_addition);

        ShellCommand {
            env_var: self.env_var,
            stdin: self.stdin,
            argv,
        }
    }
}

mod tests;
