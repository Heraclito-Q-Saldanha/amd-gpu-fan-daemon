use std::fs;
use std::path::PathBuf;

pub use crate::{Config, ConfigError};

pub fn push(path_file: PathBuf, config: Vec<Config>) -> Result<(), ConfigError> {
	let string_config = match serde_json::to_string_pretty(&config){
		Ok(string) => string,
		Err(_) => return Err(ConfigError::Deserialize{path: path_file})
	};
	match fs::write(&path_file, string_config){
		Ok(_) => Ok(()),
		Err(_) => Err(ConfigError::Write{path: path_file})
	}
}
