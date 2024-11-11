use std::process::{Child, Command, Stdio};

pub fn check_dependencies(names: &[&str]) {
    use which::which;

    let mut error = false;
    for name in names {
        if let Err(_) = which(name) {
            eprintln!("cannot find dependency '{name}'");
            error = true;
        }
    }
    if error { panic!("unsatisfied dependencies") }
}

pub fn create_awk_with_pipe<P: Into<Stdio>>(pipe: P, prefix: &str) -> Child {
    Command::new("awk")
        .stdin(pipe)
        .arg(format!("{{ print \"{prefix}\" $0 }}"))
        .spawn()
        .expect("awk failed!")
}