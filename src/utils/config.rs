use std::fs::File;
use std::io::Write;

pub fn init_config() -> String {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("envmgr").unwrap();
    let cfg_path = xdg_dirs.place_config_file("config.toml").expect("Error creating configuration file");
    let mut cfg_file = File::create(&cfg_path).unwrap();
    
    write!(&mut cfg_file, "setup = true").expect("Error writing to config file");

    cfg_path.to_str().unwrap().to_string()
}