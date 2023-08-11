use std::{
    collections::HashMap,
    io::Write,
    process::{Command, Stdio},
};

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

#[derive(Debug)]
pub struct ShellCommandOutput {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
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

    /// Add envvars to a ShellCommand
    pub fn envs<I, K, V>(self, envs: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        let env_addition: Vec<(String, String)> = envs
            .into_iter()
            .map(|(key, value): (K, V)| (key.into(), value.into()))
            .collect();

        let mut env_var = match self.env_var {
            Some(v) => v,
            None => HashMap::new(),
        };

        for (key, value) in env_addition {
            env_var.insert(key, value);
        }

        ShellCommand {
            env_var: Some(env_var),
            stdin: self.stdin,
            argv: self.argv,
        }
    }

    /// Pipe a string into a ShellCommand
    pub fn pipe_string(self, input: impl Into<String>) -> Self {
        ShellCommand {
            env_var: self.env_var,
            stdin: Some(input.into()),
            argv: self.argv,
        }
    }

    /// Run the ShellCommand
    pub fn run(self) -> Result<ShellCommandOutput, ShellCommandOutput> {
        let cmd_arg0 = &self.argv[0];
        let mut cmd = Command::new(cmd_arg0);

        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        cmd.args(&self.argv[1..]);

        if let Some(e) = self.env_var {
            cmd.envs(e);
        }

        let mut spawned = cmd.spawn().expect(EXP_SP);

        if let Some(s) = self.stdin {
            if let Some(mut stdin) = spawned.stdin.take() {
                stdin
                    .write_all(s.as_bytes())
                    .expect("Failed to write to stdin");
                drop(stdin);
            }
        }

        let output = spawned.wait_with_output().expect(EXP_EXE);

        let sc_output = ShellCommandOutput {
            success: output.status.success(),
            stdout: String::from_utf8(output.stdout).expect(EXP_SOUT),
            stderr: String::from_utf8(output.stderr).expect(EXP_SERR),
        };

        if !output.status.success() {
            return Err(sc_output);
        }

        Ok(sc_output)
    }

    /// Extract the output of the command and ignore the success
    pub fn result(self) -> ShellCommandOutput {
        match self.run() {
            Ok(s) => s,
            Err(s) => s,
        }
    }

    /// What is the name of the program this command will call
    pub fn command(&self) -> &String {
        &self.argv[0]
    }
}

static EXP_SOUT: &str = "Could not format stdout into utf-8";
static EXP_SERR: &str = "Could not format stderr into utf-8";
static EXP_SP: &str = "Failed to spawn child process";
static EXP_EXE: &str = "Failed to execute child process";

mod tests;
