use std::path::Path;
use std::os::unix::fs;
use std::env::set_current_dir;
use nix::{mount::mount, NixPath};

/// Mounts required filesystems for chrooting, returns a nix::Result<> that indicates if the mount process was successful
pub fn prep_fs(chroot_path: &str) -> nix::Result<()> {    
    if !Path::new(chroot_path).exists() {
        return Err(nix::errno::Errno::ENOENT);
    }

    bind_mnt("/dev", &format!("{chroot_path}/dev"))?;
    bind_mnt("/sys", &format!("{chroot_path}/sys"))?;
    bind_mnt("/proc", &format!("{chroot_path}/proc"))?;

    Ok(())
}

/// chroots into a directory, it is recommended to mount the /dev, /sys and /proc filesystems beforehand
pub fn chroot(new_root: &str) -> std::io::Result<()> {
    fs::chroot(new_root).expect("Unable to chroot");
    set_current_dir("/").expect("Unable to change current directory");
    Ok(())
}

fn bind_mnt<P: ?Sized + NixPath>(source: &P, target: &P) -> nix::Result<()> {
    let flags = nix::mount::MsFlags::MS_BIND | nix::mount::MsFlags::MS_REC;

    mount(Some(source), target, None::<&str>, flags, None::<&str>)
}