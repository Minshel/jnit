use std::process::Command;

pub fn start(command: &str) -> i8 {
    let mut parts = command.split_whitespace();

    let program = match parts.next() {
        Some(program) => program,
        None => return 1,
    };

    let mut process = Command::new(program);
    process.args(parts);

    // process.id(); <- get pid

    match process.status() {
        Ok(status) => {
            if status.success() {
                0
            } else {
                1
            }
        }
        Err(_) => 1,
    }
}
