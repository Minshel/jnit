use std::fs::File;
use clap::Parser;

mod process;
mod read;
mod parser;

#[derive(Parser, Debug)]
#[command(name = "jnit")]
struct Arguments {
    #[arg(long, default_value = "config.jnit")]
    config: String,
}

fn main() {
    let arguments = Arguments::parse();

    let config_file: File = File::open(arguments.config).unwrap();
    let processes: Vec<String> = read::get_processes(config_file);

    for proc in processes {
        let status = process::start(&proc);
        println!("{} started with code {}", proc, status);
    }
}
