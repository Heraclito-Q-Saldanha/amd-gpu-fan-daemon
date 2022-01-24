use crate::{push::push, Config, ConfigError};
use std::fs;
use std::path::PathBuf;

pub fn init(path_file: PathBuf) -> Result<(), ConfigError> {
	{
		let mut path = path_file.clone();
		path.pop();
		if !path.exists() {
			if let Err(_) = fs::create_dir_all(&path) {
				return Err(ConfigError::Write{path});
			}
		}
	}
	{
		if !path_file.exists() {
			push(path_file, vec![Config::default()])
		}else{
			Ok(())
		}
	}
}