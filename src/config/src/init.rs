use crate::{push::push, Config, ConfigError};
use std::fs;
use std::path::PathBuf;

pub fn init(path_file: PathBuf) -> Result<(), ConfigError> {
	if !path_file.exists() {
		if let Err(_) = fs::create_dir_all(&path_file) {
			return Err(ConfigError::Write { path: path_file });
		}
	}
	{
		let mut path_devices = path_file.clone();
		path_devices.push("devices_config.json");
		if !path_devices.exists() {
			push(path_devices, vec![Config::default()])
		}else{
			Ok(())
		}
	}
}