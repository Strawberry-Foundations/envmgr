use std::process::{Command, Stdio};
use crate::env_mngt::prep::{prep_fs, chroot, clean};

pub fn enter(env_path: &str, user: &str) {
    let shell = "bash"; // change later
    
    println!("Entering environment...");

    karen::escalate_if_needed().expect("Error while trying to get root priviliges");

    prep_fs(env_path, user).expect("Error while prepearing the environment");
    chroot(env_path).expect("Error while entering the environment");

    let mut cmd = Command::new(shell);
    cmd.current_dir("/");
    cmd.stdin(Stdio::inherit())
       .stdout(Stdio::inherit())
       .stderr(Stdio::inherit());

    let mut child = cmd.spawn().expect("Error while spawning shell");

    let status = child.wait().expect("Error while waiting for shell");

    if !status.success() {
        eprintln!("Shell ended with status: {}", status);
    }

    clean(env_path);
}