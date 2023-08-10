#[cfg(test)]
use crate::*;

#[test]
fn about_string() {
    let _ = about();
}

#[test]
fn new_command() {
    let _cmd = ShellCommand::new("ls");
}
