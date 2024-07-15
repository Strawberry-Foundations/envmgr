use std::process::{Command, Stdio};

mod prep;

fn main() {
    let chroot_dir = "/home/matteo/Junk/chroot";
    let shell = "bash";

    karen::escalate_if_needed().expect("karen fail");

    prep::prep_fs(chroot_dir).expect("prep fail");
    prep::chroot(chroot_dir).expect("chroot fail");

    let mut cmd = Command::new(shell);
    cmd.current_dir("/");
    cmd.stdin(Stdio::inherit())
       .stdout(Stdio::inherit())
       .stderr(Stdio::inherit());

    let mut child = cmd.spawn().expect("spawn fail");

    let status = child.wait().expect("wait fail");

    if !status.success() {
        eprintln!("Shell ended with status: {}", status);
    }
}
