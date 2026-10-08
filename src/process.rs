use std::{collections::HashMap, process::Command};
use sysinfo::{Pid, System};
use std::{thread, time::Duration};

pub fn start(command: &str) -> HashMap<String, i32> {
    let mut parts = command.split_whitespace();

    let program = match parts.next() {
        Some(program) => program,
        None => return HashMap::new(),
    };

    let mut process = Command::new(program);
    process.args(parts);

    let mut child = match process.spawn() {
        Ok(child) => child,
        Err(_) => {
            let mut map = HashMap::new();
            map.insert("status".to_string(), -1);
            return map;
        }
    };

    let pid = child.id();

    let status_code = match child.wait() {
        Ok(status) if status.success() => 0,
        Ok(_) => 1,
        Err(_) => -1,
    };

    let mut process_map: HashMap<String, i32> = HashMap::new();

    process_map.insert("status".to_string(), status_code);
    process_map.insert("pid".to_string(), pid as i32);

    process_map
}

pub fn check(mut processes: HashMap<String, i32>) {
    let mut system = System::new_all();

    loop {
        thread::sleep(Duration::from_millis(500));

        system.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

        for (command, pid) in processes.iter_mut() {
            if *pid < 0 {
                continue;
            }

            if system.process(Pid::from_u32(*pid as u32)).is_none() {
                println!("JNit: {} (pid {}) died, restarting...", command, pid);

                let restarted = crate::process::start(command);

                *pid = restarted.get("pid").copied().unwrap_or(-1);
            }
        }
    }
}
