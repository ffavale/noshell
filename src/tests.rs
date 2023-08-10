#[cfg(test)]
use crate::*;

#[test]
fn about_string() {
    let _ = about();
}

#[test]
fn new_command() {
    let cmd = ShellCommand::new("ls");
    let _output = cmd.run().unwrap();
    // println!("{}", _output);
}

#[test]
fn args_addition() {
    let cmd = ShellCommand::new("ls").args(vec!["-l", "-a", "-h"]);
    let _output = cmd.run().unwrap();
    // println!("{}", _output);
}

#[test]
fn envvar_addition() {
    let cmd =
        ShellCommand::new("printenv").envs(vec![("TESTVAR", "value1"), ("ANOTHERVAR", "value2")]);
    let _output = cmd.run().unwrap();
    // println!("{}", _output);
}

#[test]
fn pipe_string_into_cmd() {
    let cmd = ShellCommand::new("grep")
        .args(["a."])
        .pipe_string("asdfqweriopmnbvcxz\ndfsdfiwuehfhgfbiucshd\nsdfsdfhhurrr");
    let _output = cmd.run().unwrap();
    // println!("{}", _output);
}

#[test]
fn pipe_cmd_into_cmd() {
    let cmd =
        ShellCommand::new("printenv").envs(vec![("TESTVAR", "value1"), ("ANOTHERVAR", "value2")]);
    let grep = ShellCommand::new("grep").args(["VAR="]).pipe_stdout(cmd);
    let _output = grep.run().unwrap();
    // println!("{}", _output);
}
