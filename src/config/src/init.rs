use crate::{push::push, Config, ConfigError};
use std::fs;
use std::path::PathBuf;

pub fn init(path_file: PathBuf) -> Result<(), ConfigError> {
	if let Some(path) = path_file.parent(){
		let path = PathBuf::new().join(path);
		if !path.exists() {
			if fs::create_dir_all(&path).is_err(){
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