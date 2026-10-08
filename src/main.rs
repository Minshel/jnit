use std::{collections::HashMap, env, fs::File};
use clap::Parser;

use crate::process::check;

mod process;
mod read;
mod parser;

#[derive(Parser, Debug)]
#[command(name = "jnit")]
struct Arguments {
    #[arg(long, default_value = "config.jnit")]
    config: String,

    #[arg(long, default_value = "start")]
    group: String,
}

fn main() {
    let arguments = Arguments::parse();

    let exe_dir = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let file_name = exe_dir.join(&arguments.config);
    if !file_name.exists() {
        panic!(
            "\x1b[1mJNit: config file '{}' not exists\x1b[0m",
            arguments.config
        );
    }

    let config_file = File::open(file_name).unwrap();

    let processes: Vec<String> =
        read::get_processes(config_file, &arguments.group);
    let mut running_processes: HashMap<String, i32> = HashMap::new();

    for proc in processes {
        let process: HashMap<String, i32> = process::start(&proc);

        println!(
            "JNit: {} started with code {} and pid {}",
            &proc,
            process.get("status").copied().unwrap_or(-1),
            process.get("pid").copied().unwrap_or(-1)
        );

        if process.get("pid").copied().unwrap_or(-1) != -1 {
            running_processes.insert(proc, process.get("pid").copied().unwrap_or(-1));
        }
    }

    check(running_processes);
}
