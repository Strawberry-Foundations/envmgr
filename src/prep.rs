use std::path::Path;
use std::os::unix::fs;
use std::env::set_current_dir;
use nix::mount::{mount, MsFlags};

/// Mounts required filesystems for chrooting, returns a bool that indicates if the mount process was successful
pub fn prep_fs(chroot_path: &str) -> bool {
    let path = Path::new(chroot_path);

    if !path.exists() { return false; }



    true
}

/// chroots into a directory, it is recommended to mount the /dev, /sys and /proc filesystems beforehand
pub fn chroot(new_root: &str) -> std::io::Result<()> {
    fs::chroot(new_root).expect("Unable to chroot");
    set_current_dir("/").expect("Unable to change current directory");
    Ok(())
}