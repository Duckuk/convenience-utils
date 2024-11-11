use std::process::{Child, Command, Stdio};

pub fn create_awk_with_pipe<P: Into<Stdio>>(pipe: P, prefix: &str) -> Child {
    Command::new("awk")
        .stdin(pipe)
        .arg(format!("{{ print \"{prefix}\" $0 }}"))
        .spawn()
        .expect("awk failed!")
}