use std::{process::exit, sync::Mutex};
use lazy_static::lazy_static;
use clap::{Parser, Subcommand};
use users::{get_user_by_uid, get_current_uid};

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

    match &args.command {
        Some(Commands::Enter { .. }) => {
            let user = get_user_by_uid(get_current_uid()).unwrap();
            
            env_mngt::enter::enter("/home/matteo/Junk/chroot/", user.name().to_str().unwrap()); // change later
        }
        None => {
            println!("Nothing to do. Exiting...");
            exit(1)
        }
    }

    println!("Exiting...");
}