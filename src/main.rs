use clap::{Parser, Subcommand};
use lazy_static::lazy_static;
use std::{process::exit, sync::Mutex};

mod env_mngt;
mod utils;
use crate::utils::config::init_config;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Use certain config, otherwise use config in the XDG directory
    #[arg(short, long, default_value_t = String::new())]
    config: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Enter an environment
    Enter {
        #[arg(short, long, default_value_t = String::new())]
        config: String,
    },
}

lazy_static! {
    static ref CONFIG_PATH: Mutex<String> = Mutex::new(String::new());
}

fn main() {
    let args = Args::parse();

    let config_path = if args.config.is_empty() {
        init_config()
    } else {
        args.config
    };
    
    {
        let mut path = CONFIG_PATH.lock().unwrap();
        *path = config_path;
    }

    println!("Using config file at: {}", CONFIG_PATH.lock().unwrap());

    match &args.command {
        Some(Commands::Enter { config }) => {
            if !config.is_empty() {
                {
                    let mut path = CONFIG_PATH.lock().unwrap();
                    *path = config.clone();
                }
            }
            env_mngt::enter::enter("/home/matteo/Junk/chroot/"); // change later
        }
        None => {
            println!("Nothing to do. Exiting...");
            exit(1)
        }
    }

    println!("Exiting...");
}